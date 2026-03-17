/*
 * Copyright (C) 2026 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "pane_layout_controller.h"

#include <QJsonArray>
#include <QUuid>

// === PaneViewState ===

PaneViewState::PaneViewState(PaneViewContentType type, int contentId, const QJsonObject &state, QObject *parent)
    : QObject(parent), m_type(type), m_contentId(contentId), m_state(state)
{
}

void PaneViewState::setState(const QJsonObject &state)
{
    m_state = state;
}

// === PaneState ===

PaneState::PaneState(const QString &id, QObject *parent) : QObject(parent), m_paneId(id)
{
}

void PaneState::setWidthRatio(double ratio)
{
    if (!qFuzzyCompare(m_widthRatio, ratio))
    {
        m_widthRatio = ratio;
        Q_EMIT widthRatioChanged();
    }
}

void PaneState::setIsLocked(bool locked)
{
    if (m_isLocked != locked)
    {
        m_isLocked = locked;
        Q_EMIT isLockedChanged();
    }
}

bool PaneState::havePrevious() const
{
    return m_historyIndex > 0;
}

bool PaneState::haveNext() const
{
    return m_historyIndex < m_history.size() - 1;
}

void PaneState::navigateToPrevious()
{
    if (m_historyIndex > 0)
    {
        m_historyIndex--;
        Q_EMIT currentPaneViewStateChanged();
        Q_EMIT havePreviousChanged();
        Q_EMIT haveNextChanged();
    }
}

void PaneState::navigateToNext()
{
    if (m_historyIndex < m_history.size() - 1)
    {
        m_historyIndex++;
        Q_EMIT currentPaneViewStateChanged();
        Q_EMIT havePreviousChanged();
        Q_EMIT haveNextChanged();
    }
}

void PaneState::pushView(PaneViewState *viewState)
{
    viewState->setParent(this);

    // Truncate forward history
    while (m_history.size() - 1 > m_historyIndex && !m_history.isEmpty())
    {
        delete m_history.takeLast();
    }

    // Enforce history cap
    if (m_history.size() >= MaxHistory)
    {
        delete m_history.takeFirst();
    }

    m_history.append(viewState);
    m_historyIndex = m_history.size() - 1;

    Q_EMIT currentPaneViewStateChanged();
    Q_EMIT havePreviousChanged();
    Q_EMIT haveNextChanged();
}

PaneViewState *PaneState::currentPaneViewState() const
{
    if (m_historyIndex < 0 || m_historyIndex >= m_history.size())
        return nullptr;
    return m_history.at(m_historyIndex);
}

// === PaneLayoutController ===

PaneLayoutController::PaneLayoutController(QObject *parent)
    : QObject(parent), m_activePaneIndex(0), m_treeLockedLeft(false), m_minPaneWidth(500), m_availableWidth(1920)
{
}

void PaneLayoutController::initialize()
{
    if (m_panes.isEmpty())
    {
        auto *pane = createPane(PaneViewState::Tree);
        m_panes.append(pane);
        Q_EMIT panesChanged();
    }
}

void PaneLayoutController::setActivePaneIndex(int index)
{
    if (index >= 0 && index < m_panes.size() && m_activePaneIndex != index)
    {
        m_activePaneIndex = index;
        Q_EMIT activePaneIndexChanged();
    }
}

void PaneLayoutController::setTreeLockedLeft(bool locked)
{
    if (m_treeLockedLeft != locked)
    {
        m_treeLockedLeft = locked;

        if (!m_panes.isEmpty())
        {
            m_panes[0]->setIsLocked(locked);
            if (locked)
            {
                auto current = m_panes[0]->currentPaneViewState();
                if (current && current->paneViewContentType() != PaneViewState::Tree)
                {
                    m_panes[0]->pushView(new PaneViewState(PaneViewState::Tree));
                }
            }
        }

        Q_EMIT treeLockedLeftChanged();
    }
}

void PaneLayoutController::setMinPaneWidth(int width)
{
    if (m_minPaneWidth != width)
    {
        m_minPaneWidth = width;
        Q_EMIT minPaneWidthChanged();
        Q_EMIT maxPanesChanged();
        Q_EMIT canSplitMoreChanged();
    }
}

int PaneLayoutController::maxPanes() const
{
    if (m_availableWidth <= 0 || m_minPaneWidth <= 0)
        return 1;
    return qMax(1, m_availableWidth / m_minPaneWidth);
}

bool PaneLayoutController::canSplitMore() const
{
    return m_panes.size() < maxPanes();
}

void PaneLayoutController::openInPane(int paneIndex, PaneViewState::PaneViewContentType type, int contentId)
{
    int targetIndex = paneIndex;

    // Locked tree invariant: position 0 is always tree when locked
    if (m_treeLockedLeft && paneIndex == 0 && m_panes[0]->isLocked() && type != PaneViewState::Tree)
    {
        targetIndex = getTargetPaneForLockedTree(paneIndex);
    }

    if (targetIndex >= 0 && targetIndex < m_panes.size())
    {
        m_panes[targetIndex]->pushView(new PaneViewState(type, contentId));
        setActivePaneIndex(targetIndex);
    }
}

void PaneLayoutController::splitPane(int paneIndex, SplitDirection direction, PaneViewState::PaneViewContentType type,
                                      int contentId)
{
    if (!canSplitMore())
        return;

    int insertIndex = (direction == Right) ? paneIndex + 1 : paneIndex;

    PaneViewState::PaneViewContentType typeToUse = type;
    int contentIdToUse = contentId;
    QJsonObject stateToUse;

    // If no type specified, duplicate the source pane's current view
    if (type == PaneViewState::Empty && contentId == -1)
    {
        auto current = m_panes[paneIndex]->currentPaneViewState();
        if (current)
        {
            typeToUse = current->paneViewContentType();
            contentIdToUse = current->contentId();
            stateToUse = current->state();
        }
    }

    auto *newPane = createPane(typeToUse, contentIdToUse, stateToUse);

    insertPane(insertIndex, newPane);
    normalizeWidthRatios();
    setActivePaneIndex(insertIndex);

    Q_EMIT paneAdded(insertIndex);
    Q_EMIT layoutChanged();
}

void PaneLayoutController::closePane(int paneIndex)
{
    if (m_panes.size() <= 1)
    {
        // Last pane: reset to tree
        openInPane(0, PaneViewState::Tree);
        return;
    }

    // Cannot close locked tree pane
    if (paneIndex == 0 && m_treeLockedLeft && m_panes[0]->isLocked())
        return;

    // Proportional space redistribution to neighbors
    double freedRatio = m_panes[paneIndex]->widthRatio();
    removePane(paneIndex);

    if (m_panes.size() == 1)
    {
        m_panes[0]->setWidthRatio(1.0);
    }
    else if (paneIndex == 0)
    {
        // Closed leftmost: right neighbor absorbs all
        m_panes[0]->setWidthRatio(m_panes[0]->widthRatio() + freedRatio);
    }
    else if (paneIndex >= m_panes.size())
    {
        // Closed rightmost: left neighbor absorbs all
        m_panes.last()->setWidthRatio(m_panes.last()->widthRatio() + freedRatio);
    }
    else
    {
        // Middle pane: split freed space between neighbors
        m_panes[paneIndex - 1]->setWidthRatio(m_panes[paneIndex - 1]->widthRatio() + freedRatio / 2.0);
        m_panes[paneIndex]->setWidthRatio(m_panes[paneIndex]->widthRatio() + freedRatio / 2.0);
    }

    if (m_activePaneIndex >= m_panes.size())
        setActivePaneIndex(m_panes.size() - 1);

    Q_EMIT paneRemoved(paneIndex);
    Q_EMIT layoutChanged();
}

void PaneLayoutController::swapPanes(int index1, int index2)
{
    if (index1 >= 0 && index1 < m_panes.size() && index2 >= 0 && index2 < m_panes.size() && index1 != index2)
    {
        if ((index1 == 0 || index2 == 0) && m_treeLockedLeft)
            return;

        m_panes.swapItemsAt(index1, index2);
        Q_EMIT panesChanged();
        Q_EMIT layoutChanged();
    }
}

void PaneLayoutController::focusPane(int paneIndex, FocusReason reason)
{
    Q_UNUSED(reason)
    setActivePaneIndex(paneIndex);
}

void PaneLayoutController::updateAvailableWidth(int totalWidth)
{
    if (m_availableWidth != totalWidth)
    {
        m_availableWidth = totalWidth;
        Q_EMIT maxPanesChanged();
        Q_EMIT canSplitMoreChanged();
        recalculateForWidth();
    }
}

void PaneLayoutController::redistributeWidths()
{
    normalizeWidthRatios();
}

void PaneLayoutController::navigateToTreeInActivePane()
{
    if (m_activePaneIndex >= 0 && m_activePaneIndex < m_panes.size())
    {
        if (m_treeLockedLeft && !m_panes.isEmpty() && m_panes[0]->isLocked())
        {
            setActivePaneIndex(0);
        }
        else
        {
            openInPane(m_activePaneIndex, PaneViewState::Tree);
        }
    }
}

void PaneLayoutController::openInNewSplit(PaneViewState::PaneViewContentType type, int contentId)
{
    if (!canSplitMore())
    {
        openInCurrentPane(type, contentId);
        return;
    }

    splitPane(m_panes.size() - 1, Right, type, contentId);
}

void PaneLayoutController::openInCurrentPane(PaneViewState::PaneViewContentType type, int contentId)
{
    openInPane(m_activePaneIndex, type, contentId);
}

// --- Drag-and-drop ---

PaneLayoutController::DropZone PaneLayoutController::calculateDropZone(int paneIndex, double mouseX, double paneWidth)
{
    if (paneIndex < 0 || paneIndex >= m_panes.size())
        return DropZone::None;

    auto *pane = m_panes[paneIndex];

    // Locked tree always forwards
    if (pane->isLocked() && m_treeLockedLeft)
        return DropZone::LockedForward;

    int edgeWidth = adaptiveDropZoneWidth(paneWidth);

    if (mouseX < edgeWidth)
        return canSplitMore() ? DropZone::LeftEdge : DropZone::None;

    if (mouseX > paneWidth - edgeWidth)
        return canSplitMore() ? DropZone::RightEdge : DropZone::None;

    return DropZone::Center;
}

void PaneLayoutController::handleDrop(int paneIndex, DropZone zone, PaneViewState::PaneViewContentType type,
                                       int contentId)
{
    switch (zone)
    {
    case DropZone::Center:
        openInPane(paneIndex, type, contentId);
        break;
    case DropZone::LeftEdge:
        splitPane(paneIndex, Left, type, contentId);
        break;
    case DropZone::RightEdge:
        splitPane(paneIndex, Right, type, contentId);
        break;
    case DropZone::LockedForward: {
        int targetIndex = getTargetPaneForLockedTree(paneIndex);
        openInPane(targetIndex, type, contentId);
        break;
    }
    case DropZone::None:
    default:
        break;
    }
}

// --- Overflow ---

void PaneLayoutController::restoreOverflowPane()
{
    if (m_overflowPanes.isEmpty() || !canSplitMore())
        return;

    auto *pane = m_overflowPanes.takeFirst();
    m_panes.append(pane);
    normalizeWidthRatios();

    Q_EMIT panesChanged();
    Q_EMIT overflowCountChanged();
    Q_EMIT canSplitMoreChanged();
    Q_EMIT layoutChanged();
}

// --- Persistence ---

QJsonObject PaneLayoutController::serializeLayout() const
{
    QJsonObject layout;
    layout["version"] = 1;
    layout["treeLockedLeft"] = m_treeLockedLeft;
    layout["activePaneIndex"] = m_activePaneIndex;

    QJsonArray panesArray;
    auto serializePaneList = [](const QList<PaneState *> &list) {
        QJsonArray arr;
        for (const auto *pane : list)
        {
            QJsonObject paneObj;
            paneObj["id"] = pane->paneId();
            paneObj["widthRatio"] = pane->widthRatio();
            paneObj["isLocked"] = pane->isLocked();

            auto current = pane->currentPaneViewState();
            if (current)
            {
                paneObj["contentType"] = static_cast<int>(current->paneViewContentType());
                paneObj["contentId"] = current->contentId();
                paneObj["viewState"] = current->state();
            }
            arr.append(paneObj);
        }
        return arr;
    };

    layout["panes"] = serializePaneList(m_panes);
    layout["overflowPanes"] = serializePaneList(m_overflowPanes);

    return layout;
}

void PaneLayoutController::restoreLayout(const QJsonObject &layout)
{
    qDeleteAll(m_panes);
    m_panes.clear();
    qDeleteAll(m_overflowPanes);
    m_overflowPanes.clear();

    auto restorePaneList = [this](const QJsonArray &arr) {
        QList<PaneState *> list;
        for (const auto &val : arr)
        {
            QJsonObject obj = val.toObject();
            auto *pane = new PaneState(obj["id"].toString(), this);
            pane->setWidthRatio(obj["widthRatio"].toDouble(0.5));
            pane->setIsLocked(obj["isLocked"].toBool());

            auto type = static_cast<PaneViewState::PaneViewContentType>(obj["contentType"].toInt());
            int contentId = obj["contentId"].toInt(-1);
            QJsonObject viewState = obj["viewState"].toObject();
            pane->pushView(new PaneViewState(type, contentId, viewState));

            list.append(pane);
        }
        return list;
    };

    m_panes = restorePaneList(layout["panes"].toArray());
    m_overflowPanes = restorePaneList(layout["overflowPanes"].toArray());

    setTreeLockedLeft(layout["treeLockedLeft"].toBool());
    setActivePaneIndex(layout["activePaneIndex"].toInt());
    ensureAtLeastOnePane();

    Q_EMIT panesChanged();
    Q_EMIT overflowCountChanged();
    Q_EMIT layoutChanged();
}

// === Private helpers ===

PaneState *PaneLayoutController::createPane(PaneViewState::PaneViewContentType type, int contentId,
                                             const QJsonObject &state)
{
    auto *pane = new PaneState(generatePaneId(), this);
    pane->pushView(new PaneViewState(type, contentId, state));
    pane->setWidthRatio(1.0 / qMax(1, m_panes.size() + 1));
    return pane;
}

void PaneLayoutController::insertPane(int index, PaneState *pane)
{
    m_panes.insert(index, pane);
    Q_EMIT panesChanged();
    Q_EMIT canSplitMoreChanged();
}

void PaneLayoutController::removePane(int index)
{
    if (index >= 0 && index < m_panes.size())
    {
        auto *pane = m_panes.takeAt(index);
        pane->deleteLater();
        Q_EMIT panesChanged();
        Q_EMIT canSplitMoreChanged();
    }
}

void PaneLayoutController::normalizeWidthRatios()
{
    if (m_panes.isEmpty())
        return;

    double equalRatio = 1.0 / m_panes.size();
    for (auto *pane : m_panes)
        pane->setWidthRatio(equalRatio);
}

void PaneLayoutController::recalculateForWidth()
{
    int max = maxPanes();

    bool changed = false;

    // Move excess visible panes to overflow
    while (m_panes.size() > max && m_panes.size() > 1)
    {
        auto *pane = m_panes.takeLast();
        m_overflowPanes.prepend(pane);
        changed = true;
    }

    // Restore from overflow when space permits
    while (!m_overflowPanes.isEmpty() && m_panes.size() < max)
    {
        auto *pane = m_overflowPanes.takeFirst();
        m_panes.append(pane);
        changed = true;
    }

    if (changed)
    {
        normalizeWidthRatios();

        if (m_activePaneIndex >= m_panes.size())
            setActivePaneIndex(qMax(0, m_panes.size() - 1));

        Q_EMIT panesChanged();
        Q_EMIT overflowCountChanged();
        Q_EMIT canSplitMoreChanged();
        Q_EMIT layoutChanged();
    }
}

int PaneLayoutController::getTargetPaneForLockedTree(int attemptedIndex)
{
    if (attemptedIndex == 0 && m_treeLockedLeft)
    {
        if (m_panes.size() > 1)
            return 1;

        if (canSplitMore())
        {
            auto *newPane = createPane(PaneViewState::Empty);
            insertPane(1, newPane);
            normalizeWidthRatios();
            return 1;
        }
    }
    return attemptedIndex;
}

QString PaneLayoutController::generatePaneId()
{
    return QUuid::createUuid().toString(QUuid::WithoutBraces);
}

void PaneLayoutController::ensureAtLeastOnePane()
{
    if (m_panes.isEmpty())
    {
        auto *pane = createPane(PaneViewState::Tree);
        m_panes.append(pane);
        Q_EMIT panesChanged();
    }
}

int PaneLayoutController::adaptiveDropZoneWidth(double paneWidth) const
{
    constexpr int minZone = 16;
    constexpr int maxZone = 32;
    return qMax(minZone, qMin(static_cast<int>(paneWidth * 0.05), maxZone));
}

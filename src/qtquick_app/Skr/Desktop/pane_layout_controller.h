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

#pragma once

#include <QJsonObject>
#include <QObject>
#include <QPointer>
#include <QString>
#include <qqmlintegration.h>

// ---------------------------------------------------------
// PaneViewState: one entry in a pane's navigation history.
// Captures what content a pane is showing and its view state
// (scroll position, expanded nodes, selection, etc.).
// ---------------------------------------------------------

class PaneViewState : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("PaneViewState is created by PaneLayoutController")

  public:
    enum PaneViewContentType
    {
        Empty,
        CascadingList,
        Tree,
        Overview,
        TextContent
    };
    Q_ENUM(PaneViewContentType)

    explicit PaneViewState(PaneViewContentType type, int contentId = -1, const QJsonObject &state = {},
                           QObject *parent = nullptr);

    Q_INVOKABLE PaneViewContentType paneViewContentType() const
    {
        return m_type;
    }
    Q_INVOKABLE int contentId() const
    {
        return m_contentId;
    }
    Q_INVOKABLE QJsonObject state() const
    {
        return m_state;
    }
    Q_INVOKABLE void setState(const QJsonObject &state);

  private:
    PaneViewContentType m_type;
    int m_contentId;
    QJsonObject m_state;
};

// ---------------------------------------------------------
// PaneState: one pane with per-pane navigation history,
// width ratio, and optional lock.
// ---------------------------------------------------------

class PaneState : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_UNCREATABLE("PaneState is created by PaneLayoutController")

    Q_PROPERTY(PaneViewState *currentPaneViewState READ currentPaneViewState NOTIFY currentPaneViewStateChanged)
    Q_PROPERTY(bool havePrevious READ havePrevious NOTIFY havePreviousChanged)
    Q_PROPERTY(bool haveNext READ haveNext NOTIFY haveNextChanged)
    Q_PROPERTY(QString title MEMBER m_title NOTIFY titleChanged)
    Q_PROPERTY(double widthRatio READ widthRatio WRITE setWidthRatio NOTIFY widthRatioChanged)
    Q_PROPERTY(bool isLocked READ isLocked WRITE setIsLocked NOTIFY isLockedChanged)

  public:
    explicit PaneState(const QString &id, QObject *parent = nullptr);

    QString paneId() const
    {
        return m_paneId;
    }

    double widthRatio() const
    {
        return m_widthRatio;
    }
    void setWidthRatio(double ratio);

    bool isLocked() const
    {
        return m_isLocked;
    }
    void setIsLocked(bool locked);

    bool havePrevious() const;
    bool haveNext() const;

    Q_INVOKABLE void navigateToPrevious();
    Q_INVOKABLE void navigateToNext();

    void pushView(PaneViewState *viewState);
    PaneViewState *currentPaneViewState() const;

  Q_SIGNALS:
    void currentPaneViewStateChanged();
    void widthRatioChanged();
    void isLockedChanged();
    void havePreviousChanged();
    void haveNextChanged();
    void titleChanged();

  private:
    static constexpr int MaxHistory = 10;

    QString m_paneId;
    int m_historyIndex = -1;
    QList<PaneViewState *> m_history; // owned children
    QString m_title;
    double m_widthRatio = 0.5;
    bool m_isLocked = false;
};

// ---------------------------------------------------------
// PaneLayoutController: QML singleton managing the pane
// layout, splits, closing, drag-drop, overflow, and focus.
// ---------------------------------------------------------

class PaneLayoutController : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON

    Q_PROPERTY(QList<PaneState *> panes READ panes NOTIFY panesChanged)
    Q_PROPERTY(int activePaneIndex READ activePaneIndex WRITE setActivePaneIndex NOTIFY activePaneIndexChanged)
    Q_PROPERTY(bool treeLockedLeft READ treeLockedLeft WRITE setTreeLockedLeft NOTIFY treeLockedLeftChanged)
    Q_PROPERTY(int minPaneWidth READ minPaneWidth WRITE setMinPaneWidth NOTIFY minPaneWidthChanged)
    Q_PROPERTY(int maxPanes READ maxPanes NOTIFY maxPanesChanged)
    Q_PROPERTY(bool canSplitMore READ canSplitMore NOTIFY canSplitMoreChanged)
    Q_PROPERTY(int overflowCount READ overflowCount NOTIFY overflowCountChanged)

  public:
    enum SplitDirection
    {
        Left,
        Right
    };
    Q_ENUM(SplitDirection)

    enum DropZone
    {
        None,
        Center,
        LeftEdge,
        RightEdge,
        LockedForward
    };
    Q_ENUM(DropZone)

    enum FocusReason
    {
        Click,
        KeyboardNav,
        ContentOpened,
        ItemDropped
    };
    Q_ENUM(FocusReason)

    explicit PaneLayoutController(QObject *parent = nullptr);

    // Properties
    QList<PaneState *> panes() const
    {
        return m_panes;
    }
    int activePaneIndex() const
    {
        return m_activePaneIndex;
    }
    void setActivePaneIndex(int index);

    bool treeLockedLeft() const
    {
        return m_treeLockedLeft;
    }
    void setTreeLockedLeft(bool locked);

    int minPaneWidth() const
    {
        return m_minPaneWidth;
    }
    void setMinPaneWidth(int width);

    int maxPanes() const;
    bool canSplitMore() const;
    int overflowCount() const
    {
        return m_overflowPanes.size();
    }

    // Core operations
    Q_INVOKABLE void initialize();
    Q_INVOKABLE void openInPane(int paneIndex, PaneViewState::PaneViewContentType type, int contentId = -1);
    Q_INVOKABLE void splitPane(int paneIndex, SplitDirection direction,
                                PaneViewState::PaneViewContentType type = PaneViewState::Empty, int contentId = -1);
    Q_INVOKABLE void closePane(int paneIndex);
    Q_INVOKABLE void swapPanes(int index1, int index2);
    Q_INVOKABLE void focusPane(int paneIndex, FocusReason reason = Click);

    // Layout
    Q_INVOKABLE void updateAvailableWidth(int totalWidth);
    Q_INVOKABLE void redistributeWidths();

    // Navigation
    Q_INVOKABLE void navigateToTreeInActivePane();
    Q_INVOKABLE void openInNewSplit(PaneViewState::PaneViewContentType type, int contentId = -1);
    Q_INVOKABLE void openInCurrentPane(PaneViewState::PaneViewContentType type, int contentId = -1);

    // Drag-and-drop
    Q_INVOKABLE DropZone calculateDropZone(int paneIndex, double mouseX, double paneWidth);
    Q_INVOKABLE void handleDrop(int paneIndex, DropZone zone, PaneViewState::PaneViewContentType type, int contentId);

    // Overflow
    Q_INVOKABLE void restoreOverflowPane();

    // Persistence
    Q_INVOKABLE QJsonObject serializeLayout() const;
    Q_INVOKABLE void restoreLayout(const QJsonObject &layout);

  Q_SIGNALS:
    void panesChanged();
    void activePaneIndexChanged();
    void treeLockedLeftChanged();
    void minPaneWidthChanged();
    void maxPanesChanged();
    void canSplitMoreChanged();
    void overflowCountChanged();

    void paneAdded(int index);
    void paneRemoved(int index);
    void layoutChanged();

  private:
    QList<PaneState *> m_panes;
    QList<PaneState *> m_overflowPanes;
    int m_activePaneIndex = 0;
    bool m_treeLockedLeft = false;
    int m_minPaneWidth = 500;
    int m_availableWidth = 1920;

    PaneState *createPane(PaneViewState::PaneViewContentType type, int contentId = -1,
                          const QJsonObject &state = {});
    void insertPane(int index, PaneState *pane);
    void removePane(int index);
    void normalizeWidthRatios();
    void recalculateForWidth();
    int getTargetPaneForLockedTree(int attemptedIndex);
    QString generatePaneId();
    void ensureAtLeastOnePane();
    int adaptiveDropZoneWidth(double paneWidth) const;
};

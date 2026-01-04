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

#include "recent_work_list_model_from_root_recent_works.h"
#include "../recent_work/dtos.h"
#include "../recent_work/recent_work_controller.h"
#include "direct_access/event_registry.h"
#include "direct_access/recent_work/recent_work_events.h"
#include "direct_access/root/root_events.h"
#include "dtos.h"
#include "root_controller.h"
#include "service_locator.h"
#include <QCoro/QCoroTask>
#include <QTimer>

namespace Skribisto::DirectAccess::Root
{

RecentWorkListModelFromRootRecentWorks::RecentWorkListModelFromRootRecentWorks(QObject *parent)
    : QAbstractListModel(parent)
{
    resolveDependencies();
}

void RecentWorkListModelFromRootRecentWorks::resolveDependencies()
{
    auto *locator = Common::ServiceLocator::instance();
    if (!locator)
    {
        qCritical() << "ServiceLocator not initialized";
        return;
    }

    m_eventRegistry = locator->eventRegistry();

    // Create controllers
    m_recentWorkController = new RecentWork::RecentWorkController(this);
    m_rootController = new RootController(this);

    // Connect to events
    if (m_eventRegistry)
    {
        const auto recentWorkEvents = m_eventRegistry->getEvents<Common::DirectAccess::RecentWork::RecentWorkEvents>();
        if (recentWorkEvents)
        {
            connect(recentWorkEvents.data(), SIGNAL(updated(QList<int>)), this,
                    SLOT(onRecentWorkEventsUpdated(QList<int>)));
        }

        const auto rootEvents = m_eventRegistry->getEvents<Common::DirectAccess::Root::RootEvents>();
        if (rootEvents)
        {
            connect(rootEvents.data(), SIGNAL(updated(QList<int>)), this, SLOT(onRootEventsUpdated(QList<int>)));
        }
    }
}

int RecentWorkListModelFromRootRecentWorks::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
        return 0;

    return m_recentWorks.size();
}

QVariant RecentWorkListModelFromRootRecentWorks::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_recentWorks.size())
        return {};

    const auto &recentWork = m_recentWorks.at(index.row());

    switch (role)
    {
    case IdRole:
        return recentWork.id;
    case CreatedAtRole:
        return recentWork.createdAt;
    case UpdatedAtRole:
        return recentWork.updatedAt;
    case TitleRole:
        return recentWork.title;
    case LastOpenedAtRole:
        return recentWork.lastOpenedAt;
    case AbsolutePathRole:
        return recentWork.absolutePath;
    default:;
    }

    return {};
}

bool RecentWorkListModelFromRootRecentWorks::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid() || index.row() >= m_recentWorks.size())
        return false;

    auto &recentWork = m_recentWorks[index.row()];
    bool changed = false;

    switch (role)
    {
    case TitleRole:
        if (recentWork.title != value.toString())
        {
            recentWork.title = value.toString();
            changed = true;
        }
        break;
    case LastOpenedAtRole:
        if (recentWork.lastOpenedAt != value.toDateTime())
        {
            recentWork.lastOpenedAt = value.toDateTime();
            changed = true;
        }
        break;
    case AbsolutePathRole:
        if (recentWork.absolutePath != value.toString())
        {
            recentWork.absolutePath = value.toString();
            changed = true;
        }
        break;
    default:;
    }

    if (changed)
    {
        // Update via controller
        if (m_recentWorkController)
        {
            auto dto = RecentWork::RecentWorkDto(recentWork);
            QCoro::Task<QList<RecentWork::RecentWorkDto>> updateTask = m_recentWorkController->update({dto});

            QCoro::connect(std::move(updateTask), this, [this, index, role](auto &&result) {
                if (!result.isEmpty())
                {
                    // Update local items with returned data (in case of any changes from backend)
                    m_recentWorks[index.row()] = result.first();
                    Q_EMIT dataChanged(index, index, {role});
                }
            });
        }
        else
        {
            qWarning() << "RecentWorkController not available for update";
        }
    }
    return changed;
}

Qt::ItemFlags RecentWorkListModelFromRootRecentWorks::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEnabled | Qt::ItemIsSelectable | Qt::ItemIsEditable;
}

QHash<int, QByteArray> RecentWorkListModelFromRootRecentWorks::roleNames() const
{
    QHash<int, QByteArray> names;
    names[IdRole] = "itemId";
    names[CreatedAtRole] = "createdAt";
    names[UpdatedAtRole] = "updatedAt";
    names[TitleRole] = "title";
    names[LastOpenedAtRole] = "lastOpenedAt";
    names[AbsolutePathRole] = "absolutePath";
    return names;
}

int RecentWorkListModelFromRootRecentWorks::rootId() const
{
    return m_rootId;
}

void RecentWorkListModelFromRootRecentWorks::setRootId(int rootId)
{
    if (m_rootId != rootId)
    {
        m_rootId = rootId;
        refreshData();
        Q_EMIT rootIdChanged();
    }
}

void RecentWorkListModelFromRootRecentWorks::refreshData()
{
    if (!m_rootController || !m_recentWorkController || m_rootId <= 0)
        return;

    QCoro::Task<QList<RecentWork::RecentWorkDto>> fetchRecentWorksTask =
        m_rootController->getRelationshipIds(m_rootId, RootRelationshipField::RecentWorks)
            .then([this](auto &&recentWorkIds) -> QCoro::Task<QList<RecentWork::RecentWorkDto>> {
                return m_recentWorkController->get(recentWorkIds);
            });

    QCoro::connect(std::move(fetchRecentWorksTask), this, [this](auto &&result) {
        if (!m_rootController || !m_recentWorkController)
            return;

        beginResetModel();
        m_recentWorks = result;
        endResetModel();

        qDebug() << "Refresh requested for root ID:" << m_rootId;
    });
}

void RecentWorkListModelFromRootRecentWorks::onRecentWorkEventsUpdated(const QList<int> &ids)
{
    QList<int> relevantIds;
    for (const auto &item : m_recentWorks)
    {
        if (ids.contains(item.id))
        {
            relevantIds.append(item.id);
        }
    }

    if (relevantIds.isEmpty())
        return;

    QCoro::Task<QList<RecentWork::RecentWorkDto>> fetchRecentWorksTask = m_recentWorkController->get(relevantIds);

    QCoro::connect(std::move(fetchRecentWorksTask), this, [this](auto &&result) {
        if (!m_rootController || !m_recentWorkController)
            return;

        // Update existing items
        for (const RecentWork::RecentWorkDto &dto : result)
        {
            for (int i = 0; i < m_recentWorks.size(); ++i)
            {
                if (m_recentWorks[i].id == dto.id)
                {
                    m_recentWorks[i] = dto;
                    const QModelIndex idx = index(i);
                    Q_EMIT dataChanged(idx, idx);
                    break;
                }
            }
        }
    });
}

void RecentWorkListModelFromRootRecentWorks::onRootEventsUpdated(const QList<int> &ids)
{
    // Check if our root was updated
    if (ids.contains(m_rootId))
    {
        QCoro::Task<QList<RootDto>> fetchRootTask = m_rootController->get({m_rootId});

        QCoro::connect(std::move(fetchRootTask), this, [this](auto &&result) {
            if (!m_rootController || !m_recentWorkController)
                return;

            if (result.isEmpty())
                return;

            const auto &root = result.first();

            QList<int> currentItemIds;
            for (const auto &item : m_recentWorks)
            {
                currentItemIds.append(item.id);
            }

            // If recent works changed, refresh data
            if (root.recentWorks != currentItemIds)
            {
                refreshData();
            }
        });
    }
}

} // namespace Skribisto::DirectAccess::Root

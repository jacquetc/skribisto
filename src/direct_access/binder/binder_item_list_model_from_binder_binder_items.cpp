/*
 * Copyright (C) 2025 by Cyril Jacquet
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

#include "binder_item_list_model_from_binder_binder_items.h"
#include "../binder_item/binder_item_controller.h"
#include "../binder_item/dtos.h"
#include "binder_controller.h"
#include "direct_access/binder/binder_events.h"
#include "direct_access/binder_item/binder_item_events.h"
#include "direct_access/event_registry.h"
#include "dtos.h"
#include "service_locator.h"
#include <QCoro/QCoroTask>
#include <QTimer>

namespace Skribisto::DirectAccess::Binder
{

BinderItemListModelFromBinderBinderItems::BinderItemListModelFromBinderBinderItems(QObject *parent)
    : QAbstractListModel(parent)
{
    resolveDependencies();
}

void BinderItemListModelFromBinderBinderItems::resolveDependencies()
{
    auto *locator = Common::ServiceLocator::instance();
    if (!locator)
    {
        qCritical() << "ServiceLocator not initialized";
        return;
    }

    m_eventRegistry = locator->eventRegistry();

    // Create controllers
    m_binderItemController = new BinderItem::BinderItemController(this);
    m_binderController = new BinderController(this);

    // Connect to events
    if (m_eventRegistry)
    {
        const auto binderItemEvents = m_eventRegistry->getEvents<Common::DirectAccess::BinderItem::BinderItemEvents>();
        if (binderItemEvents)
        {
            connect(binderItemEvents.data(), SIGNAL(updated(QList<int>)), this,
                    SLOT(onBinderItemEventsUpdated(QList<int>)));
        }

        const auto binderEvents = m_eventRegistry->getEvents<Common::DirectAccess::Binder::BinderEvents>();
        if (binderEvents)
        {
            connect(binderEvents.data(), SIGNAL(updated(QList<int>)), this, SLOT(onBinderEventsUpdated(QList<int>)));
        }
    }
}

int BinderItemListModelFromBinderBinderItems::rowCount(const QModelIndex &parent) const
{
    if (parent.isValid())
        return 0;

    return m_binderItems.size();
}

QVariant BinderItemListModelFromBinderBinderItems::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= m_binderItems.size())
        return {};

    const auto &binderItem = m_binderItems.at(index.row());

    switch (role)
    {
    case IdRole:
        return binderItem.id;
    case CreatedAtRole:
        return binderItem.createdAt;
    case UpdatedAtRole:
        return binderItem.updatedAt;
    case TitleRole:
        return binderItem.title;
    case SubTitleRole:
        return binderItem.subTitle;
    case RoleRole:
        return binderItem.role;
    case DictLanguageRole:
        return binderItem.dictLanguage;
    case ContentsRole: {
        QVariantList list;
        for (int id : binderItem.contents)
        {
            list.append(id);
        }
        return list;
    }
    case BinderItemsRole: {
        QVariantList list;
        for (const int id : binderItem.binderItems)
        {
            list.append(id);
        }
        return list;
    }
    case ParentRole:
        return binderItem.parentItem;
    default:;
    }

    return {};
}

bool BinderItemListModelFromBinderBinderItems::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid() || index.row() >= m_binderItems.size())
        return false;

    auto &binderItem = m_binderItems[index.row()];
    bool changed = false;

    switch (role)
    {
    case TitleRole:
        if (binderItem.title != value.toString())
        {
            binderItem.title = value.toString();
            changed = true;
        }
        break;
    case SubTitleRole:
        if (binderItem.subTitle != value.toString())
        {
            binderItem.subTitle = value.toString();
            changed = true;
        }
        break;
    case RoleRole:
        if (binderItem.role != value.toString())
        {
            binderItem.role = value.toString();
            changed = true;
        }
        break;
    case DictLanguageRole:
        if (binderItem.dictLanguage != value.toString())
        {
            binderItem.dictLanguage = value.toString();
            changed = true;
        }
        break;
    default:;
    }

    if (changed)
    {
        // Update via controller
        if (m_binderItemController)
        {
            auto dto = BinderItem::BinderItemDto(binderItem);
            // Use QTimer::singleShot for async update without coroutine in this context
            QCoro::Task<QList<BinderItem::BinderItemDto>> updateTask = m_binderItemController->update({dto});

            QCoro::connect(std::move(updateTask), this, [this, index, role](auto &&result) {
                if (!result.isEmpty())
                {
                    // Update local items with returned data (in case of any changes from backend)
                    m_binderItems[index.row()] = result.first();
                    Q_EMIT dataChanged(index, index, {role});
                }
            });
        }
        else
        {
            qWarning() << "BinderItemController not available for update";
        }
    }
    return changed;
}

Qt::ItemFlags BinderItemListModelFromBinderBinderItems::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEnabled | Qt::ItemIsSelectable | Qt::ItemIsEditable;
}

QHash<int, QByteArray> BinderItemListModelFromBinderBinderItems::roleNames() const
{
    QHash<int, QByteArray> names;
    names[IdRole] = "itemId";
    names[CreatedAtRole] = "createdAt";
    names[UpdatedAtRole] = "updatedAt";
    names[TitleRole] = "title";
    names[SubTitleRole] = "subTitle";
    names[RoleRole] = "role";
    names[DictLanguageRole] = "dictLanguage";
    names[ContentsRole] = "contents";
    names[BinderItemsRole] = "binderItems";
    names[ParentRole] = "parent";
    return names;
}

int BinderItemListModelFromBinderBinderItems::binderId() const
{
    return m_binderId;
}

void BinderItemListModelFromBinderBinderItems::setBinderId(int binderId)
{
    if (m_binderId != binderId)
    {
        m_binderId = binderId;
        refreshData();
        Q_EMIT binderIdChanged();
    }
}

int BinderItemListModelFromBinderBinderItems::parentId() const
{
    return m_parentId;
}

void BinderItemListModelFromBinderBinderItems::setParentId(int parentId)
{
    if (m_parentId != parentId)
    {
        m_parentId = parentId;
        refreshData();
        Q_EMIT parentIdChanged();
    }
}

void BinderItemListModelFromBinderBinderItems::refreshData()
{
    if (!m_binderController || !m_binderItemController || m_binderId <= 0)
        return;

    // top level items
    if (m_parentId <= 0)
    {
        // Use QTimer::singleShot to handle async operations without coroutines in this context
        QCoro::Task<QList<BinderItem::BinderItemDto>> fetchBinderItemsTask =
            m_binderController->getRelationshipIds(m_binderId, BinderRelationshipField::BinderItems)
                .then([this](auto &&binderItemIds) -> QCoro::Task<QList<BinderItem::BinderItemDto>> {
                    return m_binderItemController->get(binderItemIds);
                });

        QCoro::connect(std::move(fetchBinderItemsTask), this, [this](auto &&result) {
            if (!m_binderController || !m_binderItemController)
                return;

            beginResetModel();
            m_binderItems.clear();
            for (const BinderItem::BinderItemDto &dto : result)
            {
                if (dto.parentItem == 0) // Only top-level items
                    m_binderItems.append(dto);
            }
            endResetModel();

            qDebug() << "Refresh requested for binder ID:" << m_binderId << "parent ID:" << m_parentId;
        });
    }
    else
    {
        QCoro::Task<QList<BinderItem::BinderItemDto>> fetchBinderItemsTask =
            m_binderItemController->getRelationshipIds(m_parentId, BinderItem::BinderItemRelationshipField::BinderItems)
                .then([this](auto &&binderItemIds) -> QCoro::Task<QList<BinderItem::BinderItemDto>> {
                    return m_binderItemController->get(binderItemIds);
                });

        QCoro::connect(std::move(fetchBinderItemsTask), this, [this](auto &&result) {
            if (!m_binderController || !m_binderItemController)
                return;

            beginResetModel();
            m_binderItems = result;
            endResetModel();
            qDebug() << "Refresh requested for binder ID:" << m_binderId << "parent ID:" << m_parentId;
        });
    }
}

void BinderItemListModelFromBinderBinderItems::onBinderItemEventsUpdated(const QList<int> &ids)
{
    QList<int> relevantIds;
    for (const auto &item : m_binderItems)
    {
        if (ids.contains(item.id))
        {
            relevantIds.append(item.id);
        }
    }

    QCoro::Task<QList<BinderItem::BinderItemDto>> fetchBinderItemsTask = m_binderItemController->get(relevantIds);

    QCoro::connect(std::move(fetchBinderItemsTask), this, [this](auto &&result) {
        if (!m_binderController || !m_binderItemController)
            return;

        // Update existing items
        for (const BinderItem::BinderItemDto &dto : result)
        {
            for (int i = 0; i < m_binderItems.size(); ++i)
            {
                if (m_binderItems[i].id == dto.id)
                {
                    m_binderItems[i] = dto;
                    const QModelIndex idx = index(i);
                    Q_EMIT dataChanged(idx, idx);
                    break;
                }
            }
        }
    });
}

void BinderItemListModelFromBinderBinderItems::onBinderEventsUpdated(const QList<int> &ids)
{
    // Check if our binder was updated
    if (ids.contains(m_binderId))
    {
        QCoro::Task<QList<Binder::BinderDto>> fetchBinderTask = m_binderController->get({m_binderId});

        QCoro::connect(std::move(fetchBinderTask), this, [this](auto &&result) {
            if (!m_binderController || !m_binderItemController)
                return;

            if (result.isEmpty())
                return;

            const auto &binder = result.first();

            QList<int> currentItemIds;
            for (const auto &item : m_binderItems)
            {
                currentItemIds.append(item.id);
            }

            // If binder items changed, refresh data
            if (binder.binderItems != currentItemIds)
            {
                refreshData();
            }
        });
    }
}

} // namespace Skribisto::DirectAccess::Binder
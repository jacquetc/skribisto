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

#include "single_binder_item.h"

#include "service_locator.h"

namespace Skribisto::DirectAccess::BinderItem
{
SingleBinderItem::SingleBinderItem(QObject *parent) : QObject(parent)
{
    resolveDependencies();
}

void SingleBinderItem::resolveDependencies()
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

    // Connect to events
    if (m_eventRegistry)
    {
        auto binderItemEvents = m_eventRegistry->getEvents<Common::DirectAccess::BinderItem::BinderItemEvents>();
        if (binderItemEvents)
        {
            connect(binderItemEvents.data(), SIGNAL(updated(QList<int>)), this,
                    SLOT(onBinderItemEventsUpdated(QList<int>)));
        }
    }
}

void SingleBinderItem::onBinderItemEventsUpdated(const QList<int> &ids)
{
    if (m_id > 0 && ids.contains(m_id))
    {
        refreshData();
    }
}

void SingleBinderItem::refreshData()
{
    if (m_id <= 0)
        return;

    auto getBinderItemTask = m_binderItemController->get({m_id});

    QCoro::connect(std::move(getBinderItemTask), this, [this](const QList<BinderItemDto> &&items) {
        if (!m_binderItemController)
            return;

        if (items.isEmpty())
            return;

        const auto &item = items.first();
        setId(item.id);
        setCreatedAt(item.createdAt);
        setUpdatedAt(item.updatedAt);
        setTitle(item.title);
        setSubTitle(item.subTitle);
        setRole(item.role);
        setDictLanguage(item.dictLanguage);
        setContents(item.contents);
        setBinderItems(item.binderItems);
        setParentItem(item.parentItem);
    });
}

int SingleBinderItem::id() const
{
    return m_id;
}
void SingleBinderItem::setId(int id)
{
    if (m_id != id)
    {
        m_id = id;
        refreshData();
        Q_EMIT itemIdChanged();
    }
}
QDateTime SingleBinderItem::createdAt() const
{
    return m_createdAt;
}
void SingleBinderItem::setCreatedAt(const QDateTime &createdAt)
{
    if (m_createdAt != createdAt)
    {
        m_createdAt = createdAt;
        Q_EMIT createdAtChanged();
    }
}
QDateTime SingleBinderItem::updatedAt() const
{
    return m_updatedAt;
}
void SingleBinderItem::setUpdatedAt(const QDateTime &updatedAt)
{
    if (m_updatedAt != updatedAt)
    {
        m_updatedAt = updatedAt;
        Q_EMIT updatedAtChanged();
    }
}
QString SingleBinderItem::title() const
{
    return m_title;
}
void SingleBinderItem::setTitle(const QString &title)
{
    if (m_title != title)
    {
        m_title = title;
        Q_EMIT titleChanged();
    }
}
QString SingleBinderItem::subTitle() const
{
    return m_subTitle;
}
void SingleBinderItem::setSubTitle(const QString &subTitle)
{
    if (m_subTitle != subTitle)
    {
        m_subTitle = subTitle;
        Q_EMIT subTitleChanged();
    }
}
QString SingleBinderItem::role() const
{
    return m_role;
}
void SingleBinderItem::setRole(const QString &role)
{
    if (m_role != role)
    {
        m_role = role;
        Q_EMIT roleChanged();
    }
}
QString SingleBinderItem::dictLanguage() const
{
    return m_dictLanguage;
}
void SingleBinderItem::setDictLanguage(const QString &dictLanguage)
{
    if (m_dictLanguage != dictLanguage)
    {
        m_dictLanguage = dictLanguage;
        Q_EMIT dictLanguageChanged();
    }
}
QList<int> SingleBinderItem::contents() const
{
    return m_contents;
}
void SingleBinderItem::setContents(const QList<int> &contents)
{
    if (m_contents != contents)
    {
        m_contents = contents;
        Q_EMIT contentsChanged();
    }
}
QList<int> SingleBinderItem::binderItems() const
{
    return m_binderItems;
}
void SingleBinderItem::setBinderItems(const QList<int> &binderItems)
{
    if (m_binderItems != binderItems)
    {
        m_binderItems = binderItems;
        Q_EMIT binderItemsChanged();
    }
}
int SingleBinderItem::parentItem() const
{
    return m_parentItem;
}
void SingleBinderItem::setParentItem(int parentItem)
{
    if (m_parentItem != parentItem)
    {
        m_parentItem = parentItem;
        Q_EMIT parentItemChanged();
    }
}
} // namespace Skribisto::DirectAccess::BinderItem
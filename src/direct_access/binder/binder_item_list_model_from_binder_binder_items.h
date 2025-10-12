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

#pragma once
#include "binder_item/dtos.h"

#include <QAbstractListModel>
#include <QPointer>

namespace Skribisto::Common::DirectAccess
{
class EventRegistry;
}

namespace Skribisto::DirectAccess::BinderItem
{
class BinderItemController;
}

namespace Skribisto::DirectAccess::Binder
{
class BinderController;

class BinderItemListModelFromBinderBinderItems : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int binderId READ binderId WRITE setBinderId NOTIFY binderIdChanged)
    Q_PROPERTY(int parentId READ parentId WRITE setParentId NOTIFY parentIdChanged)

  public:
    enum Roles
    {
        IdRole = Qt::UserRole + 1,
        CreatedAtRole,
        UpdatedAtRole,
        TitleRole,
        SubTitleRole,
        RoleRole,
        DictLanguageRole,
        ContentsRole,
        BinderItemsRole,
        ParentRole,
    };
    Q_ENUM(Roles)

    explicit BinderItemListModelFromBinderBinderItems(QObject *parent = nullptr);

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Add data:
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    QHash<int, QByteArray> roleNames() const override;

    int binderId() const;
    void setBinderId(int binderId);

    int parentId() const;
    void setParentId(int parentId);

  Q_SIGNALS:
    void binderIdChanged();
    void parentIdChanged();

  private Q_SLOTS:
    void onBinderItemEventsUpdated(const QList<int> &ids);
    void onBinderEventsUpdated(const QList<int> &ids);

  private:
    void resolveDependencies();
    void refreshData();

    int m_binderId = -1;
    int m_parentId = -1;
    QList<BinderItem::BinderItemDto> m_binderItems;

    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<BinderItem::BinderItemController> m_binderItemController;
    QPointer<BinderController> m_binderController;
};

} // namespace Skribisto::DirectAccess::Binder
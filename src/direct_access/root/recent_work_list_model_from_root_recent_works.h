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
#include "recent_work/dtos.h"

#include <QAbstractListModel>
#include <QPointer>

namespace Skribisto::Common::DirectAccess
{
class EventRegistry;
}

namespace Skribisto::DirectAccess::RecentWork
{
class RecentWorkController;
}

namespace Skribisto::DirectAccess::Root
{
class RootController;

class RecentWorkListModelFromRootRecentWorks : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int rootId READ rootId WRITE setRootId NOTIFY rootIdChanged)

  public:
    enum Roles
    {
        IdRole = Qt::UserRole + 1,
        CreatedAtRole,
        UpdatedAtRole,
        TitleRole,
        LastOpenedAtRole,
        AbsolutePathRole
    };
    Q_ENUM(Roles)

    explicit RecentWorkListModelFromRootRecentWorks(QObject *parent = nullptr);

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Add data:
    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    QHash<int, QByteArray> roleNames() const override;

    int rootId() const;
    void setRootId(int rootId);

  Q_SIGNALS:
    void rootIdChanged();

  private Q_SLOTS:
    void onRecentWorkEventsUpdated(const QList<int> &ids);
    void onRootEventsUpdated(const QList<int> &ids);

  private:
    void resolveDependencies();
    void refreshData();

    int m_rootId = -1;
    QList<RecentWork::RecentWorkDto> m_recentWorks;

    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<RecentWork::RecentWorkController> m_recentWorkController;
    QPointer<RootController> m_rootController;
};

} // namespace Skribisto::DirectAccess::Root

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
#include "binder_item_controller.h"
#include "direct_access/event_registry.h"

#include <QObject>

namespace Skribisto::DirectAccess::BinderItem
{

class SingleBinderItem : public QObject
{
    Q_OBJECT
    Q_PROPERTY(int itemId READ id WRITE setId NOTIFY itemIdChanged)
    Q_PROPERTY(QDateTime createdAt READ createdAt WRITE setCreatedAt NOTIFY createdAtChanged)
    Q_PROPERTY(QDateTime updatedAt READ updatedAt WRITE setUpdatedAt NOTIFY updatedAtChanged)
    Q_PROPERTY(QString title READ title WRITE setTitle NOTIFY titleChanged)
    Q_PROPERTY(QString subTitle READ subTitle WRITE setSubTitle NOTIFY subTitleChanged)
    Q_PROPERTY(QString role READ role WRITE setRole NOTIFY roleChanged)
    Q_PROPERTY(QString dictLanguage READ dictLanguage WRITE setDictLanguage NOTIFY dictLanguageChanged)
    Q_PROPERTY(QList<int> contents READ contents WRITE setContents NOTIFY contentsChanged)
    Q_PROPERTY(QList<int> binderItems READ binderItems WRITE setBinderItems NOTIFY binderItemsChanged)
    Q_PROPERTY(int parentItem READ parentItem WRITE setParentItem NOTIFY parentItemChanged)

  public:
    SingleBinderItem(QObject *parent = nullptr);

    int id() const;
    void setId(int id);
    QDateTime createdAt() const;
    void setCreatedAt(const QDateTime &createdAt);
    QDateTime updatedAt() const;
    void setUpdatedAt(const QDateTime &updatedAt);
    QString title() const;
    void setTitle(const QString &title);
    QString subTitle() const;
    void setSubTitle(const QString &subTitle);
    QString role() const;
    void setRole(const QString &role);
    QString dictLanguage() const;
    void setDictLanguage(const QString &dictLanguage);
    QList<int> contents() const;
    void setContents(const QList<int> &contents);
    QList<int> binderItems() const;
    void setBinderItems(const QList<int> &binderItems);
    int parentItem() const;
    void setParentItem(int parentItem);

  Q_SIGNALS:
    void itemIdChanged();
    void createdAtChanged();
    void updatedAtChanged();
    void titleChanged();
    void subTitleChanged();
    void roleChanged();
    void dictLanguageChanged();
    void contentsChanged();
    void binderItemsChanged();
    void parentItemChanged();

  private Q_SLOTS:
    void onBinderItemEventsUpdated(const QList<int> &ids);

  private:
    void resolveDependencies();
    void refreshData();

    int m_id = -1;
    QDateTime m_createdAt;
    QDateTime m_updatedAt;
    QString m_title;
    QString m_subTitle;
    QString m_role;
    QString m_dictLanguage;
    QList<int> m_contents;
    QList<int> m_binderItems;
    int m_parentItem = 0;

    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<BinderItem::BinderItemController> m_binderItemController;
};

} // namespace Skribisto::DirectAccess::BinderItem
/***************************************************************************
 *   Copyright (C) 2020 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrtagitem.h                                                   *
 *  This file is part of Skribisto.                                    *
 *                                                                         *
 *  Skribisto is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Skribisto is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#ifndef SKRTAGITEM_H
#define SKRTAGITEM_H

#include <QtCore>
#include <QObject>
#include "./skribisto_data_global.h"

class EXPORT SKRTagItem : public QObject
{
    Q_OBJECT
public:
    enum Roles {
        // papers :
        NameRole  = Qt::UserRole,
        ProjectIdRole    = Qt::UserRole + 1,
        TagIdRole      = Qt::UserRole + 2,
        ColorRole      = Qt::UserRole + 3,
        CreationDateRole = Qt::UserRole + 4,
        UpdateDateRole   = Qt::UserRole + 5
    };
    Q_ENUM(Roles)



    explicit SKRTagItem();
    explicit SKRTagItem(int projectId,
                        int tagId);

    ~SKRTagItem();

    void          invalidateData(int role);
    void          invalidateAllData();


    int           projectId();
    int           tagId();
    Q_INVOKABLE QString       name();
    QString color();

    QVariant      data(int role);
    QList<int>    dataRoles() const;

    bool          isRootItem() const;
    void          setIsRootItem();

signals:
private:

    QHash<int, QVariant>m_data;
    QList<int> m_invalidatedRoles;
    bool m_isRootItem;

};

#endif // SKRTAGITEM_H

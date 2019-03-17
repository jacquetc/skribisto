/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsqlstrings                                                   *
*  This file is part of Plume Creator.                                    *
*                                                                         *
*  Plume Creator is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Plume Creator is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#ifndef PLMSQLQUERIES_H
#define PLMSQLQUERIES_H

#include <QObject>

#include <QString>
#include <QStringList>
#include <QVariant>
#include <QHash>
#include <QSqlQuery>
#include <QtSql/QSqlDatabase>
#include "../plmerror.h"


class PLMSqlQueries : public QObject {
public:

    enum DBType { ProjectDB, UserDB };
    Q_ENUM(DBType)

    explicit PLMSqlQueries(int            projectId,
                           const QString& tableName);
    explicit PLMSqlQueries(QSqlDatabase   sqlDB,
                           const QString& tableName,
                           const QString& idName);

    PLMError get(int            id,
                 const QString& valueName,
                 QVariant     & result) const;
    PLMError getMultipleValues(int id,
                               const QStringList& valueList,
                               QHash<QString, QVariant>& result) const;
    PLMError getValueByIds(const QString& valueName,
                           QHash<int, QVariant>& result,
                           const QString& where       = QString(),
                           const QVariant& whereValue = QVariant(),
                           bool sorted                = false) const;
    PLMError getValueByIdsWhere(const QString& valueName,
                                QHash<int, QVariant>& result,
                                const QHash<QString, QVariant>& where,
                                bool sorted = false) const;
    PLMError set(int             id,
                 const QString & valueName,
                 const QVariant& value) const;

    void     beginTransaction();
    void     rollback();
    void     commit();


    PLMError setCurrentDate(int            id,
                            const QString& valueName) const;
    PLMError getSortedIds(QList<int>& result) const;
    PLMError getIds(QList<int>& result) const;
    bool     idExists(int id) const;
    bool     resultExists(const QHash<QString, QVariant>& where) const;
    PLMError add(const QHash<QString, QVariant>& values,
                 int& newId) const;
    PLMError removeAll() const;
    PLMError remove(int id) const;
    PLMError renumberSortOrder();
    PLMError setId(int id,
                   int newId) const;

    PLMError injectDirectSql(const QString& sqlString);

private:

    QSqlDatabase m_sqlDB;
    int m_projectId;
    QString m_tableName, m_idName;

    //
};


#endif // PLMSQLQUERIES_H

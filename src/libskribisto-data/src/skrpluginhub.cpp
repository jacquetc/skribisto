/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpluginhub.cpp
*                                                  *
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
#include "skrpluginhub.h"
#include "sql/plmsqlqueries.h"

SKRPluginHub::SKRPluginHub(QObject *parent) : SKRPluginLoader(parent)
{
    qRegisterMetaType<SKRCommand>("SKRCommand");
}

void      SKRPluginHub::reloadPlugins()
{}

SKRResult SKRPluginHub::set(int projectId, int id,
                            const QString& tableName,
                            const QString& fieldName,
                            const QVariant& value)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, tableName);

    queries.beginTransaction();
    result = queries.set(id, fieldName, value);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

QVariant SKRPluginHub::get(int            projectId,
                           int            id,
                           const QString& tableName,
                           const QString& fieldName) const
{
    SKRResult result(this);
    QVariant  var;
    QVariant  value;
    PLMSqlQueries queries(projectId, tableName);

    result = queries.get(id, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

QList<int>SKRPluginHub::getIds(int            projectId,
                               const QString& tableName) const
{
    SKRResult result(this);

    QList<int> list;
    QList<int> finalList;
    PLMSqlQueries queries(projectId, tableName);

    result = queries.getIds(list);
    IFOK(result) {
        finalList = list;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return finalList;
}

SKRResult SKRPluginHub::ensureTableExists(int            projectId,
                                          const QString& tableName,
                                          const QString& sqlString)
{
    SKRResult result(this);

    // TODO: check if table exist:


    // if table doesn't exist :


    PLMSqlQueries queries(projectId, tableName);

    result = queries.injectDirectSql(sqlString);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

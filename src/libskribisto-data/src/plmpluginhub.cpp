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
#include "plmpluginhub.h"
#include "tasks/plmsqlqueries.h"

PLMPluginHub::PLMPluginHub(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<PLMCommand>("PLMCommand");
}

void     PLMPluginHub::reloadPlugins()
{}

PLMError PLMPluginHub::set(int projectId, int id,
                           const QString& tableName,
                           const QString& fieldName,
                           const QVariant& value)
{
    PLMError error;
    PLMSqlQueries queries(projectId, tableName);

    queries.beginTransaction();
    error = queries.set(id, fieldName, value);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

QVariant PLMPluginHub::get(int            projectId,
                           int            id,
                           const QString& tableName,
                           const QString& fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    PLMSqlQueries queries(projectId, tableName);

    error = queries.get(id, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QList<int>PLMPluginHub::getIds(int            projectId,
                               const QString& tableName) const
{
    PLMError error;

    QList<int> list;
    QList<int> result;
    PLMSqlQueries queries(projectId, tableName);

    error = queries.getIds(list);
    IFOK(error) {
        result = list;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

PLMError PLMPluginHub::ensureTableExists(int            projectId,
                                         const QString& tableName,
                                         const QString& sqlString)
{
    PLMError error;

    // TODO: check if table exist:


    // if table doesn't exist :


    PLMSqlQueries queries(projectId, tableName);

    error = queries.injectDirectSql(sqlString);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

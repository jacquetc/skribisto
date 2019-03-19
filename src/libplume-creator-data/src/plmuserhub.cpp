/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmuserfilehub.cpp
*                                                  *
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
#include "plmuserhub.h"
#include "tasks/plmsqlqueries.h"

PLMUserHub::PLMUserHub(QObject *parent) : QObject(parent)
{
    // connection for 'getxxx' functions to have a way to get errors.
    connect(this,
            &PLMUserHub::errorSent,
            this,
            &PLMUserHub::setError,
            Qt::DirectConnection);
}

void PLMUserHub::setError(const PLMError& error)
{
    m_error = error;
}

// ------------------------------------------------------------


// ------------------------------------------------------------


QVariant PLMUserHub::get(int            projectId,
                         const QString& tableName,
                         int            id,
                         const QString& fieldName) const
{
    PLMError error;

    if (tableName.left(8) != "tbl_user") {
        error.setSuccess(false);
    }
    IFKO(error) {
        emit errorSent(error);

        return QVariant();
    }


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

// ------------------------------------------------------------

PLMError PLMUserHub::getMultipleValues(int projectId,
                                       const QString& tableName,
                                       int id,
                                       const QStringList& valueList,
                                       QHash<QString, QVariant>& result)
{
    PLMSqlQueries queries(projectId, tableName);
    PLMError error = queries.getMultipleValues(id, valueList, result);

    return error;
}

// ------------------------------------------------------------


PLMError PLMUserHub::setCurrentDate(int            projectId,
                                    const QString& tableName,
                                    int            id,
                                    const QString& fieldName) const
{
    PLMSqlQueries queries(projectId, tableName);
    PLMError error = queries.setCurrentDate(id, fieldName);

    return error;
}

// ------------------------------------------------------------


PLMError PLMUserHub::getIds(int projectId, const QString& tableName,
                            QList<int>& result) const
{
    PLMSqlQueries queries(projectId, tableName);
    PLMError error = queries.getIds(result);

    return error;
}

// ------------------------------------------------------------

PLMError PLMUserHub::add(int                    projectId,
                         const QString        & tableName,
                         const QHash<QString,
                                     QVariant>& values,
                         int                  & newId) const
{
    PLMSqlQueries queries(projectId, tableName);
    PLMError error = queries.add(values, newId);

    return error;
}

// ------------------------------------------------------------

PLMError PLMUserHub::set(int             projectId,
                         const QString & tableName,
                         int             id,
                         const QString & fieldName,
                         const QVariant& value,
                         bool            setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId, tableName);

    queries.beginTransaction();
    error = queries.set(id, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(id, "dt_updated"));
    }

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

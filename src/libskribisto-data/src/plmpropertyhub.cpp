/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpropertyhub.cpp
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
#include "plmpropertyhub.h"
#include "tools.h"
#include "tasks/plmsqlqueries.h"

#include <QDebug>

PLMPropertyHub::PLMPropertyHub(QObject       *parent,
                               const QString& tableName,
                               const QString& paperCodeFieldName)
    : QObject(parent), m_tableName(tableName), m_paperCodeFieldName(paperCodeFieldName),
    m_last_added_id(-1)
{}

QHash<int, QString>PLMPropertyHub::getAllNames(int projectId) const
{
    PLMError error;

    QHash<int, QString>  result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("t_name", out);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntQString(out);
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QHash<int, QString>PLMPropertyHub::getAllValues(int projectId) const
{
    PLMError error;

    QHash<int, QString>  result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("t_value", out);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntQString(out);
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QHash<int, bool>PLMPropertyHub::getAllIsSystems(int projectId) const
{
    PLMError error;

    QHash<int, bool> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("b_system", out);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntBool(out);
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QHash<int, int>PLMPropertyHub::getAllPaperCodes(int projectId) const
{
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds(m_paperCodeFieldName, out);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QList<int>PLMPropertyHub::getAllIds(int projectId) const
{
    PLMError error;

    QList<int> result;
    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getIds(out);

    IFOK(error) {
        result = out;
    }


    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

QList<int>PLMPropertyHub::getAllIdsWithPaperCode(int projectId, int paperCode) const
{
    PLMError error;

    QList<int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("l_property_id", out, m_paperCodeFieldName, paperCode);

    IFOK(error) {
        result = out.keys();
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

PLMError PLMPropertyHub::setProperty(int            projectId,
                                     int            paperCode,
                                     const QString& name,
                                     const QString& value)
{
    PLMError error;
    int propertyId = -1;

    if (propertyExists(projectId, paperCode, name)) {
        propertyId = findPropertyId(projectId, paperCode, name);
    }
    else {
        IFOKDO(error, addProperty(projectId, paperCode, propertyId));
        IFOK(error) {
            propertyId = getLastAddedId();
        }
    }

    IFOKDO(error, setPropertyById(projectId, propertyId, name, value));


    return error;
}

PLMError PLMPropertyHub::setPropertyById(int            projectId,
                                         int            propertyId,
                                         const QString& name,
                                         const QString& value)
{
    PLMError error;

    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();

    error = queries.set(propertyId, "t_name", name);
    IFOKDO(error, queries.set(propertyId, "t_value", value));
    IFOKDO(error, queries.setCurrentDate(propertyId, "dt_updated"));
    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    QVariant result;

    IFOKDO(error, queries.get(propertyId, m_paperCodeFieldName, result));
    IFOK(error) {
        emit propertyChanged(projectId, propertyId, result.toInt(), name, value);
        emit projectModified(projectId);
    }
    IFKO(error) {
        emit errorSent(error);
    }

    return error;
}

// ---------------------------------------------------------------------

PLMError PLMPropertyHub::setId(int projectId, int propertyId, int newId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();

    PLMError error = queries.setId(propertyId, newId);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit idChanged(projectId, propertyId, newId);
        emit projectModified(projectId);
    }
    return error;
}

// ---------------------------------------------------------------------

PLMError PLMPropertyHub::setValue(int projectId, int propertyId, const QString& value)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant result;

    queries.get(propertyId, "t_name",             result);
    QString name = result.toString();

    queries.get(propertyId, m_paperCodeFieldName, result);
    int paperCode = result.toInt();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, "t_value", value);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return error;
}

PLMError PLMPropertyHub::setName(int projectId, int propertyId, const QString& name)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant result;

    queries.get(propertyId, "t_value",            result);
    QString value = result.toString();

    queries.get(propertyId, m_paperCodeFieldName, result);
    int paperCode = result.toInt();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, "t_name", name);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return error;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getName(int projectId, int propertyId)
{
    PLMError error;
    QString  result;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, "t_name", out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toString();
    return result;
}

// ---------------------------------------------------------------------

PLMError PLMPropertyHub::setPaperCode(int projectId, int propertyId, int paperCode)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant result;

    queries.get(propertyId, "t_value", result);
    QString value = result.toString();

    queries.get(propertyId, "t_name",  result);
    QString name = result.toString();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, m_paperCodeFieldName, paperCode);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return error;
}

// ---------------------------------------------------------------------

int PLMPropertyHub::getPaperCode(int projectId, int propertyId)
{
    PLMError error;
    int result;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, m_paperCodeFieldName, out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toInt();
    return result;
}

PLMError PLMPropertyHub::setCreationDate(int              projectId,
                                         int              propertyId,
                                         const QDateTime& date)
{
    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "t_value", result);
    //    QString value = result.toString();
    //    queries.get(propertyId, "t_name", result);
    //    QString name = result.toString();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, "dt_created", date);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return error;
}

// ---------------------------------------------------------------------

QDateTime PLMPropertyHub::getCreationDate(int projectId, int propertyId) const
{
    PLMError  error;
    QDateTime result;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, "dt_created", out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toDateTime();
    return result;
}

// ---------------------------------------------------------------------

PLMError PLMPropertyHub::setModificationDate(int              projectId,
                                             int              propertyId,
                                             const QDateTime& date)
{
    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "t_value", result);
    //    QString value = result.toString();
    //    queries.get(propertyId, "t_name", result);
    //    QString name = result.toString();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, "dt_updated", date);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return error;
}

// ---------------------------------------------------------------------

QDateTime PLMPropertyHub::getModificationDate(int projectId, int propertyId) const
{
    PLMError  error;
    QDateTime result;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, "dt_updated", out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toDateTime();
    return result;
}

PLMError PLMPropertyHub::setSystem(int projectId, int propertyId, bool isSystem)
{
    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "t_value", result);
    //    QString value = result.toString();
    //    queries.get(propertyId, "t_name", result);
    //    QString name = result.toString();

    queries.beginTransaction();

    PLMError error = queries.set(propertyId, "b_system", isSystem);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return error;
}

// ---------------------------------------------------------------------


bool PLMPropertyHub::getSystem(int projectId, int propertyId) const
{
    PLMError error;
    bool     result;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, "b_system", out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toBool();
    return result;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getProperty(int projectId, int paperCode,
                                    const QString& name) const
{
    PLMError error;
    QString  result;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);
    error = queries.getValueByIdsWhere("t_value", out, where);

    if (out.isEmpty()) {
        return result;
    }

    IFOK(error) {
        result = out.values().first().toString();
    }

    IFKO(error) {
        emit errorSent(error);
    }

    return result;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getProperty(int            projectId,
                                    int            paperCode,
                                    const QString& name,
                                    const QString& defaultValue) const
{
    QString result = getProperty(projectId, paperCode, name);

    if (result.isNull()) {
        result = defaultValue;
    }
    return result;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getPropertyById(int projectId, int propertyId) const
{
    PLMError error;
    QString  result;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(propertyId, "t_value", out);
    IFKO(error) {
        emit errorSent(error);
    }
    result = out.toString();
    return result;
}

// -----------------------------------------------------------------------------

int PLMPropertyHub::getLastAddedId()
{
    return m_last_added_id;
}

// ---------------------------------------------------------------------


PLMError PLMPropertyHub::addProperty(int projectId, int paperCode, int imposedPropertyId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    QHash<QString, QVariant> values;

    values.insert(m_paperCodeFieldName, paperCode);

    if (imposedPropertyId != -1) values.insert("l_property_id", imposedPropertyId);
    int newPropertyId = -1;
    PLMError error    = queries.add(values, newPropertyId);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
        m_last_added_id = newPropertyId;
        emit propertyAdded(projectId, newPropertyId);
        emit projectModified(projectId);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

PLMError PLMPropertyHub::removeProperty(int projectId, int propertyId)
{
    PLMSqlQueries queries(projectId, m_tableName);
    PLMError error = queries.remove(propertyId);

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
        emit propertyRemoved(projectId, propertyId);
        emit projectModified(projectId);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

bool PLMPropertyHub::propertyExists(int projectId, int paperCode, const QString& name)
{
    PLMSqlQueries queries(projectId, m_tableName);


    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);

    return queries.resultExists(where);
}

int PLMPropertyHub::findPropertyId(int projectId, int paperCode, const QString& name)
{
    PLMError error;
    int result = 0;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);
    error = queries.getValueByIdsWhere("l_property_id", out, where);

    IFOK(error) {
        result = out.keys().first();
    }

    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

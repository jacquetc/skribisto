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
    SKRResult result(this);

    QHash<int, QString>  hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("t_name", out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntQString(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QHash<int, QString>PLMPropertyHub::getAllValues(int projectId) const
{
    SKRResult result(this);

    QHash<int, QString>  hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("t_value", out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntQString(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QHash<int, bool>PLMPropertyHub::getAllIsSystems(int projectId) const
{
    SKRResult result(this);

    QHash<int, bool> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("b_system", out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntBool(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QHash<int, int>PLMPropertyHub::getAllPaperCodes(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds(m_paperCodeFieldName, out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QList<int>PLMPropertyHub::getAllIds(int projectId) const
{
    SKRResult result(this);

    QList<int> list;
    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getIds(out);

    IFOK(result) {
        list = out;
    }


    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

QList<int>PLMPropertyHub::getAllIdsWithPaperCode(int projectId, int paperCode) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_property_id", out, m_paperCodeFieldName, paperCode);

    IFOK(result) {
        list = out.keys();
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

SKRResult PLMPropertyHub::setProperty(int            projectId,
                                      int            paperCode,
                                      const QString& name,
                                      const QString& value,
                                      bool isSystem,
                                      bool triggerProjectModifiedSignal)
{
    SKRResult result(this);
    int propertyId = -1;

    if (propertyExists(projectId, paperCode, name)) {
        propertyId = findPropertyId(projectId, paperCode, name);
    }
    else {
        IFOKDO(result, addProperty(projectId, paperCode, propertyId));
        IFOK(result) {
            propertyId = getLastAddedId();
        }
    }



    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();

    IFOKDO(result, queries.set(propertyId, "t_name", name));
    IFOKDO(result, queries.set(propertyId, "t_value", value));
    IFOKDO(result, queries.set(propertyId, "b_system", isSystem));
    IFOKDO(result, queries.setCurrentDate(propertyId, "dt_updated"));
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    QVariant variant;

    IFOKDO(result, queries.get(propertyId, m_paperCodeFieldName, variant));
    IFOK(result) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        if(triggerProjectModifiedSignal){
            emit projectModified(projectId);
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}



// ---------------------------------------------------------------------

SKRResult PLMPropertyHub::setId(int projectId, int propertyId, int newId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();

    SKRResult result = queries.setId(propertyId, newId);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit idChanged(projectId, propertyId, newId);
        emit projectModified(projectId);
    }
    return result;
}

// ---------------------------------------------------------------------

SKRResult PLMPropertyHub::setValue(int projectId, int propertyId, const QString& value)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "t_name",             variant);
    QString name = variant.toString();

    queries.get(propertyId, m_paperCodeFieldName, variant);
    int paperCode = variant.toInt();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, "t_value", value);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return result;
}

SKRResult PLMPropertyHub::setName(int projectId, int propertyId, const QString& name)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "t_value",            variant);
    QString value = variant.toString();

    queries.get(propertyId, m_paperCodeFieldName, variant);
    int paperCode = variant.toInt();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, "t_name", name);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return result;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getName(int projectId, int propertyId)
{
    SKRResult result(this);
    QString  string;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "t_name", out);
    IFKO(result) {
        emit errorSent(result);
    }
    string = out.toString();
    return string;
}

// ---------------------------------------------------------------------

SKRResult PLMPropertyHub::setPaperCode(int projectId, int propertyId, int paperCode)
{
    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "t_value", variant);
    QString value = variant.toString();

    queries.get(propertyId, "t_name",  variant);
    QString name = variant.toString();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, m_paperCodeFieldName, paperCode);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit propertyChanged(projectId, propertyId, paperCode, name, value);
        emit projectModified(projectId);
    }
    return result;
}

// ---------------------------------------------------------------------

int PLMPropertyHub::getPaperCode(int projectId, int propertyId)
{
    SKRResult result(this);
    int value;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, m_paperCodeFieldName, out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toInt();
    return value;
}

SKRResult PLMPropertyHub::setCreationDate(int              projectId,
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

    SKRResult result = queries.set(propertyId, "dt_created", date);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit projectModified(projectId);

        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return result;
}

// ---------------------------------------------------------------------

QDateTime PLMPropertyHub::getCreationDate(int projectId, int propertyId) const
{
    SKRResult  result;
    QDateTime date;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "dt_created", out);
    IFKO(result) {
        emit errorSent(result);
    }
    date = out.toDateTime();
    return date;
}

// ---------------------------------------------------------------------

SKRResult PLMPropertyHub::setModificationDate(int              projectId,
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

    SKRResult result = queries.set(propertyId, "dt_updated", date);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit projectModified(projectId);

        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return result;
}

// ---------------------------------------------------------------------

QDateTime PLMPropertyHub::getModificationDate(int projectId, int propertyId) const
{
    SKRResult  result;
    QDateTime date;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "dt_updated", out);
    IFKO(result) {
        emit errorSent(result);
    }
    date = out.toDateTime();
    return date;
}

SKRResult PLMPropertyHub::setIsSystem(int projectId, int propertyId, bool isSystem)
{
    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "t_value", result);
    //    QString value = result.toString();
    //    queries.get(propertyId, "t_name", result);
    //    QString name = result.toString();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, "b_system", isSystem);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit projectModified(projectId);

        // emit propertyChanged(projectId, propertyId, paperCode, name, value);
    }
    return result;
}

// ---------------------------------------------------------------------


bool PLMPropertyHub::getIsSystem(int projectId, int propertyId) const
{
    SKRResult result(this);
    bool     value;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "b_system", out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toBool();
    return value;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getProperty(int projectId, int paperCode,
                                    const QString& name) const
{
    SKRResult result(this);
    QString  value;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);
    result = queries.getValueByIdsWhere("t_value", out, where);

    if (out.isEmpty()) {
        return value;
    }

    IFOK(result) {
        value = out.values().first().toString();
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return value;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getProperty(int            projectId,
                                    int            paperCode,
                                    const QString& name,
                                    const QString& defaultValue) const
{
    QString value = getProperty(projectId, paperCode, name);

    if (value.isNull()) {
        value = defaultValue;
    }
    return value;
}

// ---------------------------------------------------------------------

QString PLMPropertyHub::getPropertyById(int projectId, int propertyId) const
{
    SKRResult result(this);
    QString  value;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "t_value", out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toString();
    return value;
}

// ---------------------------------------------------------------------

int PLMPropertyHub::getPropertyId(int projectId, int paperCode, const QString& name) const
{
    SKRResult result(this);
    int  value = -2;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);
    result = queries.getValueByIdsWhere("t_value", out, where);

    if (out.isEmpty()) {
        return value;
    }

    IFOK(result) {
        value = out.keys().first();
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return value;
}
// -----------------------------------------------------------------------------

int PLMPropertyHub::getLastAddedId()
{
    return m_last_added_id;
}

// ---------------------------------------------------------------------


SKRResult PLMPropertyHub::addProperty(int projectId, int paperCode, int imposedPropertyId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    QHash<QString, QVariant> values;

    values.insert(m_paperCodeFieldName, paperCode);

    if (imposedPropertyId != -1) values.insert("l_property_id", imposedPropertyId);
    int newPropertyId = -1;
    SKRResult result    = queries.add(values, newPropertyId);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        m_last_added_id = newPropertyId;
        emit propertyAdded(projectId, newPropertyId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

SKRResult PLMPropertyHub::removeProperty(int projectId, int propertyId)
{
    PLMSqlQueries queries(projectId, m_tableName);
    SKRResult result = queries.remove(propertyId);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        emit propertyRemoved(projectId, propertyId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
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
    SKRResult result(this);
    int value = 0;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_paperCodeFieldName, paperCode);
    where.insert("t_name",             name);
    result = queries.getValueByIdsWhere("l_property_id", out, where);

    IFOK(result) {
        value = out.keys().first();
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

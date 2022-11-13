/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpropertyhub.cpp
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
#include "skrpropertyhub.h"
#include "tools.h"
#include "sql/plmsqlqueries.h"

SKRPropertyHub::SKRPropertyHub(QObject       *parent,
                               const QString& tableName,
                               const QString& codeFieldName)
    : QObject(parent), m_tableName(tableName), m_codeFieldName(codeFieldName),
    m_last_added_id(-1)
{}

QHash<int, QString>SKRPropertyHub::getAllNames(int projectId) const
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

QHash<int, QString>SKRPropertyHub::getAllValues(int projectId) const
{
    SKRResult result(this);

    QHash<int, QString>  hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("m_value", out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntQString(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QHash<int, bool>SKRPropertyHub::getAllIsSystems(int projectId) const
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

QHash<int, int>SKRPropertyHub::getAllPaperCodes(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds(m_codeFieldName, out);

    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

QList<int>SKRPropertyHub::getAllIds(int projectId) const
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

QList<int>SKRPropertyHub::getAllIdsWithPaperCode(int projectId, int treeItemCode) const
{
    SKRResult result(this);

    QList<int> list;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds(queries.getIdName(), out, m_codeFieldName, treeItemCode);

    IFOK(result) {
        list = out.keys();
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// --------------------------------------------------------------


QList<QVariantMap> SKRPropertyHub::save(int projectId) const
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);
    QStringList fieldNames = queries.getAllFieldTitles();

    QVariantMap allFields;
    QList<QVariantMap> list;

    for(int propertyId : this->getAllIds(projectId)){

        for(const QString &fieldName : fieldNames) {
            allFields.insert(fieldName, this->get(projectId, propertyId, fieldName));
        }

        list.append(allFields);
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return list;

}

// ----------------------------------------------------------------------------------------

SKRResult SKRPropertyHub::restore(int projectId, QList<QVariantMap> allValues)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.injectDirectSql("PRAGMA foreign_keys = 0");
    result = queries.injectDirectSql("DELETE FROM tbl_tree");

    for(const QVariantMap &values : allValues){

        QHash<QString, QVariant> hash;
        QVariantMap::const_iterator i = values.constBegin();
        while (i != values.constEnd()) {
            hash.insert(i.key(), i.value());
            ++i;
        }
        int newId;
        queries.add(hash, newId);

    }
    result = queries.injectDirectSql("PRAGMA foreign_keys = 1");


    IFOK(result) {
        queries.commit();
        emit propertiesReset(projectId);
        emit projectModified(projectId);
    }
    IFKO(result) {
        queries.rollback();
        emit errorSent(result);
    }
    return result;

}

// --------------------------------------------------------------
SKRResult SKRPropertyHub::setProperty(int            projectId,
                                      int            treeItemCode,
                                      const QString& name,
                                      const QString& value,
                                      bool           isSystem,
                                      bool           isSilent,
                                      bool           triggerProjectModifiedSignal)
{
    SKRResult result(this);
    int propertyId = -2;

    if (propertyExists(projectId, treeItemCode, name)) {
        propertyId = findPropertyId(projectId, treeItemCode, name);
    }
    else {
        IFOKDO(result, addProperty(projectId, treeItemCode));
        IFOK(result) {
            propertyId = getLastAddedId();
        }
    }


    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();

    IFOKDO(result, queries.set(propertyId, "t_name", name));
    IFOKDO(result, queries.set(propertyId, "m_value", value));
    IFOKDO(result, queries.set(propertyId, "b_system", isSystem));
    IFOKDO(result, queries.set(propertyId, "b_silent", isSilent));
    IFOKDO(result, queries.setCurrentDate(propertyId, "dt_updated"));
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    QVariant variant;

    IFOKDO(result, queries.get(propertyId, m_codeFieldName, variant));
    IFOK(result) {
        emit propertyChanged(projectId, propertyId, treeItemCode, name, value);

        if (triggerProjectModifiedSignal && !isSilent) {
            emit projectModified(projectId);
        }

        result.addData("propertyId", propertyId);
    }
    IFKO(result) {
        emit errorSent(result);
    }

    return result;
}

// ---------------------------------------------------------------------

SKRResult SKRPropertyHub::setId(int projectId, int propertyId, int newId)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

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

        if (!isSilent) {
            emit projectModified(projectId);
        }
    }
    return result;
}

// ---------------------------------------------------------------------

SKRResult SKRPropertyHub::setValue(int projectId, int propertyId, const QString& value)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "t_name",        variant);
    QString name = variant.toString();

    queries.get(propertyId, m_codeFieldName, variant);
    int treeItemCode = variant.toInt();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, "m_value", value);

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
        emit propertyChanged(projectId, propertyId, treeItemCode, name, value);

        if (!isSilent) {
            emit projectModified(projectId);
        }
    }
    return result;
}

SKRResult SKRPropertyHub::setName(int projectId, int propertyId, const QString& name)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "m_value",       variant);
    QString value = variant.toString();

    queries.get(propertyId, m_codeFieldName, variant);
    int treeItemCode = variant.toInt();

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
        emit propertyChanged(projectId, propertyId, treeItemCode, name, value);

        if (!isSilent) {
            emit projectModified(projectId);
        }
    }
    return result;
}

// ---------------------------------------------------------------------

QString SKRPropertyHub::getName(int projectId, int propertyId)
{
    SKRResult result(this);
    QString   string;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "t_name", out);
    IFKO(result) {
        emit errorSent(result);
    }
    string = out.toString();
    return string;
}

// ---------------------------------------------------------------------

SKRResult SKRPropertyHub::setPaperCode(int projectId, int propertyId, int treeItemCode)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);
    QVariant variant;

    queries.get(propertyId, "m_value", variant);
    QString value = variant.toString();

    queries.get(propertyId, "t_name",  variant);
    QString name = variant.toString();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, m_codeFieldName, treeItemCode);

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
        emit propertyChanged(projectId, propertyId, treeItemCode, name, value);

        if (!isSilent) {
            emit projectModified(projectId);
        }
    }
    return result;
}

// ---------------------------------------------------------------------

int SKRPropertyHub::getPaperCode(int projectId, int propertyId)
{
    SKRResult result(this);
    int value;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, m_codeFieldName, out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toInt();
    return value;
}

SKRResult SKRPropertyHub::setCreationDate(int              projectId,
                                          int              propertyId,
                                          const QDateTime& date)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "m_value", result);
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
        if (!isSilent) {
            emit projectModified(projectId);
        }

        // emit propertyChanged(projectId, propertyId, treeItemCode, name,
        // value);
    }
    return result;
}

// ---------------------------------------------------------------------

QDateTime SKRPropertyHub::getCreationDate(int projectId, int propertyId) const
{
    SKRResult result;
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

SKRResult SKRPropertyHub::setModificationDate(int              projectId,
                                              int              propertyId,
                                              const QDateTime& date)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "m_value", result);
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
        if (!isSilent) {
            emit projectModified(projectId);
        }

        // emit propertyChanged(projectId, propertyId, treeItemCode, name,
        // value);
    }
    return result;
}

// ---------------------------------------------------------------------

QDateTime SKRPropertyHub::getModificationDate(int projectId, int propertyId) const
{
    SKRResult result;
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

SKRResult SKRPropertyHub::setIsSystem(int projectId, int propertyId, bool isSystem)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "m_value", result);
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
        if (!isSilent) {
            emit projectModified(projectId);
        }

        // emit propertyChanged(projectId, propertyId, treeItemCode, name,
        // value);
    }
    return result;
}

// ---------------------------------------------------------------------


bool SKRPropertyHub::getIsSystem(int projectId, int propertyId) const
{
    SKRResult result(this);
    bool value;
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

SKRResult SKRPropertyHub::setIsSilent(int projectId, int propertyId, bool isSilent)
{
    PLMSqlQueries queries(projectId, m_tableName);

    //    QVariant result;
    //    queries.get(propertyId, "m_value", result);
    //    QString value = result.toString();
    //    queries.get(propertyId, "t_name", result);
    //    QString name = result.toString();

    queries.beginTransaction();

    SKRResult result = queries.set(propertyId, "b_silent", isSilent);

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
        if (!isSilent) {
            emit projectModified(projectId);
        }

        // emit propertyChanged(projectId, propertyId, treeItemCode, name,
        // value);
    }
    return result;
}

// ---------------------------------------------------------------------

bool SKRPropertyHub::getIsSilent(int projectId, int propertyId) const
{
    SKRResult result(this);
    bool value;
    QVariant out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "b_silent", out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toBool();
    return value;
}

// ---------------------------------------------------------------------

QString SKRPropertyHub::getProperty(int projectId, int treeItemCode,
                                    const QString& name) const
{
    SKRResult result(this);
    QString   value;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_codeFieldName, treeItemCode);
    where.insert("t_name",        name);
    result = queries.getValueByIdsWhere("m_value", out, where);

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

QString SKRPropertyHub::getProperty(int            projectId,
                                    int            treeItemCode,
                                    const QString& name,
                                    const QString& defaultValue) const
{
    QString value = getProperty(projectId, treeItemCode, name);

    if (value.isNull()) {
        value = defaultValue;
    }
    return value;
}

// ---------------------------------------------------------------------

QString SKRPropertyHub::getPropertyById(int projectId, int propertyId) const
{
    SKRResult result(this);
    QString   value;
    QVariant  out;

    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, "m_value", out);
    IFKO(result) {
        emit errorSent(result);
    }
    value = out.toString();
    return value;
}

// ---------------------------------------------------------------------

int SKRPropertyHub::getPropertyId(int projectId, int treeItemCode, const QString& name) const
{
    SKRResult result(this);
    int value = -2;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_codeFieldName, treeItemCode);
    where.insert("t_name",        name);
    result = queries.getValueByIdsWhere("m_value", out, where);

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

int SKRPropertyHub::getLastAddedId()
{
    return m_last_added_id;
}

// ---------------------------------------------------------------------


SKRResult SKRPropertyHub::addProperty(int projectId, int treeItemCode, int imposedPropertyId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    QHash<QString, QVariant> values;

    values.insert(m_codeFieldName, treeItemCode);

    if (imposedPropertyId != -1) values.insert(queries.getIdName(), imposedPropertyId);
    int newPropertyId = -1;
    SKRResult result  = queries.add(values, newPropertyId);

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

SKRResult SKRPropertyHub::removeProperty(int projectId, int propertyId)
{
    bool isSilent = this->getIsSilent(projectId, propertyId);

    PLMSqlQueries queries(projectId, m_tableName);
    SKRResult     result = queries.remove(propertyId);

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
        emit propertyRemoved(projectId, propertyId);

        if (!isSilent) {
            emit projectModified(projectId);
        }
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

//--------------------------------------------------------------------------

QVariant SKRPropertyHub::get(int projectId, int propertyId, const QString &fieldName) const
{
    SKRResult result(this);
    QVariant  var;
    QVariant  value;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(propertyId, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

//--------------------------------------------------------------------------

bool SKRPropertyHub::propertyExists(int projectId, int treeItemCode, const QString& name)
{
    PLMSqlQueries queries(projectId, m_tableName);


    QHash<QString, QVariant> where;

    where.insert(m_codeFieldName, treeItemCode);
    where.insert("t_name",        name);

    return queries.resultExists(where);
}

int SKRPropertyHub::findPropertyId(int projectId, int treeItemCode, const QString& name)
{
    SKRResult result(this);
    int value = -2;

    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    QHash<QString, QVariant> where;

    where.insert(m_codeFieldName, treeItemCode);
    where.insert("t_name",        name);
    result = queries.getValueByIdsWhere(m_codeFieldName, out, where);

    IFOK(result) {
        value = out.keys().first();
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

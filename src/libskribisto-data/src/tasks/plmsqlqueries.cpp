/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmqueries                                                   *
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
#include "plmsqlqueries.h"
#include "plmprojectmanager.h"
#include <QDebug>
#include <QSqlError>
#include <QSqlDriver>
#include <QSqlRecord>
#include <QSqlField>

PLMSqlQueries::PLMSqlQueries(int            projectId,
                             const QString& tableName) : m_projectId(projectId),
    m_tableName(tableName)
{
    qRegisterMetaType<DBType>("DBType");

    PLMProject *project = plmProjectManager->project(m_projectId);

    if (!project) {
        qDebug() << "PLMSqlQueries: result tableName" << tableName;
        qDebug() << "PLMSqlQueries: result projectId" << m_projectId;
        return;
    }

    m_sqlDB  = project->getSqlDb();
    m_idName = project->getIdNameFromTable(m_tableName);
}

PLMSqlQueries::PLMSqlQueries(QSqlDatabase   sqlDB,
                             const QString& tableName,
                             const QString& idName) :
    m_sqlDB(sqlDB), m_tableName(tableName), m_idName(idName)
{
    qRegisterMetaType<DBType>("DBType");
}

void PLMSqlQueries::beginTransaction()
{
    m_sqlDB.transaction();
}

void PLMSqlQueries::rollback()
{
    m_sqlDB.rollback();
}

void PLMSqlQueries::commit()
{
    m_sqlDB.commit();
}

SKRResult PLMSqlQueries::get(int id, const QString& valueName, QVariant& out) const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "SELECT " + valueName
                             + " FROM " + m_tableName +
                             " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id", id);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);

        while (query.next()) {
            out = query.value(0);
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }
    }

    return result;
}

SKRResult PLMSqlQueries::getMultipleValues(int id, const QStringList& valueList,
                                          QHash<QString, QVariant>& out) const
{
    SKRResult result;

    {
        out.clear();
        QSqlQuery query(m_sqlDB);
        QString   valuesStr;

        for (const QString& value : valueList) {
            valuesStr += value;

            if (valueList.size() > 1) {
                valuesStr += ", ";
            }
        }

        // remove last comma
        if (valueList.size() > 1) {
            valuesStr.chop(2);
        }

        QString queryStr = "SELECT " + valuesStr
                           + " FROM " + m_tableName +
                           " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id", id);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);

        while (query.next()) {
            for(const QString& valueName : valueList) {
                out.insert(valueName, query.value(valueName));
            }
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }

        //        if(result.isEmpty()){
        //            result.setSuccess(false);
        //        }
    }

    return result;
}

SKRResult PLMSqlQueries::getSortedIds(QList<int>& out) const
{
    SKRResult result;

    {
        out.clear();
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "SELECT " + m_idName
                             + " FROM " + m_tableName
                             + " ORDER BY l_sort_order"
        ;
        query.prepare(queryStr);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);

        while (query.next()) {
            out.append(query.value(0).toInt());
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }
    }

    return result;
}

SKRResult PLMSqlQueries::getIds(QList<int>& out) const
{
    SKRResult result;

    {
        out.clear();
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "SELECT " + m_idName
                             + " FROM " + m_tableName
        ;
        query.prepare(queryStr);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);

        while (query.next()) {
            out.append(query.value(0).toInt());
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }
    }

    return result;
}

bool PLMSqlQueries::idExists(int id) const
{
    QSqlQuery query(m_sqlDB);
    QString   queryStr = "SELECT :id FROM " + m_tableName
    ;

    query.prepare(queryStr);
    query.bindValue(":id", id);
    query.exec();

    if (query.size() == 0) {
        return false;
    }
    return true;
}

///
/// \brief PLMSqlQueries::getValueByIds
/// \param valueName
/// \param result
/// \param where
/// \param whereValue
/// \param sorted
/// \return
/// You can use "id" as 'where' ; result QHash isn't garantied to keep the same order ! Use getSortedIds to have the sort order
SKRResult PLMSqlQueries::getValueByIds(const QString& valueName,
                                      QHash<int, QVariant>& out,
                                      const QString& where,
                                      const QVariant& whereValue,
                                      bool sorted) const
{
    out.clear();
    SKRResult result;

    {
        out.clear();
        QSqlQuery query(m_sqlDB);
        QString   whereStr;
        QString   wh = where;

        if (!wh.isEmpty()) {
            if (wh == "id") {
                wh = m_idName;
            }

            QRegExp rx("[><=]^");

            if (!wh.contains(rx)) {
                wh.append(" =");
            }

            whereStr = " WHERE " + wh + " :whereValue";
        }

        QString sortedStr;
        sorted ? sortedStr = " ORDER BY l_sort_order" : sortedStr = "";
        QString queryStr = "SELECT " + m_idName + " , " + valueName
                           + " FROM " + m_tableName + whereStr + sortedStr;
        query.prepare(queryStr);

        if (!where.isEmpty()) {
            query.bindValue(":whereValue", whereValue);
        }

        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }

        while (query.next()) {
            out.insert(query.value(0).toInt(), query.value(1));
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }
    }

    return result;
}

SKRResult PLMSqlQueries::getValueByIdsWhere(const QString& valueName,
                                           QHash<int, QVariant>& out,
                                           const QHash<QString, QVariant>& where,
                                           bool sorted) const
{
    SKRResult result;

    {
        out.clear();
        QSqlQuery query(m_sqlDB);
        QHash<QString, QVariant> finalWhereHash;
        QString whereStr                           = " WHERE ";
        QHash<QString, QVariant>::const_iterator i = where.constBegin();

        while (i != where.constEnd()) {
            QString wh = i.key();

            if (wh == "id") {
                wh = m_idName;
            }

            QRegExp rx("\\s*[><=]{1,2}$");
            QString valueWh = wh;

            if (wh.contains(rx)) {
                valueWh.remove(rx);
                finalWhereHash.insert(valueWh, i.value());
            } else {
                finalWhereHash.insert(wh, i.value());
                wh.append(" =");
            }

            whereStr += wh + " :" + valueWh + " AND ";
            ++i;
        }

        // remove final AND
        whereStr.chop(5);
        QString sortedStr;
        sorted ? sortedStr = " ORDER BY l_sort_order" : sortedStr = "";
        QString queryStr = "SELECT " + m_idName + " , " + valueName
                           + " FROM " + m_tableName + whereStr + sortedStr;
        query.prepare(queryStr);
        i = finalWhereHash.constBegin();

        while (i != finalWhereHash.constEnd()) {
            query.bindValue(":" + i.key(), i.value());
            ++i;
        }

        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }

        while (query.next()) {
            out.insert(query.value(0).toInt(), query.value(1));
        }

        if (query.size() == 0) {
            result.setSuccess(false);
        }
    }

    return result;
}

bool PLMSqlQueries::resultExists(const QHash<QString, QVariant>& where) const
{
    SKRResult result;
    bool     value;

    {
        QSqlQuery query(m_sqlDB);
        QString   whereStr                         = " WHERE ";
        QHash<QString, QVariant>::const_iterator i = where.constBegin();

        while (i != where.constEnd()) {
            whereStr += i.key() + " = :" + i.key() + " AND ";
            ++i;
        }

        // remove final AND
        whereStr.chop(5);

        if (where.isEmpty()) {
            whereStr = "";
        }

        QString queryStr = "SELECT " + m_idName
                           + " FROM " + m_tableName + whereStr;
        query.prepare(queryStr);
        i = where.constBegin();

        while (i != where.constEnd()) {
            query.bindValue(":" + i.key(), i.value());
            ++i;
        }

        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }

        QHash<int, QVariant> out;

        while (query.next()) {
            out.insert(query.value(0).toInt(), query.value(1));
        }

        out.size() > 0 ? value = true : value = false;
    }

    return result;
}

SKRResult PLMSqlQueries::add(const QHash<QString, QVariant>& values, int& newId) const
{
    SKRResult result;

    {
        QSqlQuery   query(m_sqlDB);
        QStringList valueNamesStrList;
        QString     valueNamesStr                  = "(";
        QString     valuesStr                      = " VALUES(:";
        QHash<QString, QVariant>::const_iterator i = values.constBegin();

        while (i != values.constEnd()) {
            valueNamesStrList.append(i.key());
            ++i;
        }

        valueNamesStr.append(valueNamesStrList.join(", ") + ")");
        valuesStr.append(valueNamesStrList.join(", :") + ")");

        if (values.isEmpty()) {
            valuesStr     = "";
            valueNamesStr = " DEFAULT VALUES";
        }

        QString queryStr = "INSERT INTO " + m_tableName + " "
                           + valueNamesStr + valuesStr;
        query.prepare(queryStr);
        i = values.constBegin();

        while (i != values.constEnd()) {
            query.bindValue(":" + i.key(), i.value());
            ++i;
        }

        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
        newId = query.lastInsertId().toInt();
    }

    return result;
}

SKRResult PLMSqlQueries::removeAll() const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "DELETE * FROM " + m_tableName;
        query.prepare(queryStr);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::remove(int id) const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "DELETE FROM " + m_tableName
                             + " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id", id);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::set(int id, const QString& valueName, const QVariant& value) const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "UPDATE " + m_tableName
                             + " SET " + valueName + " = :value"
                                                     " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id",    id);
        query.bindValue(":value", value);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::setId(int id, int newId) const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "UPDATE " + m_tableName
                             + " SET " + m_idName + " = :value"
                                                    " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id",    id);
        query.bindValue(":value", newId);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::injectDirectSql(const QString& sqlString)
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = sqlString;
        query.prepare(queryStr);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::setCurrentDate(int id, const QString& valueName) const
{
    SKRResult result;

    {
        QSqlQuery query(m_sqlDB);
        QString   queryStr = "UPDATE " + m_tableName
                             + " SET " + valueName + " = CURRENT_TIMESTAMP"
                                                     " WHERE " + m_idName + " = :id"
        ;
        query.prepare(queryStr);
        query.bindValue(":id", id);
        query.exec() ? result.setSuccess(true) : result.setSuccess(false);
        if(query.lastError().isValid()){
            qDebug() << "SQL Error" << query.lastError();
            qDebug() << "SQL Error" << query.lastError().text();
        }
    }

    return result;
}

SKRResult PLMSqlQueries::renumberSortOrder()
{
    SKRResult result;
    int renumInterval = 1000;

    // Renumber all non-trashed paper in this version. DOES NOT COMMIT - Caller
    // should
    QSqlQuery query(m_sqlDB);
    QString   queryStr = "SELECT " + m_idName
                         + " FROM " + m_tableName
                         + " ORDER BY l_sort_order"
    ;

    /*bool prepareOk = */ query.prepare(queryStr);

    //    qDebug() << "prepareOk" << prepareOk;
    //    qDebug() << query.lastError().text();
    query.exec() ? result.setSuccess(true) : result.setSuccess(false);
    if(query.lastError().isValid()){
        qDebug() << "SQL Error" << query.lastError();
        qDebug() << "SQL Error" << query.lastError().text();
    }

    //    qDebug() << query.lastError().text();

    int dest = renumInterval;
    QList<int> list;

    while (query.next()) {
        list.append(query.value(0).toInt());
    }

    for (int& id : list) {
        // For each note to renumber, pass it to the renum function.. For speed
        // we commit after all rows renumbered
        IFOKDO(result, set(id, "l_sort_order", dest));
        dest += renumInterval;
    }

    return result;
}

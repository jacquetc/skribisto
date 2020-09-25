/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmdbpaper.cpp                                                   *
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
#include "plmdbpaper.h"
#include <QtSql/QSqlError>
#include <QDebug>
#include <QDateTime>

PLMDbPaper::PLMDbPaper(const QSqlDatabase& sqlDb,
                       const QString     & tableName,
                       const QString     & idName,
                       int                 paperId,
                       bool                commit)
{
    m_sqlDb     = sqlDb;
    m_paperId   = paperId;
    m_commit    = commit;
    m_tableName = tableName;
    m_idName    = idName;
}

void PLMDbPaper::setCommit(bool commit)
{
    m_commit = commit;
}

bool PLMDbPaper::getCommit()
{
    return m_commit;
}

bool PLMDbPaper::exists()
{
    QVariant value = get(m_idName);

    if (value.isNull()) return false;

    return true;
}

int PLMDbPaper::copy(const QString& prefix)
{
    Q_UNUSED(prefix);
    return 0;
}

void PLMDbPaper::commit()
{
    m_sqlDb.commit();
}

QVariant PLMDbPaper::get(const QString& valueName) const
{
    QSqlQuery query(m_sqlDb);
    QString   queryStr = "SELECT " + valueName
                         + " FROM " + m_tableName
                         + " WHERE " + m_idName + " = :paperId"
    ;

    query.prepare(queryStr);
    query.bindValue(":paperId", m_paperId);
    query.exec();

    while (query.next()) {
        return query.value(0);
    }
    return QVariant();
}

void PLMDbPaper::set(const QString & valueName,
                     const QVariant& value,
                     bool            updatedDateStampSet)
{
    if (m_commit) m_sqlDb.transaction();

    QString updatedDateStamp = " dt_updated = CURRENT_TIMESTAMP";

    if (!updatedDateStampSet) updatedDateStamp = "";

    QSqlQuery query(m_sqlDb);
    QString   queryStr = "UPDATE " + m_tableName
                         + " SET " + valueName + " = :value, "
                         + updatedDateStamp +
                         " WHERE " + m_idName + " = :paperId"
    ;

    query.prepare(queryStr);
    query.bindValue(":paperId", m_paperId);
    query.bindValue(":value",   value);
    query.exec();

    //    qDebug() << "set : " << getLastExecutedQuery(query);
    //    qDebug() << m_sqlDb.lastError();

    if (m_commit) m_sqlDb.commit();
}

int PLMDbPaper::getSortOrder()
{
    return get("l_sort_order").toInt();
}

void PLMDbPaper::setSortOrder(int value, bool updatedDateStampSet)
{
    set("l_sort_order", value, updatedDateStampSet);
}

int PLMDbPaper::getIndent()
{
    return get("l_indent").toInt();
}

void PLMDbPaper::setIndent(int value)
{
    set("l_indent", value);
}

QVariant PLMDbPaper::getContent() const
{
    return get("m_content");
}

void PLMDbPaper::setContent(const QVariant& value)
{
    set("m_content", value);
}

QString PLMDbPaper::getTitle() const
{
    return get("t_title").toString();
}

void PLMDbPaper::setTitle(const QString& value)
{
    set("t_title", value);
}

bool PLMDbPaper::getDelete() const
{
    return get("b_deleted").toBool();
}

void PLMDbPaper::setDelete(bool value)
{
    set("b_deleted", value);
}

QDateTime PLMDbPaper::getCreationDate() const
{
    return get("dt_created").toDateTime();
}

void PLMDbPaper::setCreationDate(const QDateTime& value)
{
    set("dt_created", value);
}

QDateTime PLMDbPaper::getUpdateDate() const
{
    return get("dt_updated").toDateTime();
}

void PLMDbPaper::setUpdateDate(const QDateTime& value)
{
    set("dt_updated", value);
}

QDateTime PLMDbPaper::getContentDate() const
{
    return get("dt_content").toDateTime();
}

void PLMDbPaper::setContentDate(const QDateTime& value)
{
    set("dt_content", value);
}

int PLMDbPaper::add()
{
    if (m_commit) m_sqlDb.transaction();

    QSqlQuery query(m_sqlDb);
    QString   queryStr = "INSERT INTO " + m_tableName
                         + " ( "
                         + "t_title,"
                         + "dt_created,"
                         + "dt_updated,"
                         + "dt_content,"
                         + "l_version,"
                         + "b_deleted "
                         + " ) "
                         + " VALUES ( "
                         + ":title,"
                         + "CURRENT_TIMESTAMP,"
                         + "CURRENT_TIMESTAMP,"
                         + "CURRENT_TIMESTAMP,"
                         + "0,"
                         + "0"
                           " ) "
    ;

    query.prepare(queryStr);
    query.bindValue(":title", "new");
    query.exec();

    //    qDqsqlerror m_sqlDb.lastError();
    int lastInserted = query.lastInsertId().toInt();

    if (m_commit) m_sqlDb.commit();
    return lastInserted;
}

QList<int>PLMDbPaper::childIdList()
{
    QSqlQuery query(m_sqlDb);
    QString   queryStr = "SELECT "
                         "l_indent, l_sort_order, b_deleted"
                         " FROM " + m_tableName
                         + " WHERE " + m_idName + " = " + QString::number(m_paperId)
                         + " ORDER BY l_sort_order"
    ;

    query.exec(queryStr);
    int  parentIndent    = -1;
    int  parentSortOrder = -1;
    bool parentDeleted   = false;

    while (query.next()) {
        parentIndent    = query.value(0).toInt();
        parentSortOrder = query.value(1).toInt();
        parentDeleted   = query.value(3).toBool();
    }

    queryStr = "SELECT "
               + m_idName + ", l_indent "
               + " FROM " + m_tableName
               + " WHERE "
               + "l_sort_order > :parent_sort"
               + " AND b_deleted = :parent_deleted"
               + " ORDER BY l_sort_order"
    ;
    query.prepare(queryStr);
    query.bindValue(":parent_sort",    parentSortOrder);
    query.bindValue(":parent_deleted", parentDeleted);
    query.exec();

    //      qDebug() << " : " << getLastExecutedQuery(query);

    QList<int> finalList;

    while (query.next()) {
        if (query.value("l_indent").toInt() <= parentIndent) break;
        finalList.append(query.value(m_idName).toInt());
    }


    return finalList;
}

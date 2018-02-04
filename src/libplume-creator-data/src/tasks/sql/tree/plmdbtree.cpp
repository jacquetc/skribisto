/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmdbtree.cpp                                                   *
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
#include "plmdbtree.h"
#include "plmdbpaper.h"
#include <QtSql/QSqlError>
#include <QDebug>

/**
 * @brief PLMDbTree::PLMDbTree
 * @param sqlDb
 * @param tableName
 * @param idName
 * @param commit
 */
PLMDbTree::PLMDbTree(const QSqlDatabase &sqlDb, const QString &tableName, const QString &idName, bool commit)
{
    m_sqlDb = sqlDb;
    m_commit = commit;
    m_tableName = tableName;
    m_idName = idName;
    m_version = 0;
    m_error = PLMDbError();
    m_renumInterval = 1000;
}

/**
 * @brief PLMDbTree::setCommit
 * @param value
 */
void PLMDbTree::setCommit(bool value)
{
    m_commit = value;
}

/**
 * @brief PLMDbTree::getCommit
 * @return
 */
bool PLMDbTree::getCommit()
{
    return m_commit;
}

/**
 * @brief PLMDbTree::renumAll
 * Will not update the update date stamp
 */
void PLMDbTree::renumAll()
{


    //Renumber all non-deleted paper in this version. DOES NOT COMMIT - Caller should
    QSqlQuery query(m_sqlDb);
    QString queryStr = "SELECT "+ m_idName
            + " FROM " + m_tableName
            + " WHERE"
            //            + " b_deleted = 0"
            //            + " and"
            + " l_version = :ver"
            + " ORDER BY l_sort_order"
            ;
    query.prepare(queryStr);
    query.bindValue(":ver", m_version);
    query.exec();
    //            qDebug() << getLastExecutedQuery(query);

    //   if(!query.isValid())
    //       qDebug() << "renumAll() : " << m_sqlDb.lastError();


    if(m_commit)
        m_sqlDb.transaction();
    int dest = m_renumInterval;
    while (query.next()) {
        //For each note to renumber, pass it to the renum function.. For speed we commit after all rows renumbered

        PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, query.value(m_idName).toInt(), false);
        paper.setSortOrder(dest, false);
        dest += m_renumInterval;
    }

    if(m_commit)
        m_sqlDb.commit();
}

/**
 * @brief PLMDbTree::moveList
 * @param idList
 * @param paperId
 * @return
 */
int PLMDbTree::moveList(QList<int> idList, int paperId)
//
// Move the list of notes to after i_after_note.
//
// If i_after_note is 0, then move the notes to the top. -1 = move notes to bottom
//
// The notes are moved in list order such at the first in the list ends up as the first after i_after_note.
//
// To move, we renumber notes from i_dest + 1. The max notes we can move is RENUM_INT -1.
//
{
    if(m_commit)
        m_sqlDb.transaction();


    if(idList.count() >= m_renumInterval){
        m_error.setStatus(PLMDbError::InternalError, 0, QString("Can't move more than %1 notes").arg(QString::number(m_renumInterval - 1)));
        return PLMDbError::Error;
    }
    int dest;
    if(paperId == 0)
        dest = 0;
    else if(paperId == -1)
        dest = INT_MAX - 1000;
    else {
        PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, paperId, false);
        dest = paper.getSortOrder();
        if(dest == PLMDbError::Error){
            m_error.setStatus(PLMDbError::InvalidParameter, 2, QString("After Item %1 not found").arg(QString::number(paperId)));
            return PLMDbError::Error;
        }
    }

    // Renumber each sheet to +1 after dest
    dest += 1;
    foreach(int id, idList){
        PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, id, false);
        if(!paper.exists()){
            m_sqlDb.rollback();
            m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet %1 not found").arg("%1", QString::number(id)));
            return PLMDbError::Error;
        }

        paper.setSortOrder(dest);
        dest += 1;

    }
    // Renumber all sheets to restore default spacing

    //??? renumAll()
    if(m_commit)
        m_sqlDb.commit();

    return PLMDbError::OK;
}

/**
 * @brief PLMDbTree::copyList
 * @param idList
 * @return
 */
int PLMDbTree::copyList(QList<int> idList)
//
// Copy the list of sheets. Each sheet is copied directly after its original sheet.
//
{
    if(m_commit)
        m_sqlDb.transaction();

    foreach(int id, idList){
        PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, id, false);
        if(!paper.exists()){
            m_sqlDb.rollback();
            m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet (%1) not found").arg(QString::number(id)));
            return PLMDbError::Error;
        }
        paper.copy("Copy of ");
    }
    // Renumber all sheets to restore default spacing

    //??? renumAll()
    if(m_commit)
        m_sqlDb.commit();

    return PLMDbError::OK;
}
/**
 * @brief PLMDbTree::deleteList
 * @param idList
 * @return
 */
/**
 * @brief PLMDbTree::deleteList
 * @param idList
 * @return
 */
///
/// \brief PLMDbTree::deleteList
/// \param idList
/// \return
///
int PLMDbTree::deleteList(QList<int> idList)
//
// Mark a list of sheets as deleted.
// This function just sets the deleted flag in the record. Call empty_trash to really delete the records
//
{
    if(m_commit)
        m_sqlDb.transaction();

    foreach(int id, idList){
        PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, id, false);
        if(!paper.exists()){
            m_sqlDb.rollback();
            m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet (%1) not found").arg(QString::number(id)));
            return PLMDbError::Error;
        }
        paper.setDelete(true);
    }
    // Renumber all sheets to restore default spacing

    //??? renumAll()
    if(m_commit)
        m_sqlDb.commit();

    return PLMDbError::OK;
}

int PLMDbTree::undeleteList(QList<int> idList)
{
    if(m_commit)
        m_sqlDb.transaction();

    foreach(int id, idList){
        PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, id, false);
        if(!paper.exists()){
            m_sqlDb.rollback();
            m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet (%1) not found").arg(QString::number(id)));
            return PLMDbError::Error;
        }
        paper.setDelete(false);
    }
    // Renumber all sheets to restore default spacing

    //??? renumAll()
    if(m_commit)
        m_sqlDb.commit();

    return PLMDbError::OK;
}

int PLMDbTree::emptyTrash()
//
// Permanently remove any deleted sheets for the CURRENT version
//
{
    if(m_commit)
        m_sqlDb.transaction();

    QSqlQuery query(m_sqlDb);
    QString queryStr = "DELETE FROM "
            + m_tableName
            + " WHERE b_deleted = 1"
            + "and l_version = :ver"
            ;
    query.prepare(queryStr);
    query.bindValue(":ver", m_version);
    query.exec();

    return PLMDbError::OK;

}

QList<int> PLMDbTree::listVisibleId()
{
    QSqlQuery query(m_sqlDb);
    QString queryStr = "SELECT "+ m_idName
            + " FROM " + m_tableName
            + " WHERE b_deleted = 0"
            + "and l_version = :ver"
            + " ORDER BY l_sort_order"
            ;
    query.prepare(queryStr);
    query.bindValue(":ver", m_version);
    query.exec();

    QList<int> list;
    while (query.next()) {

        list.append(query.value(m_idName).toInt());
    }

    return list;
}

QList<int> PLMDbTree::listTrash()
//
//  Returns a list of deleted sheet ids for the current version
//
{
    QSqlQuery query(m_sqlDb);
    QString queryStr = "SELECT "+ m_idName
            + " FROM " + m_tableName
            + " WHERE b_deleted = 1"
            + "and l_version = :ver"
            + " ORDER BY l_sort_order"
            ;
    query.prepare(queryStr);
    query.bindValue(":ver", m_version);
    query.exec();

    QList<int> list;
    while (query.next()) {

        list.append(query.value(m_idName).toInt());
    }

    return list;
}

QList<int> PLMDbTree::listAllIds()
//
//  Returns a list of ALL sheet ids. Probably only useful for testing?
//
{
    QSqlQuery query(m_sqlDb);
    QString queryStr = "SELECT "+ m_idName
            + " FROM " + m_tableName
            + " ORDER BY l_sort_order"
            ;
    query.prepare(queryStr);
    query.exec();

    QList<int> list;
    while (query.next()) {

        list.append(query.value(m_idName).toInt());
    }

    return list;
}

int PLMDbTree::getPaperAbove(int paperId)
{
    PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, paperId, false);
    if(!paper.exists()){
        m_sqlDb.rollback();
        m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet (%1) not found").arg(QString::number(paperId)));
        return PLMDbError::Error;
    }
    if(paper.getSortOrder() == m_renumInterval) // first one
        return 0;

    return listVisibleId().at(listVisibleId().indexOf(paperId) - 1);

}

int PLMDbTree::getPaperBelow(int paperId)
{
    PLMDbPaper paper(m_sqlDb,m_tableName, m_idName, paperId, false);
    if(!paper.exists()){
        m_sqlDb.rollback();
        m_error.setStatus(PLMDbError::InvalidParameter, 1, QString("Sheet (%1) not found").arg(QString::number(paperId)));
        return PLMDbError::Error;
    }
    if(paper.getSortOrder() == listAllIds().last())
        return -1;

    return listVisibleId().at(listVisibleId().indexOf(paperId) + 1);

}

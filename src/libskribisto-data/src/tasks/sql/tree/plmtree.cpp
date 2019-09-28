/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmtree.cpp                                                   *
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

#include "plmtree.h"
#include "plmdbtree.h"
#include <QStringList>
#include <QtSql/QSqlQuery>
#include <QSqlError>
#include <QSqlDriver>
#include <QSqlRecord>
#include <QSqlField>
#include <QDebug>
#include <QDateTime>

PLMTree::PLMTree(QObject *parent, const QString &tableName, const QString &idName, QSqlDatabase sqlDb) :
    QObject(parent)
{

    m_tableName = tableName;
    m_idName = idName;
    m_sqlDb = sqlDb;

    PLMDbTree tree(m_sqlDb, m_tableName, m_idName, true);
    tree.renumAll();

}

QList<QHash<QString, QVariant> > PLMTree::getAll()
{
    if(!m_sqlDb.isOpen())
        m_sqlDb.open();

    QSqlQuery query(m_sqlDb);
    QStringList names;
    names.append(getAllHeaders());

    //            << "t_title"
    //          << m_idName
    //          << "l_sort_order"
    //          << "l_indent"
    //          << "m_content"
    //          << "l_version_code"
    //          << "l_dna_code"
    //          << "t_synopsis"
    //          << "l_char_count"
    //          << "l_word_count"
    //          << "dt_created"
    //          << "dt_updated"
    //          << "dt_content"
    //          << "b_deleted"
    //             ;
    QString queryStr = "SELECT "+ names.join(", ")
            + " FROM " + m_tableName
            + " ORDER BY l_sort_order"
            ;
    query.exec(queryStr);
    QList<QHash<QString, QVariant> > list;
    while (query.next()) {
        QHash<QString, QVariant> hash;

        for(int i = 0 ; i < names.count(); ++i) {
            QString key = names.at(i);
            hash.insert(key, query.value(i));
        }

        list.append(hash);
    }
    return list;
}

QStringList PLMTree::getAllHeaders()
{
    if(!m_sqlDb.isOpen())
        m_sqlDb.open();

    QSqlRecord record =  m_sqlDb.driver()->record(m_tableName);
    QStringList list;
    for(int i = 0; i < record.count() ; ++i)
        list << record.field(i).name();
    return list;
}

QHash<int, QVariant> PLMTree::getAllValues(const QString &header)
{
    if(!m_sqlDb.isOpen())
        m_sqlDb.open();

    QSqlQuery query(m_sqlDb);
    QStringList names;
    names << m_idName << header;

    QString queryStr = "SELECT "+ names.join(", ")
            + " FROM " + m_tableName
            + " ORDER BY l_sort_order"
            ;
    query.exec(queryStr);
    QHash<int, QVariant> hash;
    int idField = query.record().indexOf(m_idName);
    int valueField = query.record().indexOf(header);
    while (query.next()) {
        int id = query.value(idField).toInt();
        QVariant value = query.value(valueField);
        hash.insert(id, value);
    }

    return hash;

}

QString PLMTree::tableName() const
{
    return m_tableName;
}

QString PLMTree::idName() const
{
    return m_idName;
}

QSqlDatabase PLMTree::sqlDb() const
{
    return m_sqlDb;
}

QList<int>  PLMTree::addNewChildPapers(int parentId, int number)
{

    return addNewChildPapers(parentId, number, true);

}

QList<int>  PLMTree::addNewChildPapers(int parentId, int number, bool commit)
{
    if(commit)
        m_sqlDb.transaction();

    QList <int> list;
    for(int i = 0 ; i < number ; ++i){
        PLMDbPaper paper(sqlDb(), tableName(), idName(), -1, false);
        list.append(paper.add());
        movePapersAsChildOf(list, parentId, false);

    }
    if(commit)
        m_sqlDb.commit();


    return list;

}

QList<int> PLMTree::addNewPapersBy(int paperId, int number)
{
    return addNewPapersBy(paperId, number, true);
}

QList<int> PLMTree::addNewPapersBy(int paperId, int number, bool commit)
{
    if(commit)
        m_sqlDb.transaction();

    QList <int> list;
    for(int i = 0 ; i < number ; ++i){
        PLMDbPaper paper(sqlDb(), tableName(), idName(), -1, false);
        list.append(paper.add());
        movePapersBelow(list, paperId, false);
    }
    if(commit)
        m_sqlDb.commit();

    return list;
}

void PLMTree::movePapersAsChildOf(QList<int> paperIdList, int destId)
{
    movePapersAsChildOf(paperIdList, destId, true);
}

void PLMTree::movePapersAsChildOf(QList<int> paperIdList, int destId, bool commit)
{
    if(commit)
        m_sqlDb.transaction();

    PLMDbPaper paper(sqlDb(), tableName(), idName(), destId, false);
    PLMDbTree tree(m_sqlDb, m_tableName, m_idName, false);
    QList<int> childIdList = paper.childIdList();
    int destIndent = paper.getIndent();
    //    qDebug() << "idList : " << m_sqlDb.lastError();

    if(childIdList.isEmpty()){
        tree.moveList(paperIdList, destId);


        foreach (int paperId, paperIdList) {
            PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
            paper.setIndent(destIndent + 1);
        }
    }
    else if (childIdList.count() > 0){

        movePapersBelow(paperIdList, childIdList.last(), false);
        foreach (int paperId, paperIdList) {
            PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
            paper.setIndent(destIndent + 1);
        }

    }
    else
        movePapersBelow(paperIdList, childIdList.last(), false);

    tree.renumAll();
    //    qDebug() << "movePaperAsChildOf : " << m_sqlDb.lastError();
    if(commit)
        m_sqlDb.commit();

}
void PLMTree::movePapersAbove(QList<int> paperIdList, int destId)
{
    movePapersAbove(paperIdList, destId, true);
}

void PLMTree::movePapersAbove(QList<int> paperIdList, int destId, bool commit)
{
    if(commit)
        m_sqlDb.transaction();
    PLMDbTree tree(m_sqlDb, m_tableName, m_idName, false);
    int paperAbove = tree.getPaperAbove(destId);
    tree.moveList(paperIdList, paperAbove);

    PLMDbPaper paper(sqlDb(), tableName(), idName(), destId, false);
    int destIndent = paper.getIndent();
    foreach (int paperId, paperIdList) {
        PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
        paper.setIndent(destIndent);
    }
    tree.renumAll();

    if(commit)
        m_sqlDb.commit();
}
void PLMTree::movePapersBelow(QList<int> paperIdList, int destId)
{
    movePapersBelow(paperIdList, destId, true);
}

void PLMTree::movePapersBelow(QList<int> paperIdList, int destId, bool commit)
{
    if(commit)
        m_sqlDb.transaction();

    PLMDbTree tree(m_sqlDb, m_tableName, m_idName, false);
    tree.moveList(paperIdList, destId);

    PLMDbPaper paper(sqlDb(), tableName(), idName(), destId, false);

    int destIndent = paper.getIndent();
    foreach (int paperId, paperIdList) {
        PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
        paper.setIndent(destIndent);
    }
    tree.renumAll();

    if(commit)
        m_sqlDb.commit();
}

QVariant PLMTree::getContent(int paperId) const
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getContent();
}

void PLMTree::setContent(int paperId, const QVariant &value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setContent(value);
}
QString PLMTree::getTitle(int paperId) const
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getTitle();
}

void PLMTree::setTitle(int paperId, const QString &value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setTitle(value);
}

bool PLMTree::getDeleted(int paperId)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getDelete();
}

void PLMTree::setDeleted(int paperId, bool value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setDelete(value);
}

QList<int> PLMTree::getAllIds()
{
    PLMDbTree tree(m_sqlDb, m_tableName, m_idName, false);
    return tree.listAllIds();

}

QDateTime PLMTree::getCreationDate(int paperId) const
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getCreationDate();
}

void PLMTree::setCreationDate(int paperId, const QDateTime &value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setCreationDate(value);
}

QDateTime PLMTree::getUpdateDate(int paperId) const
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getUpdateDate();
}

void PLMTree::setUpdateDate(int paperId, const QDateTime &value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setUpdateDate(value);
}

QDateTime PLMTree::getContentDate(int paperId) const
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, false);
    return paper.getContentDate();
}

void PLMTree::setContentDate(int paperId, const QDateTime &value)
{
    PLMDbPaper paper(m_sqlDb, m_tableName, m_idName, paperId, true);
    paper.setContentDate(value);
}


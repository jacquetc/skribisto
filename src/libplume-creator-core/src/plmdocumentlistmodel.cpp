/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdocumentslistmodel.cpp
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
#include "plmdocumentlistmodel.h"
#include "plmdata.h"
#include <QDebug>

PLMDocumentListModel::PLMDocumentListModel(QObject *parent, const QString &tableName)
    : QAbstractListModel(parent), m_tableName(tableName)
{
    qRegisterMetaType<QList<PLMDocumentListItem> >("QList<PLMSheetListItem>");

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMDocumentListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMDocumentListModel::populate);

    connect(plmdata->userHub(),
            &PLMUserHub::userDataAdded,
//            this, &PLMDocumentListModel::populate
            this, &PLMDocumentListModel::addDocument, Qt::DirectConnection
            );

    connect(plmdata->userHub(),
            &PLMUserHub::userDataRemoved,
//            this, &PLMDocumentListModel::populate
            this, &PLMDocumentListModel::removeDocument, Qt::DirectConnection
            );

    connect(plmdata->userHub(),
            &PLMUserHub::userDataModified,
//            this, &PLMDocumentListModel::populate
            this, &PLMDocumentListModel::modifyDocument, Qt::DirectConnection
            );


}

QVariant PLMDocumentListModel::headerData(int             section,
                                          Qt::Orientation orientation,
                                          int             role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)


    return m_headerData;
}

bool PLMDocumentListModel::setHeaderData(int             section,
                                         Qt::Orientation orientation,
                                         const QVariant& value,
                                         int             role)
{
    if (value != headerData(section, orientation, role)) {
        m_headerData = value;
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

int PLMDocumentListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;
    return m_allDocuments.count();
}


QVariant PLMDocumentListModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    int row = index.row();
    int col = index.column();


    int projectId     = m_allDocuments.at(row).projectId;
    int documentId    = m_allDocuments.at(row).documentId;
    QString tableName = m_allDocuments.at(row).tableName;

    if (role == Qt::DisplayRole  && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "t_title");
    }


    if (role == PLMDocumentListModel::ProjectIdRole  && (col == 0)) {
        return projectId;
    }

    if (role == PLMDocumentListModel::DocumentIdRole && (col == 0)) {
        return documentId;
    }

    if (role == PLMDocumentListModel::PaperCodeRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "l_paper_code");
    }

    if (role == PLMDocumentListModel::NameRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "t_title");
    }

    if (role == PLMDocumentListModel::TypeRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "t_type");
    }

    if (role == PLMDocumentListModel::SubWindowRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "l_subwindow");
    }

    if (role == PLMDocumentListModel::CursorPosRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "l_cursor_pos");
    }

    if (role == PLMDocumentListModel::PropertyRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "t_property");
    }


    if (role == PLMDocumentListModel::UpdateDateRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId, "dt_updated");
    }


    if (role == PLMDocumentListModel::LasFocusedDateRole && (col == 0)) {
        return plmdata->userHub()->get(projectId, tableName, documentId,
                                       "dt_last_focused");
    }


    return QVariant();
}

bool PLMDocumentListModel::setData(const QModelIndex& index,
                                   const QVariant   & value,
                                   int                role)
{
    if (data(index, role) != value) {
        // FIXME: Implement me!
        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}::

Qt::ItemFlags PLMDocumentListModel::flags(const QModelIndex& index) const
{
    if (!index.isValid()) return Qt::NoItemFlags;

    return Qt::NoItemFlags; // FIXME: Implement me!
}

void PLMDocumentListModel::addDocument(int            projectId,
                                       const QString& tableName,
                                       int            documentId)
{
    if (!tableName.endsWith("doc_list")) return;

    //QModelIndex parentIndex = this->index(0, 0, QModelIndex());

    beginInsertRows(QModelIndex(), m_allDocuments.count(), m_allDocuments.count() + 1);

    m_allDocuments.append(PLMDocumentListItem(projectId, documentId, tableName));

    endInsertRows();

}

//------------------------------------------------------

void PLMDocumentListModel::removeDocument(int projectId, const QString &tableName, int documentId)
{
    if (!tableName.endsWith("doc_list")) return;


    //QModelIndex parentIndex = this->index(0, 0, QModelIndex());


    beginRemoveRows(QModelIndex(), m_allDocuments.count(), m_allDocuments.count());


    QMutableListIterator<PLMDocumentListItem> i(m_allDocuments);
    while(i.hasNext()){
        i.next();
        if(i.value().projectId == projectId && i.value().tableName == tableName && i.value().documentId == documentId){
            i.remove();
        }
    }
    endRemoveRows();

}

void PLMDocumentListModel::modifyDocument(int projectId, const QString &tableName, int documentId, const QString &fieldName)
{
    if (!tableName.endsWith("doc_list")) return;



    QList<int> list;

QModelIndexList finalIndexList;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMDocumentListModel::Roles::DocumentIdRole,
                                             documentId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : modelList) {
        if (modelIndex.data(PLMDocumentListModel::Roles::ProjectIdRole).toInt() ==
                projectId)  {
            finalIndexList.append(modelIndex);
        }
    }

    QModelIndex index = finalIndexList.first();
    QVector<int> role;
    if(fieldName == "t_title"){
        role << PLMDocumentListModel::Roles::NameRole;
    }

    emit dataChanged(index, index, role);

}

//------------------------------------------------------

QHash<int, QByteArray>PLMDocumentListModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[ProjectIdRole]      = "projectId";
    roles[DocumentIdRole]     = "documentId";
    roles[PaperCodeRole]      = "paperCode";
    roles[NameRole]           = "name";
    roles[TypeRole]           = "type";
    roles[SubWindowRole]      = "subWindow";
    roles[CursorPosRole]      = "cursorPos";
    roles[PropertyRole]       = "property";
    roles[UpdateDateRole]     = "updateDate";
    roles[LasFocusedDateRole] = "lastFocusedDate";
    return roles;
}

QList<int>PLMDocumentListModel::getSubWindowIdList(int projectId, int paperId)
{
    QList<int> list;


    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMDocumentListModel::Roles::PaperCodeRole,
                                             paperId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : modelList) {
        if (modelIndex.data(PLMDocumentListModel::Roles::ProjectIdRole).toInt() ==
                projectId) {
            list.append(modelIndex.data(
                            PLMDocumentListModel::Roles::SubWindowRole).toInt());
        }
    }

    return list;
}

QList<int>PLMDocumentListModel::getDocumentId(int projectId, int paperId, int subWindowId)
{
    QList<int> list;


    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMDocumentListModel::Roles::SubWindowRole,
                                             subWindowId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : modelList) {
        if ((modelIndex.data(PLMDocumentListModel::Roles::ProjectIdRole).toInt() ==
             projectId) &&
                (modelIndex.data(PLMDocumentListModel::Roles::PaperCodeRole).toInt() ==
                 paperId)) {
            list.append(modelIndex.data(
                            PLMDocumentListModel::Roles::DocumentIdRole).toInt());
        }
    }

    return list;
}

QList<int> PLMDocumentListModel::getDocumentIdEverywhere(int projectId, int paperId)
{
    QList<int> result;

    for(int subWindowId : this->getSubWindowIdList(projectId, paperId)){
        result.append(getDocumentId(projectId, paperId, subWindowId));
    }

    return result;
}

void PLMDocumentListModel::populate()
{
    this->beginResetModel();
    m_allDocuments.clear();
     foreach(int projectId, plmProjectManager->projectIdList()) {
            QList<int> results;
            plmdata->userHub()->getIds(projectId, m_tableName, results);

            for (int documentId :  results) {
                m_allDocuments.append(PLMDocumentListItem(projectId, documentId,
                                                          m_tableName));
            }

    }
    this->endResetModel();
}


void PLMDocumentListModel::clear()
{
    this->beginResetModel();
    m_allDocuments.clear();
    this->endResetModel();
}

// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------

PLMDocumentListItem::PLMDocumentListItem()
{
    this->projectId  = -1;
    this->documentId = -1;
    this->tableName  = "";
}

PLMDocumentListItem::PLMDocumentListItem(int            projectId,
                                         int            documentId,
                                         const QString& tableName)
{
    this->projectId  = projectId;
    this->documentId = documentId;
    this->tableName  = tableName;
}

PLMDocumentListItem::~PLMDocumentListItem()
{}

PLMDocumentListItem::PLMDocumentListItem(const PLMDocumentListItem& item)
{
    this->projectId  = item.projectId;
    this->documentId = item.documentId;
    this->tableName  = item.tableName;
}

/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrrecentprojectlistmodel.cpp
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
#include "skrrecentprojectlistmodel.h"
#include <QFileInfo>
#include <QSettings>
#include "plmdata.h"

SKRRecentProjectListModel::SKRRecentProjectListModel(QObject *parent)
    : QAbstractListModel(parent)
{
    this->populate();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRRecentProjectListModel::insertInRecentProjectsFromAnId);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &SKRRecentProjectListModel::populate);
    connect(plmdata->projectHub(), &PLMProjectHub::projectNameChanged, this,
            [this](int projectId, const QString& name) {
        this->insertInRecentProjectsFromAnId(projectId);
    });
}

QVariant SKRRecentProjectListModel::headerData(int             section,
                                               Qt::Orientation orientation,
                                               int             role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return QVariant();
}

int SKRRecentProjectListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allRecentProjects.count();
}

QVariant SKRRecentProjectListModel::data(const QModelIndex& index, int role) const
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid
                        |  QAbstractItemModel::CheckIndexOption::DoNotUseParent));

    if (!index.isValid()) return QVariant();


    if (role == Qt::DisplayRole) {
        return m_allRecentProjects.at(index.row())->title;
    }

    if (role == Qt::UserRole) {
        return m_allRecentProjects.at(index.row())->title;
    }

    if (role == Qt::UserRole + 1) {
        return m_allRecentProjects.at(index.row())->fileUrl;
    }

    if (role == Qt::UserRole + 2) {
        return m_allRecentProjects.at(index.row())->writable;
    }

    if (role == Qt::UserRole + 3) {
        return m_allRecentProjects.at(index.row())->exists;
    }

    if (role == Qt::UserRole + 4) {
        return m_allRecentProjects.at(index.row())->lastModification;
    }

    if (role == Qt::UserRole + 5) {
        return m_allRecentProjects.at(index.row())->isOpened;
    }

    if (role == Qt::UserRole + 6) {
        return m_allRecentProjects.at(index.row())->projectId;
    }
    return QVariant();
}

// ----------------------------------------------------------

QHash<int, QByteArray>SKRRecentProjectListModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[Qt::UserRole]     = "title";
    roles[Qt::UserRole + 1] = "fileName";
    roles[Qt::UserRole + 2] = "writable";
    roles[Qt::UserRole + 3] = "exists";
    roles[Qt::UserRole + 4] = "lastModification";
    roles[Qt::UserRole + 5] = "isOpened";
    roles[Qt::UserRole + 6] = "projectId";

    return roles;
}

// ----------------------------------------------------------

void SKRRecentProjectListModel::insertInRecentProjects(const QString& title,
                                                       const QUrl   & fileName)
{
    bool alreadyHere      = false;
    int  alreadyHereIndex = -1;

    QSettings settings;

    settings.beginGroup("welcome");
    int size = settings.beginReadArray("recentProjects");


    for (int i = 0; i < size; ++i) {
        settings.setArrayIndex(i);

        QUrl settingFileName = settings.value("fileNameUrl").toUrl();

        if (settingFileName == fileName) {
            alreadyHere      = true;
            alreadyHereIndex = i;
        }
    }
    settings.endArray();


    if (alreadyHere) {
        // write again the title in case it was changed
        settings.beginWriteArray("recentProjects", size);
        settings.setArrayIndex(alreadyHereIndex);
        settings.setValue("title", title);
    }
    else {
        // add a new recent project
        settings.beginWriteArray("recentProjects", size + 1);
        settings.setArrayIndex(size);
        settings.setValue("title",    title);
        settings.setValue("fileNameUrl", fileName);
    }


    settings.endArray();

    settings.endGroup();

    settings.sync();

    this->populate();
}

// ----------------------------------------------------------

void SKRRecentProjectListModel::insertInRecentProjectsFromAnId(int projectId)
{
    QString title    = plmdata->projectHub()->getProjectName(projectId);
    QUrl    fileName = plmdata->projectHub()->getPath(projectId);

    this->insertInRecentProjects(title, fileName);
}


// ----------------------------------------------------------


void SKRRecentProjectListModel::forgetProject(const QUrl &fileUrl){

    m_allRecentProjects.clear();

    //read :
    QSettings settings;
    settings.beginGroup("welcome");
    int size = settings.beginReadArray("recentProjects");

    for (int i = 0; i < size; ++i) {
        settings.setArrayIndex(i);

        PLMProjectItem *projectItem = new PLMProjectItem();
        projectItem->title    = settings.value("title").toString();
        projectItem->fileUrl = settings.value("fileNameUrl").toUrl();

        // forget
        if(fileUrl != projectItem->fileUrl){
            m_allRecentProjects.append(projectItem);
        }
    }

    settings.endArray();

    //write:
    settings.beginWriteArray("recentProjects", m_allRecentProjects.count());

    for (int i = 0; i < m_allRecentProjects.count(); ++i) {
        settings.setArrayIndex(i);

        settings.setValue("title",    m_allRecentProjects.at(i)->title);
        settings.setValue("fileNameUrl", m_allRecentProjects.at(i)->fileUrl);
    }

    settings.endArray();

    settings.endGroup();

    this->populate();
}



// ----------------------------------------------------------

void SKRRecentProjectListModel::populate()
{
    this->beginResetModel();

    m_allRecentProjects.clear();

    QSettings settings;

    settings.beginGroup("welcome");
    int size = settings.beginReadArray("recentProjects");

    for (int i = 0; i < size; ++i) {
        settings.setArrayIndex(i);

        PLMProjectItem *projectItem = new PLMProjectItem();
        projectItem->title    = settings.value("title").toString();
        projectItem->fileUrl = settings.value("fileNameUrl").toUrl();

        // exists ?
        QFileInfo info(projectItem->fileUrl.toLocalFile());
        projectItem->exists = info.exists();

        // writable ?
        projectItem->writable = info.isWritable();


        // last modification :
        projectItem->lastModification = info.lastModified();

        // is project opened ?
        //default :
        projectItem->isOpened  = false;
        projectItem->projectId = -2;

        for (int projectId : plmdata->projectHub()->getProjectIdList()) {
            QString projectName = plmdata->projectHub()->getProjectName(projectId);
            QUrl    projectPath = plmdata->projectHub()->getPath(projectId);

            if ((projectName == projectItem->title) &&
                    (projectPath == projectItem->fileUrl)) {
                projectItem->isOpened  = true;
                projectItem->projectId = projectId;

            }
        }

        m_allRecentProjects.append(projectItem);



    }


    settings.endArray();
    settings.endGroup();

    //sorting

    //sorting opened first
    QList<PLMProjectItem *> openedList;
    for (PLMProjectItem *item : m_allRecentProjects) {
        if(item->isOpened){
            openedList.append(item);
        }
    }



    //    //sorting opened more recent second
    QList<PLMProjectItem *> moreRecentList;
    for (PLMProjectItem *item : m_allRecentProjects) {
        if(!item->isOpened){
            moreRecentList.append(item);
        }
    }

    for(int p = 0 ; p < moreRecentList.count() ; p++ ){
        for(int i = 0 ; i < moreRecentList.count() ; i++ ){
            QDateTime lastModification = moreRecentList.at(i)->lastModification;

            if(i + 1 ==  moreRecentList.count()){
                break;
            }


            QDateTime lastModificationFromItemAfter = moreRecentList.at(i + 1)->lastModification;

            if(lastModification < lastModificationFromItemAfter){
                moreRecentList.swapItemsAt(i, i + 1);
            }

        }
    }

    m_allRecentProjects.clear();
    m_allRecentProjects << openedList << moreRecentList;


    this->endResetModel();
}

// ----------------------------------------------------------

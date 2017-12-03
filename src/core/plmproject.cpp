/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmproject.cpp                                                   *
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

#include "plmproject.h"
#include "../data/plmdata.h"
#include "plmmessagehandler.h"

#include <QFileInfo>
#include <QDir>


PLMProject::PLMProject(QObject *parent) :
    QObject(parent)
{
}

PLMProject::~PLMProject()
{

}

int PLMProject::projectId() const
{
    return m_projectId;
}

void PLMProject::setProjectId(int projectId)
{
    m_projectId = projectId;
}

PLMSheetTreeModel *PLMProject::sheetTreeModel() const
{
    return m_sheetTreeModel;
}

PLMSheetPropertyModel *PLMProject::sheetPropertyModel() const
{
    return m_sheetPropertyModel;
}


PLMNoteTreeModel *PLMProject::noteTreeModel() const
{
    return m_noteTreeModel;
}

PLMNotePropertyModel *PLMProject::notePropertyModel() const
{
    return m_notePropertyModel;
}


PLMNoteListModel *PLMProject::noteListModel() const
{
    return m_noteListModel;
}



bool PLMProject::loadProject(const QString& fileName)
{
    m_projectFileName = fileName;
    //
    if(fileName == "")//create empty database :
        plmdata->createNewEmptyDatabase(m_projectId);
    else
        plmdata->loadDatabase(m_projectId, fileName);
    //init models
    m_sheetTreeModel = new PLMSheetTreeModel(this, m_projectId);
    m_sheetTreeModel->resetModel();
    m_sheetPropertyModel = new PLMSheetPropertyModel(this, m_projectId);
    m_sheetPropertyModel->resetModel();
    m_noteTreeModel = new PLMNoteTreeModel(this, m_projectId);
    m_noteListModel = new PLMNoteListModel(this, m_projectId);
    connect(m_noteTreeModel, &PLMNoteTreeModel::modelReset, m_noteListModel, &PLMNoteListModel::resetModel);
    m_noteTreeModel->resetModel();
    m_notePropertyModel = new PLMNotePropertyModel(this, m_projectId);
    m_notePropertyModel->resetModel();

    m_sheetDocumentRepo = new PLMDocumentRepo(this, m_sheetTreeModel);
    m_noteDocumentRepo = new PLMDocumentRepo(this, m_noteTreeModel);


}

bool PLMProject::saveProjectAs(const QString& fileName)
{
    if(QFileInfo(m_projectFileName).fileName().contains("plume_test_project.sqlite"))
        return false;

    if(!QFileInfo(QFileInfo(m_projectFileName).path()).isWritable()){
        PLMMessageHandler::instance()->sendMessage(tr("%1 is not writable").arg(m_projectFileName));
        return false;
      }

    return plmdata->saveDatabase(m_projectId, fileName);



}


bool PLMProject::saveProject()
{

    return this->saveProjectAs(m_projectFileName);



}

PLMDocumentRepo *PLMProject::sheetDocumentRepo() const
{
    return m_sheetDocumentRepo;
}

PLMDocumentRepo *PLMProject::noteDocumentRepo() const
{
    return m_noteDocumentRepo;
}

QString PLMProject::projectFileName() const
{
    return m_projectFileName;
}

void PLMProject::setProjectFileName(const QString &projectFileName)
{
    m_projectFileName = projectFileName;
}

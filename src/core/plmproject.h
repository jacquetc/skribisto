/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmproject.h                                                   *
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

#ifndef PLMPROJECT_H
#define PLMPROJECT_H

#include <QObject>
#include "models/plmsheettreemodel.h"
#include "models/plmsheetpropertymodel.h"
#include "models/plmnotetreemodel.h"
#include "models/plmnotepropertymodel.h"
#include <models/plmnotelistmodel.h>
#include "plmdocumentrepo.h"

class PLMProject : public QObject
{
    Q_OBJECT
public:
    explicit PLMProject(QObject *parent = 0);
    ~PLMProject();



    int projectId() const;
    void setProjectId(int projectId);

    PLMSheetTreeModel *sheetTreeModel() const;
    PLMSheetPropertyModel *sheetPropertyModel() const;
    PLMNoteTreeModel *noteTreeModel() const;
    PLMNotePropertyModel *notePropertyModel() const;
    PLMNoteListModel *noteListModel() const;

    PLMDocumentRepo *sheetDocumentRepo() const;
    PLMDocumentRepo *noteDocumentRepo() const;

    QString projectFileName() const;
    void setProjectFileName(const QString &projectFileName);


signals:
    void projectStarted(bool);

    void openSheetInWriteWritingZone_signal(int paperId);
    void openNoteInNoteWritingZone_signal(int paperId);
    void openNoteInBinder_signal(int paperId);

public slots:

    bool loadProject(const QString &fileName);
    bool saveProjectAs(const QString &fileName);
    bool saveProject();
private:
    PLMSheetTreeModel *m_sheetTreeModel;
    PLMSheetPropertyModel *m_sheetPropertyModel;
    PLMNoteTreeModel *m_noteTreeModel;
    PLMNotePropertyModel *m_notePropertyModel;
    PLMDocumentRepo *m_sheetDocumentRepo, *m_noteDocumentRepo;
    PLMNoteListModel *m_noteListModel;

    int m_projectId;
    QString m_projectFileName;

};

#endif // PLMPROJECT_H

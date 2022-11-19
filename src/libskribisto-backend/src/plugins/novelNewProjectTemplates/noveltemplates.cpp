/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrexporter.cpp                                                   *
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
#include "noveltemplates.h"


#include "projecttreecommands.h"

NovelTemplates::NovelTemplates(QObject *parent) : QObject(parent)
{

}

// ---------------------------------------------------

NovelTemplates::~NovelTemplates()
{}



QStringList NovelTemplates::templateDetailText() const
{
    return QStringList() << tr("Create a skeleton project for light novels.")
                         << tr("Create a skeleton project for novels.");

}

void NovelTemplates::applyTemplate(int projectId, const QString &templateName)
{
    int sortOrder = 1;

    QString chapterTitle("Chapter %1");
    int chapterNumber = 1;

    QString sceneTitle("Scene %1");


    if(templateName == "light novel"){

        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 1, "FOLDER", tr("Writings"));

        QVariantMap customProperties;
        customProperties.insert("section_type", "book-beginning");
        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "SECTION", skrdata->projectHub()->getProjectName(projectId),  QString(), customProperties);

        for(int i = 0; i < 15; i++){
            int sceneNumber = 1;

            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "FOLDER", chapterTitle.arg(chapterNumber));
            customProperties.clear();
            customProperties.insert("section_type", "chapter");
            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 3, "SECTION", chapterTitle.arg(chapterNumber++),  QString(), customProperties);
            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 3, "TEXT", sceneTitle.arg(sceneNumber++));


        }

        customProperties.clear();
        customProperties.insert("section_type", "book-end");
        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "SECTION", "",  QString(), customProperties);

        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 1, "FOLDER", tr("Notes"), "note_folder");



    }
    else if(templateName == "novel"){

        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 1, "FOLDER", tr("Writings"));

        QVariantMap customProperties;
        customProperties.insert("section_type", "book-beginning");
        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "SECTION", skrdata->projectHub()->getProjectName(projectId),  QString(), customProperties);

        for(int i = 0; i < 20; i++){
            int sceneNumber = 1;

            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "FOLDER", chapterTitle.arg(chapterNumber));
            customProperties.clear();
            customProperties.insert("section_type", "chapter");
            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 3, "SECTION", chapterTitle.arg(chapterNumber++),  QString(), customProperties);
            projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 3, "TEXT", sceneTitle.arg(sceneNumber++));


        }

        customProperties.clear();
        customProperties.insert("section_type", "book-end");
        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 2, "SECTION", "",  QString(), customProperties);

        projectTreeCommands->addTreeItemInTemplate(projectId, sortOrder++, 1, "FOLDER", tr("Notes"), "note_folder");



    }
}

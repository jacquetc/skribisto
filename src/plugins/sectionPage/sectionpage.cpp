/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: folderpage.cpp                                                   *
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
#include "sectionpage.h"
#include "skrdata.h"

SectionPage::SectionPage(QObject *parent) : QObject(parent)
{}

// ---------------------------------------------------

SectionPage::~SectionPage()
{}

// ---------------------------------------------------

QString SectionPage::pageTypeIconUrl(int projectId, int treeItemId) const {

    if(projectId == -1 && treeItemId == -1){
        return "qrc:///icons/backup/bookmark-new.svg";
    }

    QString section_type = skrdata->treePropertyHub()->getProperty(projectId, treeItemId, "section_type", "separator");


    if (section_type == "book-beginning") {
        return "qrc:///icons/backup/skribisto-book-beginning.svg";
    }
    else if (section_type == "book-end") {
        return "qrc:///icons/backup/skribisto-book-end.svg";
    }
    else if (section_type == "chapter") {
        return "qrc:///icons/backup/bookmark-new.svg";
    }
    else if (section_type == "separator") {
        return "qrc:///icons/backup/menu_new_sep.svg";
    }


    return "qrc:///icons/backup/bookmark-new.svg";
}

// ---------------------------------------------------

SKRResult SectionPage::finaliseAfterCreationOfTreeItem(int projectId, int treeItemId)
{
    SKRResult result(this);

    result = skrdata->treePropertyHub()->setProperty(projectId, treeItemId, "can_add_child_tree_item", "false", true);

    QString section_type = m_creationParametersMap.value("section_type", "separator").toString();
    result = skrdata->treePropertyHub()->setProperty(projectId, treeItemId, "section_type", section_type, true);

    return result;
}

// ---------------------------------------------------

void SectionPage::updateCharAndWordCount(int projectId, int treeItemId, bool sameThread)
{}

// ---------------------------------------------------
QTextDocumentFragment SectionPage::generateExporterTextFragment(int                projectId,
                                                                int                treeItemId,
                                                                const QVariantMap& exportProperties,
                                                                SKRResult        & result) const
{
    QTextDocument *document = nullptr;


    return QTextDocumentFragment(document);
}

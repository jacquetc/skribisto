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
#include "creationparameterswidget.h"
#include "sectionview.h"

SectionPage::SectionPage(QObject *parent) : QObject(parent)
{}

// ---------------------------------------------------

SectionPage::~SectionPage()
{}

View *SectionPage::getView() const
{
  return new SectionView();
}

TreeItemCreationParametersWidget *SectionPage::pageCreationParametersWidget() const
{
    auto *widget = new CreationParametersWidget;

            return widget;

}

// ---------------------------------------------------

QString SectionPage::pageTypeIconUrl(int projectId, int treeItemId) const {

    if(projectId == -1 && treeItemId == -1){
        return ":/icons/backup/bookmark-new.svg";
    }

    QString section_type = skrdata->treePropertyHub()->getProperty(projectId, treeItemId, "section_type", "separator");


    if (section_type == "book-beginning") {
        return ":/icons/backup/skribisto-book-beginning.svg";
    }
    else if (section_type == "book-end") {
        return ":/icons/backup/skribisto-book-end.svg";
    }
    else if (section_type == "chapter") {
        return ":/icons/backup/bookmark-new.svg";
    }
    else if (section_type == "separator") {
        return ":/icons/backup/menu_new_sep.svg";
    }


    return ":/icons/backup/bookmark-new.svg";
}

// ---------------------------------------------------

QVariantMap SectionPage::propertiesForCreationOfTreeItem(const QVariantMap &customProperties) const
{
    QVariantMap properties;

    properties.insert("can_add_child_tree_item", "false");
    properties.insert("section_type", "separator");

    properties.insert(customProperties);

    return properties;
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

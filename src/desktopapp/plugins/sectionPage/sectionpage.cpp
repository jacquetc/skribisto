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
#include "text/markdowntextdocument.h"
#include "skrdata.h"
#include "creationparameterswidget.h"
#include "sectionview.h"

#include <QTextCursor>

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
        return ":/icons/skribisto/skribisto-book-beginning.svg";
    }
    else if (section_type == "book-end") {
        return ":/icons/skribisto/skribisto-book-end.svg";
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
    MarkdownTextDocument document;
    QString sectionType = skrdata->treePropertyHub()->getProperty(projectId, treeItemId, "section_type", "separator");

    int indent = skrdata->treeHub()->getIndent(projectId, treeItemId);

    QFont font;
    font.setFamily(exportProperties.value("font_family", "Times New Roman").toString());
    int pointSize = exportProperties.value("font_size", 12).toInt();

    if(sectionType == "book-beginning") {
        pointSize += 6;
        font.setBold(true);
    }
    else if(sectionType == "chapter"){
        pointSize += 3;
        font.setBold(true);
    }
    font.setPointSize(pointSize);

    QTextBlockFormat blockFormat;
    blockFormat.setTextIndent(exportProperties.value("text_block_indent", 0).toInt());
    blockFormat.setTopMargin(exportProperties.value("text_block_top_margin", 0).toInt());
    blockFormat.setLineHeight(exportProperties.value("text_space_between_line", 100).toInt(), QTextBlockFormat::ProportionalHeight);
    blockFormat.setHeadingLevel(indent);
    blockFormat.setAlignment(Qt::AlignHCenter);

    QTextCharFormat charFormat;
    charFormat.setFont(font, QTextCharFormat::FontPropertiesSpecifiedOnly);

    QString title = skrdata->treeHub()->getTitle(projectId, treeItemId);

    if(sectionType == "book-beginning" || sectionType == "chapter") {
        document.setSkribistoMarkdown(title);
    }
    else if(sectionType == "separator"){
        document.setPlainText("***");
    }

    QTextCursor cursor(&document);
    cursor.insertBlock(blockFormat, charFormat);
    cursor.select(QTextCursor::SelectionType::Document);
    cursor.mergeBlockCharFormat(charFormat);
    cursor.mergeCharFormat(charFormat);
    cursor.mergeBlockFormat(blockFormat);
    cursor.movePosition(QTextCursor::MoveOperation::End);
    cursor.insertBlock(blockFormat, charFormat);




    return QTextDocumentFragment(&document);
}

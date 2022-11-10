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
#include "folderpage.h"
#include "folderview.h"
#include "text/markdowntextdocument.h"
#include "skrdata.h"

#include <QTextCursor>

FolderPage::FolderPage(QObject *parent) : QObject(parent)
{


}

// ---------------------------------------------------

FolderPage::~FolderPage()
{}

// ---------------------------------------------------

View *FolderPage::getView() const
{
        FolderView* view = new FolderView;
        return view;
}

//---------------------------------------------------

QVariantMap FolderPage::propertiesForCreationOfTreeItem(const QVariantMap &customProperties) const
{
 return QVariantMap();
}
// ---------------------------------------------------

void FolderPage::updateCharAndWordCount(int projectId, int treeItemId, bool sameThread)
{
}

// ---------------------------------------------------
QTextDocumentFragment FolderPage::generateExporterTextFragment(int                projectId,
                                                             int                treeItemId,
                                                             const QVariantMap& exportProperties,
                                                             SKRResult        & result) const
{
    MarkdownTextDocument document;

    int indent = skrdata->treeHub()->getIndent(projectId, treeItemId);

    QFont font;
    font.setFamily(exportProperties.value("font_family", "Times New Roman").toString());
    int pointSize = exportProperties.value("font_size", 12).toInt();
    pointSize += 6 - indent;
    font.setPointSize(pointSize);
    font.setBold(true);

    QTextBlockFormat blockFormat;
    blockFormat.setTextIndent(exportProperties.value("text_block_indent", 0).toInt());
    blockFormat.setTopMargin(exportProperties.value("text_block_top_margin", 0).toInt());
    blockFormat.setLineHeight(exportProperties.value("text_space_between_line", 100).toInt(), QTextBlockFormat::ProportionalHeight);
    blockFormat.setHeadingLevel(indent);
    blockFormat.setAlignment(Qt::AlignHCenter);

    QTextCharFormat charFormat;
    charFormat.setFont(font, QTextCharFormat::FontPropertiesSpecifiedOnly);

    QString title = skrdata->treeHub()->getTitle(projectId, treeItemId);

    document.setSkribistoMarkdown(title);

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

/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: textpage.cpp                                                   *
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
#include "textpage.h"
#include "skrdata.h"
#include "text/markdowntextdocument.h"
#include "textpagesettings.h"
#include "textview.h"

#include <QTextCursor>

TextPage::TextPage(QObject *parent) : QObject(parent)
{
    m_wordMeter = new SKRWordMeter(this);

    connect(m_wordMeter, &SKRWordMeter::characterCountCalculated, this,
            [](const TreeItemAddress &treeItemAddress, int charCount, bool triggerProjectModifiedSignal) {
                skrdata->statHub()->updateStats(treeItemAddress, SKRStatHub::Character, charCount,
                                                triggerProjectModifiedSignal);
            });

    connect(m_wordMeter, &SKRWordMeter::wordCountCalculated, this,
            [](const TreeItemAddress &treeItemAddress, int wordCount, bool triggerProjectModifiedSignal) {
                skrdata->statHub()->updateStats(treeItemAddress, SKRStatHub::Word, wordCount,
                                                triggerProjectModifiedSignal);
            });
}

// ---------------------------------------------------

TextPage::~TextPage()
{
}

// ---------------------------------------------------

View *TextPage::getView() const
{
    return new TextView();
}

// ---------------------------------------------------

QVariantMap TextPage::propertiesForCreationOfTreeItem(const QVariantMap &customProperties) const
{
    QVariantMap properties;

    properties.insert("can_add_child_tree_item", "false");

    properties.insert(customProperties);

    return properties;
}
// ---------------------------------------------------

void TextPage::updateCharAndWordCount(const TreeItemAddress &treeItemAddress, bool sameThread,
                                      bool triggerProjectModifiedSignal)
{
    // deactivated because of lag
    //    const QString &primaryContent = skrdata->treeHub()->getPrimaryContent(treeItemAddress);

    //    MarkdownTextDocument textDocument;

    //    textDocument.setSkribistoMarkdown(primaryContent);
    //    const QString &plainText = textDocument.toPlainText();

    //    m_wordMeter->countText(treeItemAddress, std::move(plainText), sameThread, triggerProjectModifiedSignal);
}

// ---------------------------------------------------
QTextDocumentFragment TextPage::generateExporterTextFragment(const TreeItemAddress &treeItemAddress,
                                                             const QVariantMap &exportProperties,
                                                             SKRResult &result) const
{
    MarkdownTextDocument document;

    QFont font;
    font.setFamily(exportProperties.value("font_family", "Times New Roman").toString());
    font.setPointSize(exportProperties.value("font_size", 12).toInt());

    QTextCharFormat charFormat;
    charFormat.setFont(font, QTextCharFormat::FontPropertiesSpecifiedOnly);

    QTextBlockFormat blockFormat;
    blockFormat.setTextIndent(exportProperties.value("text_block_indent", 0).toInt());
    blockFormat.setTopMargin(exportProperties.value("text_block_top_margin", 0).toInt());
    blockFormat.setLineHeight(exportProperties.value("text_space_between_line", 100).toInt(),
                              QTextBlockFormat::ProportionalHeight);

    QString primaryContent = skrdata->treeHub()->getPrimaryContent(treeItemAddress);

    document.setSkribistoMarkdown(primaryContent);

    QTextCursor cursor(&document);
    cursor.select(QTextCursor::SelectionType::Document);
    cursor.mergeBlockCharFormat(charFormat);
    cursor.mergeCharFormat(charFormat);
    cursor.mergeBlockFormat(blockFormat);

    return QTextDocumentFragment(&document);
}

SettingsSubPanel *TextPage::settingsPanel() const
{
    return new TextPageSettings();
}

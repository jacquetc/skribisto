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
#include "textview.h"
#include "textpagesettings.h"

TextPage::TextPage(QObject *parent) : QObject(parent)
{
    m_wordMeter = new SKRWordMeter(this);

    connect(m_wordMeter, &SKRWordMeter::characterCountCalculated, skrdata->statHub(),
            &SKRStatHub::updateCharacterStats);
    connect(m_wordMeter,
            &SKRWordMeter::wordCountCalculated,
            skrdata->statHub(),
            &SKRStatHub::updateWordStats);
}

// ---------------------------------------------------

TextPage::~TextPage()
{}

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

void TextPage::updateCharAndWordCount(int projectId, int treeItemId, bool sameThread)
{
    QString primaryContent = skrdata->treeHub()->getPrimaryContent(projectId, treeItemId);

    m_wordMeter->countText(projectId, treeItemId, primaryContent, sameThread, false);
}

// ---------------------------------------------------
QTextDocumentFragment TextPage::generateExporterTextFragment(int                projectId,
                                                             int                treeItemId,
                                                             const QVariantMap& exportProperties,
                                                             SKRResult        & result) const
{
    QTextDocument *document = nullptr;


    return QTextDocumentFragment(document);
}

SettingsPanel *TextPage::settingsPanel() const
{
    return new TextPageSettings();
}

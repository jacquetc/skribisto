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
#include "plmdata.h"

TextPage::TextPage(QObject *parent) : QObject(parent)
{

}

//---------------------------------------------------

TextPage::~TextPage()
{

}

//---------------------------------------------------

SKRResult TextPage::finaliseAfterCreationOfTreeItem(int projectId, int treeItemId)
{
    SKRResult result(this);

    result = plmdata->treePropertyHub()->setProperty(projectId, treeItemId, "can_add_child_paper", "false", true);


    return result;
}



//---------------------------------------------------
QTextDocumentFragment TextPage::generateExporterTextFragment(int projectId, int treeItemId, const QVariantMap &exportProperties, SKRResult &result) const
{
    QTextDocument *document = nullptr;


    return QTextDocumentFragment(document);
}

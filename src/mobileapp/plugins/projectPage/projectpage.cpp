/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: projectpage.cpp                                                   *
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
#include "projectpage.h"
#include "skrdata.h"

ProjectPage::ProjectPage(QObject *parent) : QObject(parent)
{}

// ---------------------------------------------------

ProjectPage::~ProjectPage()
{}

// ---------------------------------------------------

SKRResult ProjectPage::finaliseAfterCreationOfTreeItem(int projectId, int treeItemId)
{
    SKRResult result(this);

    return result;
}

// ---------------------------------------------------

void ProjectPage::updateCharAndWordCount(int projectId, int treeItemId, bool sameThread)
{}

// ---------------------------------------------------
QTextDocumentFragment ProjectPage::generateExporterTextFragment(int                projectId,
                                                                int                treeItemId,
                                                                const QVariantMap& exportProperties,
                                                                SKRResult        & result) const
{
    QTextDocument *document = nullptr;


    return QTextDocumentFragment(document);
}

/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmnotetree.cpp                                                   *
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
#include "plmnotetree.h"

PLMNoteTree::PLMNoteTree(QObject       *parent,
                         const QString& tableName,
                         const QString& codeName,
                         QSqlDatabase   sqlDb) :
    PLMTree(parent, tableName, codeName, sqlDb)
{}

int PLMNoteTree::addNewChildNote(int parentNoteId)
{
    return addNewChildPapers(parentNoteId, 1).first();
}

void PLMNoteTree::moveNoteToSynopsis(int parentNoteId)
{
    Q_UNUSED(parentNoteId);
}

bool PLMNoteTree::getIsSynopsis(int noteId)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), noteId, false);

    return paper.get("b_synopsis").toBool();
}

void PLMNoteTree::setIsSynopsis(int noteId, bool value)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), noteId, true);

    paper.set("b_synopsis", value);
}

int PLMNoteTree::getSheetCode(int noteId)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), noteId, false);

    return paper.get("l_sheet_code").toInt();
}

void PLMNoteTree::setSheetCode(int noteId, int value)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), noteId, true);

    paper.set("l_sheet_code", value);
}

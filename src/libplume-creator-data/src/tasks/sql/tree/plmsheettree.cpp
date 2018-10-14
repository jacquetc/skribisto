/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmsheettree.cpp                                                   *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#include "plmsheettree.h"

PLMSheetTree::PLMSheetTree(QObject *parent, const QString &tableName, const QString &codeName, QSqlDatabase sqlDb) :
    PLMTree(parent, tableName, codeName, sqlDb)
{

}

int PLMSheetTree::getWordCount(int paperId) const
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
    return paper.get("l_word_count").toInt();
}

void PLMSheetTree::setWordCount(int paperId, int count)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, true);
    paper.set("l_word_count", count);

}

int PLMSheetTree::getCharCount(int paperId) const
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, false);
    return paper.get("l_char_count").toInt();

}

void PLMSheetTree::setCharCount(int paperId, int count)
{
    PLMDbPaper paper(sqlDb(), tableName(), idName(), paperId, true);
    paper.set("l_char_count", count);

}

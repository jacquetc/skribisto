/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmsheettree.h                                                   *
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
#ifndef PLMSHEETTREE_H
#define PLMSHEETTREE_H

#include "plmtree.h"

class PLMSheetTree : public PLMTree
{
    Q_OBJECT

public:
    PLMSheetTree(QObject *parent, const QString &tableName, const QString &codeName, QSqlDatabase sqlDb);
    int getWordCount(int paperId) const;
    void setWordCount(int paperId, int count);
    int getCharCount(int paperId) const;
    void setCharCount(int paperId, int count);

};

#endif // PLMSHEETTREE_H

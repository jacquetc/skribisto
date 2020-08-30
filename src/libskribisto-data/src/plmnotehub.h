/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmnotehub.h                                                   *
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
#ifndef PLMNOTEHUB_H
#define PLMNOTEHUB_H

#include <QObject>
#include "plmpaperhub.h"
#include "skribisto_data_global.h"


class EXPORT PLMNoteHub : public PLMPaperHub
{
    Q_OBJECT
public:
    PLMNoteHub(QObject *parent);
    int getSynopsisFromSheetCode(int projectId, int sheetId);
    QHash<int, int> getAllSynopsisWithSheetCode(int projectId);
    QHash<int, int> getAllSheetCodes(int projectId);


    QHash<QString, QVariant>getNoteData(int projectId,
                                         int noteId) const;
};

#endif // PLMNOTEHUB_H

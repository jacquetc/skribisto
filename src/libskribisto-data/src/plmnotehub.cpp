/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmnotehub.cpp                                                   *
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
#include "plmnotehub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"

PLMNoteHub::PLMNoteHub(QObject *parent) : PLMPaperHub(parent, "tbl_note")
{

}

int PLMNoteHub::getSynopsisFromSheetCode(int projectId, int sheetId)
{
    Q_UNUSED(projectId);
    Q_UNUSED(sheetId);

    int noteId = 2;
    // create a note /synopsis for if none found

    return noteId;
}
///
/// \brief PLMNoteHub::getAllSynopsis
/// \param projectId
/// \return QHash <int  sheet_id, int sheet_code>
///
///
QHash<int, int> PLMNoteHub::getAllSynopsisWithSheetCode(int projectId)
{

    PLMError error;
    QHash<int, int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("l_sheet_code", out, "b_synopsis", true);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);
    }

    IFKO(error) {
    emit errorSent(error);
    }
    return result;

}

QHash<int, int> PLMNoteHub::getAllSheetCodes(int projectId)
{

    PLMError error;
    QHash<int, int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getValueByIds("l_sheet_code", out);

    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);
    }


    IFKO(error) {
    emit errorSent(error);
    }
    return result;

}

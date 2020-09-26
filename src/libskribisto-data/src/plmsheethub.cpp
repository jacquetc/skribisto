/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheethub.cpp                                                   *
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
#include "plmsheethub.h"
#include "tasks/plmsqlqueries.h"

PLMSheetHub::PLMSheetHub(QObject *parent) : PLMPaperHub(parent, "tbl_sheet")
{}

QHash<QString, QVariant>PLMSheetHub::getSheetData(int projectId, int sheetId) const
{
    PLMError error;

    QHash<QString, QVariant> var;
    QHash<QString, QVariant> result;
    QStringList fieldNames;

    fieldNames << "l_sheet_id" << "l_dna" << "l_sort_order" << "l_indent" <<
        "l_version" << "t_title" << "dt_created" << "dt_updated" << "dt_content" <<
        "b_trashed";
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.getMultipleValues(sheetId, fieldNames, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

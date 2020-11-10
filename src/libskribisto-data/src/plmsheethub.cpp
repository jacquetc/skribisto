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
#include "plmdata.h"
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

// -------------------------------------------------------------

PLMError PLMSheetHub::setTitle(int projectId, int paperId, const QString& newTitle)
{
    PLMError error = PLMPaperHub::setTitle(projectId, paperId, newTitle);

    IFOK(error) {
        // rename related synopsis:
        int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);

        error = plmdata->noteHub()->setTitle(projectId, synopsisId, newTitle);
    }
    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::addPaperAbove(int projectId, int targetId)
{
    // create sheet
    PLMError error = PLMPaperHub::addPaperAbove(projectId, targetId);

    // create synopsis
    IFOK(error) {
        int sheetId = error.getData("paperId", -2).toInt();

        error = plmdata->noteHub()->createSynopsis(projectId, sheetId);
    }

    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::addPaperBelow(int projectId, int targetId)
{
    PLMError error;

    // create synopsis


    // create sheet
    error = PLMPaperHub::addPaperBelow(projectId, targetId);

    // create synopsis
    IFOK(error) {
        int sheetId = error.getData("paperId", -2).toInt();

        error = plmdata->noteHub()->createSynopsis(projectId, sheetId);
    }

    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::addChildPaper(int projectId, int targetId)
{
    PLMError error;

    // create synopsis


    // create sheet
    error = PLMPaperHub::addChildPaper(projectId, targetId);

    // create synopsis
    IFOK(error) {
        int sheetId = error.getData("paperId", -2).toInt();

        error = plmdata->noteHub()->createSynopsis(projectId, sheetId);
    }

    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::setTrashedWithChildren(int  projectId,
                                             int  paperId,
                                             bool newTrashedState)
{
    PLMError error = PLMPaperHub::setTrashedWithChildren(projectId,
                                                         paperId,
                                                         newTrashedState);

    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);

    if (synopsisId == -2) {
        return error;
    }

    IFOKDO(error,
           plmdata->noteHub()->setTrashedWithChildren(projectId, synopsisId,
                                                      newTrashedState));


    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::untrashOnlyOnePaper(int projectId, int paperId)
{
    PLMError error = PLMPaperHub::untrashOnlyOnePaper(projectId, paperId);

    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);

    if (synopsisId == -2) {
        return error;
    }
    IFOKDO(error,
           plmdata->noteHub()->untrashOnlyOnePaper(projectId, synopsisId));
    return error;
}

// -------------------------------------------------------------

PLMError PLMSheetHub::removePaper(int projectId, int targetId)
{
    PLMError error = PLMPaperHub::removePaper(projectId, targetId);

    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, targetId);

    if (synopsisId == -2) {
        return error;
    }

    IFOKDO(error,
           plmdata->noteHub()->removePaper(projectId, synopsisId));
    return error;
}

// --------------------------------------------------------------------------------

QList<QString>PLMSheetHub::getAttributes(int projectId, int paperId)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");

    return attributes.split(";", Qt::SkipEmptyParts);
}

// --------------------------------------------------------------------------------


bool PLMSheetHub::hasAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");

    return attributes.split(";", Qt::SkipEmptyParts).contains(attribute);
}

// --------------------------------------------------------------------------------

PLMError PLMSheetHub::addAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (attributeList.contains(attribute)) {
        return PLMError();
    }

    attributeList << attribute;
    attributeList.sort();

    return plmdata->notePropertyHub()->setProperty(projectId,
                                                   paperId,
                                                   "attributes",
                                                   attributeList.join(";"));
}

// --------------------------------------------------------------------------------

PLMError PLMSheetHub::removeAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->notePropertyHub()->getProperty(projectId,
                                                                 paperId,
                                                                 "attributes",
                                                                 "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (!attributeList.contains(attribute)) {
        return PLMError();
    }

    attributeList.removeAll(attribute);

    return plmdata->notePropertyHub()->setProperty(projectId,
                                                   paperId,
                                                   "attributes",
                                                   attributeList.join(";"));
}

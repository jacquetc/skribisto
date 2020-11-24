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
#include "skr.h"
#include "tasks/plmsqlqueries.h"

PLMSheetHub::PLMSheetHub(QObject *parent) : PLMPaperHub(parent, "tbl_sheet", SKR::Sheet)
{}

QHash<QString, QVariant>PLMSheetHub::getSheetData(int projectId, int sheetId) const
{
    SKRResult result(this);

    QHash<QString, QVariant> var;
    QHash<QString, QVariant> hash;
    QStringList fieldNames;

    fieldNames << "l_sheet_id" << "l_dna" << "l_sort_order" << "l_indent" <<
        "l_version" << "t_title" << "dt_created" << "dt_updated" << "dt_content" <<
        "b_trashed";
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getMultipleValues(sheetId, fieldNames, var);
    IFOK(result) {
        hash = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::setTitle(int projectId, int paperId, const QString& newTitle)
{
    SKRResult result = PLMPaperHub::setTitle(projectId, paperId, newTitle);

    IFOK(result) {
        // rename related synopsis:
        int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);

        result = plmdata->noteHub()->setTitle(projectId, synopsisId, newTitle);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::addPaperAbove(int projectId, int targetId)
{
    // create sheet
    SKRResult result = PLMPaperHub::addPaperAbove(projectId, targetId);
    result.addData("sheetId", result.getData("paperId", -2));

    // create synopsis
    IFOK(result) {
        int sheetId = result.getData("paperId", -2).toInt();

        result = plmdata->noteHub()->createSynopsis(projectId, sheetId);
        result.addData("noteId", result.getData("paperId", -2));
    }
    result.addData("paperId", -2);

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::addPaperBelow(int projectId, int targetId)
{
    SKRResult result(this);

    // create synopsis


    // create sheet
    result = PLMPaperHub::addPaperBelow(projectId, targetId);
    result.addData("sheetId", result.getData("paperId", -2));

    // create synopsis
    IFOK(result) {
        int sheetId = result.getData("paperId", -2).toInt();

        result = plmdata->noteHub()->createSynopsis(projectId, sheetId);
        result.addData("noteId", result.getData("paperId", -2));
    }
    result.addData("paperId", -2);

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::addChildPaper(int projectId, int targetId)
{
    SKRResult result(this);

    // create synopsis


    // create sheet
    result = PLMPaperHub::addChildPaper(projectId, targetId);
    result.addData("sheetId", result.getData("paperId", -2));

    // create synopsis
    IFOK(result) {
        int sheetId = result.getData("paperId", -2).toInt();

        result = plmdata->noteHub()->createSynopsis(projectId, sheetId);
        result.addData("noteId", result.getData("paperId", -2));
    }
    result.addData("paperId", -2);

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::setTrashedWithChildren(int  projectId,
                                             int  paperId,
                                             bool newTrashedState)
{

    QList<int> childrenSheets = this->getAllChildren(projectId, paperId);

    SKRResult result = PLMPaperHub::setTrashedWithChildren(projectId,
                                                         paperId,
                                                         newTrashedState);



    IFOK(result) {
        childrenSheets.prepend(paperId);

        for(int sheetId : childrenSheets){
            int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, sheetId);

            if (synopsisId == -2) {
                return result;
            }
            plmdata->noteHub()->setTrashedWithChildren(projectId, synopsisId, newTrashedState);

        }

    }


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::untrashOnlyOnePaper(int projectId, int paperId)
{
    SKRResult result = PLMPaperHub::untrashOnlyOnePaper(projectId, paperId);

    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, paperId);

    if (synopsisId == -2) {
        return result;
    }
    IFOKDO(result,
           plmdata->noteHub()->untrashOnlyOnePaper(projectId, synopsisId));


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -------------------------------------------------------------

SKRResult PLMSheetHub::removePaper(int projectId, int targetId)
{
    SKRResult result = PLMPaperHub::removePaper(projectId, targetId);

    int synopsisId = plmdata->noteHub()->getSynopsisNoteId(projectId, targetId);

    if (synopsisId == -2) {
        return result;
    }

    IFOKDO(result,
           plmdata->noteHub()->removePaper(projectId, synopsisId));


    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// --------------------------------------------------------------------------------

QList<QString>PLMSheetHub::getAttributes(int projectId, int paperId)
{
    QString attributes = plmdata->sheetPropertyHub()->getProperty(projectId,
                                                                  paperId,
                                                                  "attributes",
                                                                  "");

    return attributes.split(";", Qt::SkipEmptyParts);
}

// --------------------------------------------------------------------------------


bool PLMSheetHub::hasAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->sheetPropertyHub()->getProperty(projectId,
                                                                  paperId,
                                                                  "attributes",
                                                                  "");

    return attributes.split(";", Qt::SkipEmptyParts).contains(attribute);
}

// --------------------------------------------------------------------------------

SKRResult PLMSheetHub::addAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->sheetPropertyHub()->getProperty(projectId,
                                                                  paperId,
                                                                  "attributes",
                                                                  "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (attributeList.contains(attribute)) {
        return SKRResult();
    }

    attributeList << attribute;
    attributeList.sort();

    return plmdata->sheetPropertyHub()->setProperty(projectId,
                                                    paperId,
                                                    "attributes",
                                                    attributeList.join(";"));
}

// --------------------------------------------------------------------------------

SKRResult PLMSheetHub::removeAttribute(int projectId, int paperId, const QString& attribute)
{
    QString attributes = plmdata->sheetPropertyHub()->getProperty(projectId,
                                                                  paperId,
                                                                  "attributes",
                                                                  "");
    QStringList attributeList = attributes.split(";", Qt::SkipEmptyParts);

    if (!attributeList.contains(attribute)) {
        return SKRResult();
    }

    attributeList.removeAll(attribute);

    return plmdata->sheetPropertyHub()->setProperty(projectId,
                                                    paperId,
                                                    "attributes",
                                                    attributeList.join(";"));
}

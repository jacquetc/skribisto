/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmwritesubwindowmanager.cpp
*                                                  *
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
#include "plmwritesubwindowmanager.h"
#include "plmwritedocument.h"
#include "plmdata.h"

PLMWriteSubWindowManager::PLMWriteSubWindowManager(QBoxLayout *parentLayout) :
    PLMBaseSubWindowManager(parentLayout, "writeWindowManager")
{
    m_documentList = new PLMDocumentList(this, tableName());


    connect(plmpluginhub, &PLMPluginHub::commandSent, [this](const PLMCommand& command) {
        if (command.cmd ==
            "open_sheet") this->openSheet(command.projectId, command.paperId);

        if (command.cmd ==
            "open_sheet_on_new_view") this->openSheet(command.projectId, command.paperId,
                                                      false);
    });
}

QString PLMWriteSubWindowManager::tableName() const
{
    return "tbl_user_write_subwindow";
}

QString PLMWriteSubWindowManager::documentTableName() const
{
    return "tbl_user_write_doc_list";
}

void PLMWriteSubWindowManager::afterApplyUserSetting(int projectId)
{
    QList<int> documentIdList;
    PLMError   error = plmdata->userHub()->getIds(projectId,
                                                  documentTableName(),
                                                  documentIdList);

    if (error) {
        qDebug() << "error :" << Q_FUNC_INFO << "getIds";
        return;
    }


    QStringList valueList;
    valueList << "l_sheet_code" << "t_type" << "l_subwindow" << "b_visible" <<
        "l_cursor_pos" << "t_property" << "dt_updated" <<
        "dt_last_focused";

    for (int documentId : documentIdList) {
        QHash<QString, QVariant> result;
        PLMError error = plmdata->userHub()->getMultipleValues(projectId,
                                                               documentTableName(),
                                                               documentId,
                                                               valueList,
                                                               result);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO << "getMultipleValues";
            return;
        }

        // get document
        QString type =
            result.value("t_type", QVariant("write_document")).toString();
        int subWindowId = result.value("l_subwindow", QVariant(-1)).toInt();


        if (subWindowId == -1) {
            return;
        }

        // get Widget :
        PLMBaseSubWindow *subWindow = this->getSubWindowById(subWindowId);
        PLMBaseDocument  *document;

        // create document
        if (type == "write_document") {
            int sheetId   = result.value("l_sheet_code", QVariant(-1)).toInt();
            int cursorPos = result.value("l_cursor_pos", QVariant(0)).toInt();

            document = new PLMWriteDocument(projectId, documentId, m_documentList);

            PLMWriteDocument *writeDocument = static_cast<PLMWriteDocument *>(document);
            writeDocument->setTextDocument(projectId, sheetId);
            writeDocument->setCursorPosition(cursorPos);
        }
        else {
            return;
        }


        subWindow->addDocument(document);
    }

    // bring forth more recent document
}

PLMBaseDocument * PLMWriteSubWindowManager::getDocument(const QString& documentType)
{
    // TODO : add PLMBaseDocument-based plugin
}

// -------------------------------------------------------------------

void PLMWriteSubWindowManager::openSheet(int projectId, int sheetId, bool onNewView)
{
    int newId = -1;

    QHash<QString, QVariant> values;
    PLMError error = plmdata->userHub()->add(projectId,
                                             this->documentTableName(),
                                             values,
                                             newId);

    PLMWriteDocument *document = new PLMWriteDocument(projectId, newId, m_documentList);
    document->setTextDocument(projectId, sheetId);
    PLMBaseSubWindow *subWindow;

    if (onNewView) {
        int callerId  = this->getLastFocusedWindow()->id();
        int newViewId = this->addSubWindow(Qt::Horizontal, callerId);
        subWindow = this->getSubWindowById(newViewId);
    }
    else {
        subWindow = this->getLastFocusedWindow();
    }
    subWindow->addDocument(document);

    //    if (!m_windowManager->haveOneWindow("writeWindow")) {
    //        // create new window
    //        PLMBaseSubWindow *subWindow =
    // m_windowManager->addSubWindow("writeWindow",
    //
    //
}

// -------------------------------------------------------------------
///
/// \brief PLMBaseDocument::closeSheet
/// \param projectId
/// \param sheetId
/// close documents associated with the sheet
void PLMWriteSubWindowManager::closeSheet(int projectId, int sheetId)
{}

// -------------------------------------------------------------------
///
/// \brief PLMBaseDocument::closeDocument
/// \param projectId
/// \param documentId
/// close Document
void PLMWriteSubWindowManager::closeDocument(int projectId, int documentId)
{}

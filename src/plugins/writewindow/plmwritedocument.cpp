/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: writingwindow.cpp
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
#include "plmwritedocument.h"
#include "plmdata.h"
#include "ui_plmwritedocument.h"
#include <QTextDocument>

PLMWriteDocument::PLMWriteDocument(int projectId, int sheetId,
                                   int              documentId, const QString documentTableName,
                                   PLMTextDocumentList *textDocumentList) :
    PLMBaseDocument(projectId, documentId, "write_document", documentTableName),
    ui(new Ui::PLMWriteDocument), m_textDocumentList(textDocumentList)
{
    ui->setupUi(this);

    ui->writingZone->setHasMinimap(false);
    ui->writingZone->setIsResizable(true);

    connect(ui->writingZone, &PLMWritingZone::focused, [this]() {
        emit documentFocusActived(this->getDocumentId());
    });

    this->setupActions();

    this->setTextDocument(projectId, sheetId);
}

PLMWriteDocument::~PLMWriteDocument()
{
    delete ui;
}

void PLMWriteDocument::setTextDocument(int projectId, int sheetId)
{
    QTextDocument *textDocument = m_textDocumentList->getTextDocument(projectId,
                                                                      sheetId,
                                                                      this->getDocumentId());

    if (textDocument->isEmpty()) {
        textDocument->setHtml(plmdata->sheetHub()->getContent(projectId, sheetId));
    }
    ui->writingZone->setTextDocument(textDocument);

    m_textDocument = textDocument;



}

QTextDocument * PLMWriteDocument::getTextDocument() const
{
    return m_textDocument;
}

void PLMWriteDocument::setupActions()
{}


int PLMWriteDocument::getCursorPosition()
{
    return ui->writingZone->getCursorPosition();
}

void PLMWriteDocument::setCursorPosition(int value)
{
    ui->writingZone->setCursorPosition(value);
}

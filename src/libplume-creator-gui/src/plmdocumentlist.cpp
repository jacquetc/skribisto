/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdocumentlist.cpp
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
#include "plmdocumentlist.h"
#include <QDebug>

PLMDocumentList::PLMDocumentList(QObject *parent, const QString& tableName) : QObject(
        parent)
{}


QTextDocument * PLMDocumentList::getTextDocument(int projectId,
                                                 int paperId, int plmBaseDocumentId)
{
    QPair<int, int> wholePaperId(projectId, paperId);
    QPair<int, int> wholeDocId(projectId, plmBaseDocumentId);

    // get existing textDocument
    if (m_subscribedHash.contains(wholePaperId)) {
        m_subscribedHash.insert(wholePaperId, wholeDocId);

        return getTextDocumentFromPaperId(wholePaperId);
    }

    // else create textDocument

    QTextDocument *textDocument = new QTextDocument(this);
    textDocument->setProperty("projectId", projectId);
    textDocument->setProperty("paperId",   paperId);
    m_textDocumentList.append(textDocument);
    m_subscribedHash.insert(wholePaperId, wholeDocId);
}

bool            PLMDocumentList::contains(int projectId, int paperId)
{}

QTextDocument * PLMDocumentList::getTextDocumentFromPaperId(const QPair<int,
                                                                        int>& wholePaperId)
{
    for (QTextDocument *textDocument : m_textDocumentList) {
        if ((wholePaperId.first == textDocument->property("projectId").toInt()) &&
            (wholePaperId.second == textDocument->property("paperId").toInt())) {
            return textDocument;
        }
    }
    qDebug() << "error :" << Q_FUNC_INFO;
    return new QTextDocument();
}

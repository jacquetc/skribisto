/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename:                                                    *
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

#include "plmdocumentrepo.h"
#include <QModelIndex>
#include <QVariant>

PLMDocumentRepo::PLMDocumentRepo(QObject *parent, PLMTreeModel *model) :
    QObject(parent), m_model(model)
{

}

PLMDocumentRepo::~PLMDocumentRepo()
{
    qDeleteAll(m_textDocumentListForIdHash);
}


QTextDocument *PLMDocumentRepo::findDocument(int paperId)
{
    if(m_textDocumentListForIdHash.contains(paperId))
        return m_textDocumentListForIdHash.value(paperId);
    else {
        QTextDocument *document = new QTextDocument(this);
        document->setHtml(m_model->getContent(paperId).toString());
        document->setProperty("paperId", paperId);
        connect(document, &QTextDocument::contentsChanged, this, &PLMDocumentRepo::saveDocument);
        m_textDocumentListForIdHash.insert(paperId, document);
        return document;
    }

}
void PLMDocumentRepo::saveDocument()
{
    QTextDocument *document = static_cast<QTextDocument *>(this->sender());
    int paperId = document->property("paperId").toInt();
    m_model->setContent(paperId, document->toPlainText());
}

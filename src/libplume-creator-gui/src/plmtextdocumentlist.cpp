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
#include "plmtextdocumentlist.h"
#include <QDateTime>
#include <QDebug>

PLMTextDocumentList::PLMTextDocumentList(QObject       *parent) :
    QObject(parent)
{}


QTextDocument * PLMTextDocumentList::getTextDocument(int projectId,
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

    //save timer :
    QTimer *timer = new QTimer(textDocument);
    timer->callOnTimeout([=](){
        saveTextDocument(textDocument);
    });
    m_textDocumentAndTimerHash.insert(textDocument, timer);

    // save connection

    connect(textDocument, &QTextDocument::contentsChanged, this, &PLMTextDocumentList::contentsChanged, Qt::UniqueConnection);

    return textDocument;
}

//--------------------------------------------------------------------------------
void PLMTextDocumentList::contentsChanged()
{
    QTextDocument *textDocument = dynamic_cast<QTextDocument*>(this->sender());

    QTimer *timer = m_textDocumentAndTimerHash.value(textDocument);
    timer->stop();
    timer->start(1000);

}

//--------------------------------------------------------------------------------

void PLMTextDocumentList::saveAllTextsImmediately()
{
    QHashIterator<QTextDocument *, QTimer *> j(m_textDocumentAndTimerHash);
    while (j.hasNext()) {
        j.next();
        j.value()->stop();
        this->saveTextDocument(j.key());
    }
}

//--------------------------------------------------------------------------------

bool PLMTextDocumentList::contains(int projectId, int paperId)
{
    QPair<int, int> wholePaperId(projectId, paperId);
    return m_subscribedHash.contains(wholePaperId);
}

//--------------------------------------------------------------------------------

bool PLMTextDocumentList::unsubscibeBaseDocumentFromTextDocument(const QPair<int,
                                             int>& wholeDocId)
{
    QMutableHashIterator<QPair<int, int>, QPair<int, int> > i(m_subscribedHash);

    QPair<int, int> wholePaperId(-1, -1);
    while (i.hasNext()) {
        i.next();
        if(i.value() == wholeDocId){
            wholePaperId = i.key();
            i.remove();
        }
    }

    //remove text if no subscription

    if(!m_subscribedHash.contains(wholePaperId)){

        QMutableListIterator<QTextDocument *> j(m_textDocumentList);
        while (j.hasNext()) {
            j.next();
            if(j.value()->property("projectId").toInt() == wholePaperId.first &&
                    j.value()->property("paperId").toInt() == wholePaperId.second){
                this->saveTextDocument(j.value());
                j.remove();
            }
        }


   }

//    QTextDocument *textDocumentToRemove;
//    for (QTextDocument *textDocument : m_textDocumentList) {
//        if ((wholeDocId.first == textDocument->property("projectId").toInt()) &&
//            (wholeDocId.second == textDocument->property("paperId").toInt())) {
//            textDocumentToRemove = textDocument;
//        }
//    }
//    if(textDocumentToRemove == nullptr){
//        return false;
//    }
//    m_textDocumentList.removeOne(textDocumentToRemove);
//    return true;
}

QTextDocument * PLMTextDocumentList::getTextDocumentFromPaperId(const QPair<int,
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

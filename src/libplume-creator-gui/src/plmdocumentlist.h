/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdocumentlist.h
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
#ifndef PLMDOCUMENTLIST_H
#define PLMDOCUMENTLIST_H

#include <QObject>
#include <QTextDocument>

class PLMDocumentList : public QObject {
    Q_OBJECT

public:

    explicit PLMDocumentList(QObject       *parent,
                             const QString& tableName);

    QTextDocument* getTextDocument(int projectId,
                                   int paperId,
                                   int plmBaseDocumentId);
    bool           contains(int projectId,
                            int paperId);

signals:

public slots:

private:

    QTextDocument* getTextDocumentFromPaperId(const QPair<int, int>& wholePaperId);

    QList<QTextDocument *>m_textDocumentList;
    QMultiHash<QPair<int, int>, QPair<int, int> >m_subscribedHash;
};

#endif // PLMDOCUMENTLIST_H

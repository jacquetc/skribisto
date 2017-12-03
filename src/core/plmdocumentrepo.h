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

#ifndef PLMDOCUMENTREPO_H
#define PLMDOCUMENTREPO_H

#include <QObject>
#include <QTextDocument>
#include <QHash>
#include "models/plmtreemodel.h"

class PLMDocumentRepo : public QObject
{
    Q_OBJECT
public:
    explicit PLMDocumentRepo(QObject *parent, PLMTreeModel *model);
    ~PLMDocumentRepo();

    QTextDocument *findDocument(int paperId);


signals:

public slots:

private slots:
    void saveDocument();
private:
//    void addDocument(QTextDocument *document);
//    void removeDocument(QTextDocument *document);
    QHash<int, QTextDocument *> m_textDocumentListForIdHash;
    PLMTreeModel *m_model;

};

#endif // PLMDOCUMENTREPO_H

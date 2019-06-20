/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmwritesubwindowmanager.h
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
#ifndef PLMWRITESUBWINDOWMANAGER_H
#define PLMWRITESUBWINDOWMANAGER_H

#include "plmbasesubwindowmanager.h"
#include "plmwritetextdocumentlist.h"
#include <plmdocumentlistmodel.h>
#include "plmmodels.h"
#include <QObject>

class PLMWriteSubWindowManager : public PLMBaseSubWindowManager {
public:

    PLMWriteSubWindowManager(QBoxLayout *parentLayout);

    QString tableName() const;
    QString documentTableName() const;


    void    openSheet(int  projectId,
                      int  sheetId,
                      bool onNewView = false);
    void    closeSheet(int projectId,
                       int sheetId);

    void    closeDocument(int projectId,
                          int documentId);

protected:

private:

    void             afterApplyUserSetting(int projectId);
    PLMBaseDocument* getDocument(const QString& documentType);

    PLMTextDocumentList *m_textDocumentList;
};

#endif // PLMWRITESUBWINDOWMANAGER_H

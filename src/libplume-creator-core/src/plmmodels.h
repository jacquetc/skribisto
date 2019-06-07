/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmmodels.h                                                   *
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
#ifndef PLMMODELS_H
#define PLMMODELS_H

#include "plmdocumentlistmodel.h"
#include "plmsheetmodel.h"
#include "plmsheetproxymodel.h"
#include "global_core.h"

#include <QObject>

#define plmmodels PLMModels::instance()

class EXPORT_CORE PLMModels : public QObject {
    Q_OBJECT

public:

    explicit PLMModels(QObject *parent = nullptr);
    ~PLMModels();
    static PLMModels* instance()
    {
        return m_instance;
    }

    PLMSheetModel       * sheetModel();
    PLMSheetProxyModel  * sheetProxyModel();

    PLMDocumentListModel* documentsListModel();

signals:

public slots:

private:

    static PLMModels *m_instance;

    PLMSheetModel *m_sheetModel;
    PLMSheetProxyModel *m_sheetProxyModel;

    PLMDocumentListModel *m_documentsListModel;
};

#endif // PLMMODELS_H

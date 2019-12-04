/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmmodels.h                                                   *
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
#ifndef PLMMODELS_H
#define PLMMODELS_H

#include "plmwritedocumentlistmodel.h"
#include "plmsheetmodel.h"
#include "plmsheetproxymodel.h"
#include "./skribisto_data_global.h"

#include <QObject>

#define plmmodels PLMModels::instance()

class EXPORT PLMModels : public QObject {
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

    PLMDocumentListModel* writeDocumentListModel();

signals:

public slots:

private:

    static PLMModels *m_instance;

    PLMSheetModel *m_sheetModel;
    PLMSheetProxyModel *m_sheetProxyModel;

    PLMWriteDocumentListModel *m_writeDocumentListModel;
};

#endif // PLMMODELS_H

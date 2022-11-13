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
#ifndef SKRMODELS_H
#define SKRMODELS_H

#include "plmwritedocumentlistmodel.h"
#include "skrtreemodel.h"
#include "skrtreelistmodel.h"
#include "skrtaglistmodel.h"
#include "./skribisto_data_global.h"

#include <QObject>

#define skrmodels SKRModels::instance()

class EXPORT SKRModels : public QObject {
    Q_OBJECT

public:

    Q_DECL_DEPRECATED explicit SKRModels(QObject *parent = nullptr);
    ~SKRModels();
    static SKRModels* instance()
    {
        return m_instance;
    }

    Q_INVOKABLE PLMWriteDocumentListModel* writeDocumentListModel();

    SKRTreeModel                         * treeModel();
    SKRTreeListModel                     * treeListModel();
    SKRTagListModel                      * tagListModel();

signals:

public slots:

private:

    static SKRModels *m_instance;

    SKRTreeModel *m_treeModel;
    SKRTreeListModel *m_treeListModel;
    SKRTagListModel *m_tagListModel;

    PLMWriteDocumentListModel *m_writeDocumentListModel;
};

#endif // SKRMODELS_H

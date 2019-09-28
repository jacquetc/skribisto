/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmdata.h                                 *
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
#ifndef PLMDATA_H
#define PLMDATA_H

#include <QObject>

#include "plmerrorhub.h"
#include "plmnotehub.h"
#include "plmprojecthub.h"
#include "plmpropertyhub.h"
#include "plmsheethub.h"
#include "plmsignalhub.h"
#include "plmuserhub.h"
#include "plmpluginhub.h"
#include "skribisto_data_global.h"
#include "tasks/plmprojectmanager.h"

#define plmdata PLMData::instance()
#define plmpluginhub PLMData::instance()->pluginHub()


class EXPORT PLMData : public QObject {
    Q_OBJECT

public:

    explicit PLMData(QObject *parent = nullptr);
    ~PLMData();

    static PLMData* instance()
    {
        return m_instance;
    }

    PLMSignalHub             * signalHub();
    PLMErrorHub              * errorHub();
    Q_INVOKABLE PLMSheetHub  * sheetHub();
    PLMPropertyHub           * sheetPropertyHub();
    PLMNoteHub               * noteHub();
    PLMPropertyHub           * notePropertyHub();
    Q_INVOKABLE PLMProjectHub* projectHub();
    PLMUserHub               * userHub();
    PLMPluginHub             * pluginHub();

signals:

public slots:

private:

    static PLMData *m_instance;

    PLMErrorHub *m_errorHub;
    PLMSignalHub *m_signalHub;
    PLMProjectHub *m_projectHub;
    PLMSheetHub *m_sheetHub;
    PLMNoteHub *m_noteHub;
    PLMProjectManager *m_projectManager;
    PLMPropertyHub *m_notePropertyHub, *m_sheetPropertyHub;
    PLMUserHub *m_userHub;
    PLMPluginHub *m_pluginHub;
};

#endif // PLMDATA_H

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

#include "skrerrorhub.h"
#include "plmnotehub.h"
#include "plmprojecthub.h"
#include "plmpropertyhub.h"
#include "plmsheethub.h"
#include "skrtaghub.h"
#include "plmsignalhub.h"
#include "plmpluginhub.h"
#include "skrprojectdicthub.h"
#include "skribisto_data_global.h"
#include "skrstathub.h"
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

    PLMSignalHub                 * signalHub();
    Q_INVOKABLE SKRErrorHub      * errorHub();
    Q_INVOKABLE PLMSheetHub      * sheetHub();
    Q_INVOKABLE PLMPropertyHub   * sheetPropertyHub();
    Q_INVOKABLE PLMNoteHub       * noteHub();
    Q_INVOKABLE PLMPropertyHub   * notePropertyHub();
    Q_INVOKABLE PLMProjectHub    * projectHub();
    Q_INVOKABLE SKRTagHub        * tagHub();
    Q_INVOKABLE SKRProjectDictHub* projectDictHub();
    Q_INVOKABLE SKRStatHub       * statHub();
    PLMPluginHub                 * pluginHub();

signals:

public slots:

private:

    static PLMData *m_instance;

    SKRErrorHub *m_errorHub;
    PLMSignalHub *m_signalHub;
    PLMProjectHub *m_projectHub;
    PLMSheetHub *m_sheetHub;
    PLMNoteHub *m_noteHub;
    PLMProjectManager *m_projectManager;
    PLMPropertyHub *m_notePropertyHub, *m_sheetPropertyHub;
    SKRTagHub *m_tagHub;
    PLMPluginHub *m_pluginHub;
    SKRProjectDictHub *m_projectDictHub;
    SKRStatHub *m_statHub;
};

#endif // PLMDATA_H

/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrsettingspanelinterface.h
*                                                  *
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

#ifndef SKRSETTINGSPANELINTERFACE_H
#define SKRSETTINGSPANELINTERFACE_H

#include <QString>
#include "skrresult.h"
#include "skrcoreinterface.h"


class SKRSettingsPanelInterface : public SKRCoreInterface  {
public:

    virtual ~SKRSettingsPanelInterface() {}

    virtual QString settingsPanelQmlUrl() const     = 0;
    virtual QString settingsPanelText() const       = 0;
    virtual QString settingsPanelIconSource() const = 0;
    virtual int     weight() const {
        return 500;
    }
};

#define SKRSettingsPanelInterface_iid "com.skribisto.SettingsPanelInterface/1.0"


Q_DECLARE_INTERFACE(SKRSettingsPanelInterface, SKRSettingsPanelInterface_iid)

#endif // SKRSETTINGSPANELINTERFACE_H

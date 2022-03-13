/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plminterfacesettings.h
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
#ifndef SKRINTERFACESETTINGS_H
#define SKRINTERFACESETTINGS_H


#include <QSettings>
#include "skr.h"

class SKRInterfaceSettings {
public:

    virtual ~SKRInterfaceSettings() {}

    virtual QString name() const                 = 0;
    virtual QString displayedName() const        = 0;
    virtual QString use() const                  = 0;
    virtual QString pluginGroup() const          = 0;
    virtual QString pluginSelectionGroup() const = 0;

    bool            isPluginEnabledSettingExisting() const
    {
        QSettings settings;

        return settings.value("plugins/" + this->name() + "-enabled", QVariant()) == QVariant() ? false : true;
    }

    bool pluginEnabled() const
    {
        QSettings settings;

        return settings.value("plugins/" + this->name() + "-enabled", true).toBool();
    }

    void setPluginEnabled(bool pluginEnabled)
    {
        QSettings settings;

        return settings.setValue("plugins/" + this->name() + "-enabled", pluginEnabled);
    }
};

#endif // SKRINTERFACESETTINGS_H

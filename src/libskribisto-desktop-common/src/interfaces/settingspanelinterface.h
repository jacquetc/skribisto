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

#ifndef SETTINGSPANELINTERFACE_H
#define SETTINGSPANELINTERFACE_H

#include "skrresult.h"
#include "settingsPanel.h"
#include <QString>

class SettingsPanelInterface {
public:
  virtual ~SettingsPanelInterface() {}

  virtual QString name() const = 0;
    virtual SettingsPanel *settingsPanel() const = 0;
    virtual QString settingsGroup() const = 0;
  virtual QString settingsPanelButtonText() const = 0;
  virtual QString settingsPanelIconSource() const = 0;
  virtual int settingsPanelWeight() const { return 500; }
};

#define SettingsPanelInterface_iid "com.skribisto.SettingsPanelInterface/1.0"

Q_DECLARE_INTERFACE(SettingsPanelInterface, SettingsPanelInterface_iid)

#endif // SETTINGSPANELINTERFACE_H

/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrcoreinterface.h
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
#ifndef SKRCOREINTERFACE_H
#define SKRCOREINTERFACE_H


#include <QString>
#include "skrinterfacesettings.h"

class SKRCoreInterface : public SKRInterfaceSettings {
public:

    virtual ~SKRCoreInterface() {}
};

#define SKRCoreInterface_iid "com.skribisto.CoreInterface/1.0"


Q_DECLARE_INTERFACE(SKRCoreInterface, SKRCoreInterface_iid)

#endif // SKRCOREINTERFACE_H

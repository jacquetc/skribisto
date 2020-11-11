/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmcoreinterface.h
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
#ifndef PLMCOREINTERFACE_H
#define PLMCOREINTERFACE_H


#include <QString>
#include "plminterfacesettings.h"

class PLMBaseInterface : public PLMInterfaceSettings {

public:

    virtual ~PLMBaseInterface() {}
};

#define PLMBaseInterface_iid "com.Skribisto.BaseInterface/1.0"


Q_DECLARE_INTERFACE(PLMBaseInterface, PLMBaseInterface_iid)

#endif // PLMCOREINTERFACE_H

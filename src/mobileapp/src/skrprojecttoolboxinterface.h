/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpagetoolboxinterface.h
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

#ifndef SKRPROJECTTOOLBOXINTERFACE_H
#define SKRPROJECTTOOLBOXINTERFACE_H

#include <QString>
#include "skrresult.h"
#include "skrcoreinterface.h"


class SKRProjectToolboxInterface : public SKRCoreInterface  {
public:

    virtual ~SKRProjectToolboxInterface() {}

    virtual QString qmlUrl() const = 0;
    virtual int     weight() const {
        return 500;
    }
};

#define SKRProjectToolboxInterface_iid "com.skribisto.ProjectToolboxInterface/1.0"


Q_DECLARE_INTERFACE(SKRProjectToolboxInterface, SKRProjectToolboxInterface_iid)

#endif // SKRPROJECTTOOLBOXINTERFACE_H

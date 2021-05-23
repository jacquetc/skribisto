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

#ifndef SKRPAGETOOLBOXINTERFACE_H
#define SKRPAGETOOLBOXINTERFACE_H

#include <QString>
#include "skrresult.h"
#include "skrcoreinterface.h"


class SKRPageToolboxInterface : public SKRCoreInterface  {
public:

    virtual ~SKRPageToolboxInterface() {}

    virtual QStringList   associatedPageTypes() const = 0;
    virtual QString   qmlUrl() const         = 0;

};

#define SKRPageToolboxInterface_iid "com.skribisto.PageToolboxInterface/1.0"


Q_DECLARE_INTERFACE(SKRPageToolboxInterface, SKRPageToolboxInterface_iid)

#endif // SKRPAGETOOLBOXINTERFACE_H

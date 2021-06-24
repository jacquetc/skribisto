/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrimporterinterface.h
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

#ifndef SKRIMPORTERINTERFACE_H
#define SKRIMPORTERINTERFACE_H

#include <QString>
#include "skrresult.h"
#include "skrcoreinterface.h"


class SKRImporterInterface : public SKRCoreInterface  {
public:

    virtual ~SKRImporterInterface() {}


    virtual QString iconSource() const = 0;
    virtual QString buttonText() const = 0;
    virtual QString qmlUrl() const     = 0;
    virtual int     weight() const {
        return 500;
    }
};

#define SKRImporterInterface_iid "com.skribisto.ImporterInterface/1.0"


Q_DECLARE_INTERFACE(SKRImporterInterface, SKRImporterInterface_iid)

#endif // SKRIMPORTERINTERFACE_H

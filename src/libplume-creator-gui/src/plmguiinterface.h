/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmguiinterface.h
*                                                  *
*  This file is part of Plume Creator.                                    *
*                                                                         *
*  Plume Creator is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Plume Creator is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#ifndef PLMGUIINTERFACE_H
#define PLMGUIINTERFACE_H

#include "plmbasewindow.h"
#include "plmsidemainbar.h"
#include "plmbasedockwidget.h"
#include "plmcoreinterface.h"
#include "global.h"

#include <QString>

class EXPORT_GUI PLMWindowInterface : public PLMBaseInterface {
public:

    virtual ~PLMWindowInterface() {}

    virtual PLMBaseWindow* window() = 0;
    virtual void           init()   = 0;

    // virtual QList<PLMSideBarAction> mainBarActions(QObject *parent) = 0;
};

#define PLMWindowInterface_iid "com.PlumeSoft.Plume-Creator.WindowInterface/1.0"

Q_DECLARE_INTERFACE(PLMWindowInterface, PLMWindowInterface_iid)

class PLMSideMainBarIconInterface : public PLMBaseInterface {
public:

    virtual ~PLMSideMainBarIconInterface() {}

    virtual QList<PLMSideBarAction>sideMainBarActions(QObject *parent) = 0;
};

#define PLMSideMainBarIconInterface_iid \
    "com.PlumeSoft.Plume-Creator.SideMainBarIconInterface/1.0"

Q_DECLARE_INTERFACE(PLMSideMainBarIconInterface, PLMSideMainBarIconInterface_iid)


#endif // PLMGUIINTERFACE_H

/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: PLMDockWidgetInterface.h
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
#ifndef PLMDOCKWIDGETINTERFACE_H
#define PLMDOCKWIDGETINTERFACE_H

#include <QPointer>
#include <QWidget>
#include <QString>

#include "plmcoreinterface.h"

// #include "plminterfacesettings.h"
#include "plmbasedockwidget.h"
#include "global.h"


class EXPORT_GUI PLMDockWidgetInterface : public PLMBaseInterface /*: public
                                                          PLMInterfaceSettings
                                                        */
{
public:

    virtual ~PLMDockWidgetInterface() {}


    virtual QWidget               * dockHeaderWidget(QWidget *parent) = 0;
    virtual PLMBaseDockWidget     * dockBodyWidget(QWidget *parent)   = 0;
    virtual Qt::Edge                getEdges()                  = 0;
    virtual QString                 getParentWindowName() const = 0;
    virtual PLMDockWidgetInterface* clone() const               = 0;

protected:

    QPointer<QWidget>m_dockHeader;
    QPointer<PLMBaseDockWidget>m_dockBody;
    virtual void instanciate(QWidget *parent) = 0;
};

#define PLMDockWidgetInterface_iid \
    "com.PlumeSoft.Plume-Creator.DockWidgetInterface/1.0"

Q_DECLARE_INTERFACE(PLMDockWidgetInterface, PLMDockWidgetInterface_iid)


#endif // PLMDOCKWIDGETINTERFACE_H

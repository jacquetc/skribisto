/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsidemainbar.cpp                                                   *
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
#include "plmsidemainbar.h"
#include "plmpluginloader.h"
#include "plmguiinterface.h"
#include "ui_plmsidemainbar.h"

#include <QVBoxLayout>
#include <QToolButton>

PLMSideMainBar::PLMSideMainBar(QWidget *parent) : QWidget(parent),
    ui(new Ui::PLMSideMainBar)
{
    ui->setupUi(this);
    this->loadPlugins();
}

void PLMSideMainBar::loadPlugins()
{
    // plugins are already loaded in plmpluginloader
    QList<PLMPanelInterface *> pluginList = PLMPluginLoader::instance()->pluginsByType<PLMPanelInterface>();

    foreach (PLMPanelInterface *plugin, pluginList) {
        QList<PLMSideBarAction> actionList = plugin->mainBarActions(this);

        foreach (const PLMSideBarAction &sideBarAction, actionList) {
            QToolButton *button = new QToolButton(this);
            button->setDefaultAction(sideBarAction.action());
            button->setIconSize(QSize(48, 48));
            ui->verticalLayout->addWidget(button);
        }
    }
}





QString PLMSideBarAction::panelName() const
{
    return m_panelName;
}

void PLMSideBarAction::setPanelName(const QString &panelName)
{
    m_panelName = panelName;
}

QAction *PLMSideBarAction::action() const
{
    return m_action;
}

void PLMSideBarAction::setAction(QAction *action)
{
    m_action = action;
}

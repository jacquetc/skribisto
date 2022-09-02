/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrviewmanager.cpp
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
#include "skrviewmanager.h"
#include "skrdata.h"
#include "skrpageinterface.h"


SKRViewManager::SKRViewManager(QObject *parent) : QObject(parent)
{}

SKRWindowManager * SKRViewManager::windowManager() const
{
    return m_windowManager;
}

void SKRViewManager::setWindowManager(SKRWindowManager *windowManager)
{
    m_windowManager = windowManager;
    emit windowManagerChanged(windowManager);
}

QObject * SKRViewManager::rootWindow() const
{
    return m_rootWindow;
}

void SKRViewManager::setRootWindow(QObject *rootWindow)
{
    m_rootWindow = rootWindow;
    emit rootWindowChanged(rootWindow);
}

QUrl SKRViewManager::getQmlUrlFromPageType(const QString& pageType) const
{
    QUrl url;
    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            url = plugin->pageUrl();
        }
    }

    if (url.isEmpty()) {
        url = "EmptyPage.qml";
    }


    return url;
}

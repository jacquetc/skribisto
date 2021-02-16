/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrviewmanager.cpp                                                   *
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
#include "plmdata.h"
#include "skrpageinterface.h"


SKRViewManager::SKRViewManager(QObject *parent) : QObject(parent)
{


    //populateFromPlugins();
}

SKRWindowManager *SKRViewManager::windowManager() const
{
    return m_windowManager;
}

void SKRViewManager::setWindowManager(SKRWindowManager *windowManager)
{
    m_windowManager = windowManager;
    emit windowManagerChanged(windowManager);
}

QObject *SKRViewManager::rootWindow() const
{
    return m_rootWindow;
}

void SKRViewManager::setRootWindow(QObject *rootWindow)
{
    m_rootWindow = rootWindow;
    emit rootWindowChanged(rootWindow);
}


//---------------------------------------------------------------------------------

//void SKRViewManager::populateFromPlugins()
//{
//}
//---------------------------------------------------------------------------------

QUrl SKRViewManager::getQmlUrlFromPageType(const QString &pageType) const
{
    QUrl url;

    if(pageType == "PROJECT"){
        url = "qrc:///qml/ProjectPage/ProjectPage.qml";
    }
    else if(pageType == "SECTION"){
        url = "qrc:///qml/SectionPage/SectionPage.qml";
    }
    else if(pageType == "OVERVIEW"){
        url = "qrc:///qml/OverviewPage/OverviewPage.qml";
    }
    else if(pageType == "WELCOME"){
        url = "qrc:///qml/WelcomePage/WelcomePage.qml";
    }
    else if(pageType == "EXPORT"){
        url = "qrc:///qml/WelcomePage/ExporterPage.qml";
    }
    else if(pageType == "IMPORT"){
        url = "qrc:///qml/WelcomePage/ImporterPage.qml";
    }
    //    else if(pageType == "PRINT"){
    //        url = "qrc:///qml/WelcomePage/PrintPage.qml";
    //    }
    else if(pageType == "SETTINGS"){
        url = "qrc:///qml/WelcomePage/SettingsPage.qml";
    }
    else if(pageType == "NEWPROJECT"){
        url = "qrc:///qml/WelcomePage/NewProjectPage.qml";
    }
    else if(pageType == "HELP"){
        url = "qrc:///qml/WelcomePage/HelpPage.qml";
    }
    else {
        url = "qrc:///qml/EmptyPage.qml";
    }

    QList<SKRPageInterface *> pluginList = plmdata->pluginHub()->pluginsByType<SKRPageInterface>();
    for ( SKRPageInterface *plugin: qAsConst(pluginList)){
        if(pageType == plugin->pageType()){
            url = plugin->pageUrl();
        }
    }

    return url;

}

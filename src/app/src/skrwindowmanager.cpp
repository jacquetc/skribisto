/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrwindowmanager.cpp
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
#include "skrwindowmanager.h"

#include <QSettings>
#include <QtQml>

SKRWindowManager::SKRWindowManager(QObject *parent, QQmlApplicationEngine *engine,
                                   const QUrl& mainUrl) : QObject(parent), m_engine(engine), m_url(mainUrl)
{}

SKRWindowManager::~SKRWindowManager()
{}

void SKRWindowManager::retranslate()
{
    m_engine->retranslate();
}

int SKRWindowManager::subscribeWindow(QObject *windowObject)
{
    m_windowList.append(windowObject);
    return getWindowId(windowObject);

    emit windowIdsChanged();
}

void SKRWindowManager::unSubscribeWindow(QObject *windowObject)
{
    m_windowList.removeAll(windowObject);

    emit windowIdsChanged();
}

int SKRWindowManager::getWindowId(QObject *windowObject)
{
    return m_windowList.indexOf(windowObject);
}

int SKRWindowManager::getNumberOfWindows()
{
    return m_windowList.count();
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::deleteWindow(QObject *windowObject) {
    windowObject->deleteLater();
}

// -------------------------------------------------------------------------------------------
void SKRWindowManager::insertAdditionalProperty(const QString& key, const QVariant& value) {
    m_additionalPropertiesMap.insert(key, value);
}

// -------------------------------------------------------------------------------------------
void SKRWindowManager::insertAdditionalPropertyForViewManager(const QString& key, const QVariant& value) {
    m_additionalPropertiesForViewManagerMap.insert(key, value);
}

// -------------------------------------------------------------------------------------------


void SKRWindowManager::addEmptyWindow() {
    addWindow(-1, -1, "");
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::addWindow(int projectId, int treeItemId, const QString& pageType)
{
    int nextFreeWindowId = m_windowList.count();
    QSettings settings;

    settings.beginGroup("window_" + QString::number(nextFreeWindowId));
    int x          = settings.value("x", 0).toInt();
    int y          = settings.value("y", 0).toInt();
    int width      = settings.value("width", 1024).toInt();
    int height     = settings.value("height", 768).toInt();
    int visibility = settings.value("visibility", 1).toInt();

    settings.endGroup();

    QVariantMap propertiesMap =
    {   { "x",
        x                                                                                      },
        { "y",
          y                                                                                                                   },
        { "width",
          width                                                                                                                                                                                       },
        { "height",
          height                                                                                                                                                                                                                  },
        { "visibility",
          visibility                                                                                                                                                                                                                                                                                 },
        { "projectIdToBeLoaded",
          projectId                                                                                                                                                                                                                                                                                  },
        { "treeItemIdToBeLoaded",
          treeItemId                                                                                                                                                                                                                                                                                                           },
        { "pageTypeToBeLoaded",
          pageType                                                                                                                                                                                                                                                                                                                                           },
        { "windowId",
          nextFreeWindowId                                                                                                                                                                                                                                                                                                                                   } };

    QMapIterator<QString, QVariant> i(m_additionalPropertiesMap);

    while (i.hasNext()) {
        i.next();
        propertiesMap.insert(i.key(), i.value());
    }
    propertiesMap.insert("additionalPropertiesForViewManager", m_additionalPropertiesForViewManagerMap);

    m_engine->setInitialProperties(propertiesMap);
    m_engine->load(m_url);
    m_additionalPropertiesMap.clear();
    m_additionalPropertiesForViewManagerMap.clear();
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::addWindowForItemId(int projectId, int treeItemId) {
    addWindow(projectId, treeItemId);
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::addWindowForProjectIndependantPageType(const QString& pageType) {
    addWindow(-1, -1, pageType);
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::addWindowForProjectDependantPageType(int projectId, const QString& pageType)
{
    addWindow(projectId, -1, pageType);
}

// -------------------------------------------------------------------------------------------

void SKRWindowManager::restoreWindows() {
    QSettings settings;

    bool multipleWindows = settings.value("window/recoverMultipleWindowDispositionAtStart", true).toBool();

    if (multipleWindows) {
        int numberOfWindows = settings.value("window/numberOfWindows", 1).toInt();

        for (int i = 0; i < numberOfWindows; i++) {
            addEmptyWindow();
        }
    }
    else {
        addEmptyWindow();
    }
}

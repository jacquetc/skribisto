/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreemanager.cpp
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
#include "skrtreemanager.h"
#include "skrdata.h"
#include "skrpageinterface.h"
#include "skrpagetoolboxinterface.h"
#include "skrprojecttoolboxinterface.h"
#include "skrprojectpageinterface.h"

SKRTreeManager::SKRTreeManager(QObject *parent) : QObject(parent)
{
    skrdata->pluginHub()->addPluginType<SKRPageInterface>();
    skrdata->pluginHub()->addPluginType<SKRPageToolboxInterface>();
    skrdata->pluginHub()->addPluginType<SKRProjectToolboxInterface>();

    connect(skrdata->treeHub(), &SKRTreeHub::treeItemAdded, this, [this](int projectId, int treeItemId) {
        QString pageType = skrdata->treeHub()->getType(projectId, treeItemId);
        this->finaliseAfterCreationOfTreeItem(projectId, treeItemId, pageType);
    });


    connect(skrdata->projectHub(), &PLMProjectHub::projectLoaded, this, [this](int projectId) {
        this->updateAllCharAndWordCount(projectId);
    });
}

// ---------------------------------------------------------------------------------

QUrl SKRTreeManager::getIconUrlFromPageType(const QString& pageType, int projectId, int treeItemId) const
{
    QUrl url;


    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            url = plugin->pageTypeIconUrl(projectId, treeItemId);
        }
    }

    if (url.isEmpty()) {
        url = "qrc:///icons/backup/data-warning.svg";
    }

    return url;
}

// ---------------------------------------------------------------------------------

QStringList SKRTreeManager::getPageTypeList(bool constructibleOnly = true) const
{
    QStringList stringList;

    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();


    std::sort(pluginList.begin(), pluginList.end(),
              [](SKRPageInterface *plugin1, SKRPageInterface *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
              );


    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (constructibleOnly && !plugin->isConstructible()) {
            continue;
        }
        else {
            stringList << plugin->pageType();
        }
    }

    return stringList;
}

// ---------------------------------------------------------------------------------

QString SKRTreeManager::getPageTypeText(const QString& pageType) const
{
    QString text;

    if (pageType == "PROJECT") {
        text = tr("Project");
    }

    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            text = plugin->visualText();
        }
    }

    return text;
}

// ---------------------------------------------------------------------------------

QString SKRTreeManager::getPageDetailText(const QString& pageType) const
{
    QString text;

    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            text = plugin->pageDetailText();
            break;
        }
    }

    return text;
}

// ---------------------------------------------------------------------------------

void SKRTreeManager::updateCharAndWordCount(int projectId, int treeItemId, const QString& pageType, bool sameThread)
{
    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            plugin->updateCharAndWordCount(projectId, treeItemId, sameThread);
            break;
        }
    }
}

// ---------------------------------------------------------------------------------

void SKRTreeManager::updateAllCharAndWordCount(int projectId)
{
    QList<int> allIds = skrdata->treeHub()->getAllIds(projectId);

    for (int treeItemId : qAsConst(allIds)) {
        QString pageType = skrdata->treeHub()->getType(projectId, treeItemId);
        this->updateCharAndWordCount(projectId, treeItemId, pageType);
    }
}

// ---------------------------------------------------------------------------------


QStringList SKRTreeManager::findToolboxUrlsForPage(const QString& pageType) const
{
    QList<SKRPageToolboxInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageToolboxInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](SKRPageToolboxInterface *plugin1, SKRPageToolboxInterface *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
              );

    QStringList list;

    for (SKRPageToolboxInterface *plugin: qAsConst(pluginList)) {
        if (plugin->associatedPageTypes().contains(pageType)) {
            list << plugin->qmlUrl();
        }
    }


    return list;
}

// ---------------------------------------------------------------------------------


QStringList SKRTreeManager::findToolboxUrlsForProject() const
{
    QList<SKRProjectToolboxInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRProjectToolboxInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(),
              pluginList.end(),
              [](SKRProjectToolboxInterface *plugin1, SKRProjectToolboxInterface *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
              );

    QStringList list;

    for (SKRProjectToolboxInterface *plugin: qAsConst(pluginList)) {
        list << plugin->qmlUrl();
    }

    return list;
}

// ---------------------------------------------------------------------------------


QUrl SKRTreeManager::getCreationParametersQmlUrlFromPageType(const QString& pageType) const
{
    QUrl url;

    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            url = plugin->pageCreationParametersUrl();
        }
    }

    return url;
}

// ---------------------------------------------------------------------------------

void SKRTreeManager::setCreationParametersForPageType(const QString& pageType, const QVariantMap& parameters) const
{
    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            plugin->setPageCreationParameters(parameters);
            break;
        }
    }
}

// ---------------------------------------------------------------------------------

SKRResult SKRTreeManager::finaliseAfterCreationOfTreeItem(int projectId, int treeItemId, const QString& pageType)
{
    SKRResult result(this);

    QList<SKRPageInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRPageInterface>();

    for (SKRPageInterface *plugin: qAsConst(pluginList)) {
        if (pageType == plugin->pageType()) {
            result = plugin->finaliseAfterCreationOfTreeItem(projectId, treeItemId);
            break;
        }
    }

    return result;
}

// ---------------------------------------------------------------------------------

QStringList SKRTreeManager::findProjectPageNamesForLocation(const QString& location) const
{
    QList<SKRProjectPageInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRProjectPageInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](SKRProjectPageInterface *plugin1, SKRProjectPageInterface
                 *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
              );

    QStringList list;

    for (SKRProjectPageInterface *plugin: qAsConst(pluginList)) {
        if (plugin->locations().contains(location)) {
            list << plugin->name();
        }
    }


    return list;
}

// ---------------------------------------------------------------------------------

QString SKRTreeManager::findProjectPageIconSource(const QString& name) const
{
    QList<SKRProjectPageInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRProjectPageInterface>();


    QString icon;

    for (SKRProjectPageInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            icon = plugin->iconSource();
            break;
        }
    }


    return icon;
}

// ---------------------------------------------------------------------------

QString SKRTreeManager::findProjectPageShowButtonText(const QString& name) const
{
    QList<SKRProjectPageInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRProjectPageInterface>();


    QString text;

    for (SKRProjectPageInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            text = plugin->showButtonText();
            break;
        }
    }


    return text;
}

// ---------------------------------------------------------------------------

QString SKRTreeManager::findProjectPageType(const QString& name) const
{
    QList<SKRProjectPageInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRProjectPageInterface>();


    QString text;

    for (SKRProjectPageInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            text = plugin->pageType();
            break;
        }
    }


    return text;
}

// ---------------------------------------------------------------------------

QStringList SKRTreeManager::findProjectPageShortcutSequences(const QString& name) const
{
    QList<SKRProjectPageInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRProjectPageInterface>();


    QStringList list;

    for (SKRProjectPageInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            list << plugin->shortcutSequences();
            break;
        }
    }


    return list;
}

// ---------------------------------------------------------------------------

/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreemanager.h
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
#ifndef SKRTREEMANAGER_H
#define SKRTREEMANAGER_H

#include <QObject>
#include <QQmlComponent>
#include "skrresult.h"

class SKRTreeManager : public QObject {
    Q_OBJECT

public:

    explicit SKRTreeManager(QObject *parent = nullptr);
    Q_INVOKABLE QUrl        getIconUrlFromPageType(const QString& pageType,
                                                   int            projectI   = -1,
                                                   int            treeItemId = -1) const;
    Q_INVOKABLE QStringList getPageTypeList(bool constructibleOnly) const;
    Q_INVOKABLE QString     getPageTypeText(const QString& pageType) const;
    Q_INVOKABLE QString     getPageDetailText(const QString& pageType) const;
    Q_INVOKABLE void        updateCharAndWordCount(int            projectId,
                                                   int            treeItemId,
                                                   const QString& pageType,
                                                   bool           sameThread = false);
    Q_INVOKABLE void        updateAllCharAndWordCount(int projectId);
    Q_INVOKABLE QStringList findToolboxUrlsForPage(const QString& pageType) const;
    Q_INVOKABLE QStringList findToolboxUrlsForProject() const;
    Q_INVOKABLE QUrl        getCreationParametersQmlUrlFromPageType(const QString& pageType) const;
    Q_INVOKABLE void        setCreationParametersForPageType(const QString    & pageType,
                                                             const QVariantMap& parameters) const;

    Q_INVOKABLE QStringList findProjectPageNamesForLocation(const QString& location) const;
    Q_INVOKABLE QString     findProjectPageIconSource(const QString& name) const;

    Q_INVOKABLE QString     findProjectPageShowButtonText(const QString& name) const;
    Q_INVOKABLE QString     findProjectPageType(const QString& name) const;
    Q_INVOKABLE QStringList findProjectPageShortcutSequences(const QString& name) const;

private:

    Q_INVOKABLE SKRResult finaliseAfterCreationOfTreeItem(int            projectId,
                                                          int            treeItemId,
                                                          const QString& pageType);

signals:
};

#endif // SKRTREEMANAGER_H

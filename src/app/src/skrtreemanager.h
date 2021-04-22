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
#include "skrresult.h"

class SKRTreeManager : public QObject {
    Q_OBJECT

public:

    explicit SKRTreeManager(QObject *parent = nullptr);
    Q_INVOKABLE QUrl        getIconUrlFromPageType(const QString& pageType) const;
    Q_INVOKABLE QStringList getPageTypeList(bool constructibleOnly) const;
    Q_INVOKABLE QString     getPageTypeText(const QString& pageType) const;
    Q_INVOKABLE QString     getPageDetailText(const QString& pageType) const;
    Q_INVOKABLE void        updateCharAndWordCount(int            projectId,
                                                   int            treeItemId,
                                                   const QString& pageType,
                                                   bool           sameThread = false);
    Q_INVOKABLE void updateAllCharAndWordCount(int projectId);

private:

    Q_INVOKABLE SKRResult finaliseAfterCreationOfTreeItem(int            projectId,
                                                          int            treeItemId,
                                                          const QString& pageType);

signals:
};

#endif // SKRTREEMANAGER_H

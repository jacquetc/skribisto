/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrviewmanager.h
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
#ifndef SKRVIEWMANAGER_H
#define SKRVIEWMANAGER_H

#include "skrwindowmanager.h"

#include <QObject>
#include <QQmlEngine>

class SKRViewManager : public QObject {
    Q_OBJECT
    QML_ELEMENT

    Q_PROPERTY(SKRWindowManager * windowManager READ windowManager WRITE setWindowManager NOTIFY windowManagerChanged)
    Q_PROPERTY(QObject * rootWindow READ rootWindow WRITE setRootWindow NOTIFY rootWindowChanged)

public:

    explicit SKRViewManager(QObject *parent = nullptr);

    SKRWindowManager* windowManager() const;
    void              setWindowManager(SKRWindowManager *windowManager);

    QObject         * rootWindow() const;
    void              setRootWindow(QObject *rootWindow);

    Q_INVOKABLE QUrl  getQmlUrlFromPageType(const QString& pageType) const;

signals:

    void windowManagerChanged(SKRWindowManager *windowManager);
    void rootWindowChanged(QObject *rootWindow);

private:

    //    void populateFromPlugins();
    SKRWindowManager *m_windowManager;
    QObject *m_rootWindow;
};

#endif // SKRVIEWMANAGER_H

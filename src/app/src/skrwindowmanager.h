/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrwindowmanager.h
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
#ifndef SKRWINDOWMANAGER_H
#define SKRWINDOWMANAGER_H

#include <QObject>
#include <QQmlApplicationEngine>

class SKRWindowManager : public QObject {
    Q_OBJECT

public:

    explicit SKRWindowManager(QObject               *parent,
                              QQmlApplicationEngine *engine,
                              const QUrl           & mainUrl);
    ~SKRWindowManager();
    Q_INVOKABLE void retranslate();
    Q_INVOKABLE int  subscribeWindow(QObject *windowObject);
    Q_INVOKABLE void unSubscribeWindow(QObject *windowObject);
    Q_INVOKABLE int  getWindowId(QObject *windowObject);
    Q_INVOKABLE int  getNumberOfWindows();

    Q_INVOKABLE void deleteWindow(QObject *windowObject);
    void             addEmptyWindow();
    void             addWindow(int            projectId  = -1,
                               int            treeItemId = -1,
                               const QString& pageType   = "");
    void             restoreWindows();
    Q_INVOKABLE void addWindowForItemId(int projectId,
                                        int treeItemId);
    Q_INVOKABLE void addWindowForProjectIndependantPageType(const QString& pageType);
    Q_INVOKABLE void insertAdditionalProperty(const QString & key,
                                              const QVariant& value);
    Q_INVOKABLE void insertAdditionalPropertyForViewManager(const QString & key,
                                                            const QVariant& value);

signals:

    void windowIdsChanged();

private:

    QList<QObject *>m_windowList;
    QQmlApplicationEngine *m_engine;
    const QUrl& m_url;
    QVariantMap m_additionalPropertiesMap;
    QVariantMap m_additionalPropertiesForViewManagerMap;
};

#endif // SKRWINDOWMANAGER_H

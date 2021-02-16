/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: SKREditMenuSignalHub.h
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

#ifndef SKREDITMENUSIGNALHUB_H
#define SKREDITMENUSIGNALHUB_H

#include <QObject>

class SKREditMenuSignalHub : public QObject {
    Q_OBJECT

public:

    explicit SKREditMenuSignalHub(QObject *parent = nullptr);
    Q_INVOKABLE bool clearCutConnections();
    Q_INVOKABLE bool clearCopyConnections();
    Q_INVOKABLE bool clearPasteConnections();
    Q_INVOKABLE bool clearItalicConnections();
    Q_INVOKABLE bool clearBoldConnections();
    Q_INVOKABLE bool clearStrikeConnections();
    Q_INVOKABLE bool clearUnderlineConnections();
    Q_INVOKABLE void subscribe(const QString& objectName);
    Q_INVOKABLE void unsubscribe(const QString& objectName);
    Q_INVOKABLE bool isSubscribed(const QString& objectName);

signals:

    void cutActionTriggered();
    void copyActionTriggered();
    void pasteActionTriggered();
    void italicActionTriggered(bool isChecked);
    void boldActionTriggered(bool isChecked);
    void strikeActionTriggered(bool isChecked);
    void underlineActionTriggered(bool isChecked);

private:

    QStringList m_subscribedList;
};

#endif // SKREDITMENUSIGNALHUB_H

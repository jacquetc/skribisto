/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmmessagehandler.h                                                   *
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
#ifndef PLMMESSAGEHANDLER_H
#define PLMMESSAGEHANDLER_H

#include <QObject>
#include <QString>

class PLMMessageHandler : public QObject
{
    Q_OBJECT
public:
    explicit PLMMessageHandler(QObject *parent = 0);
    static PLMMessageHandler* instance(){
        return m_instance;
    }
    void sendMessage(const QString &message);
signals:
    void messageSent(const QString &message);

public slots:

private:
    static PLMMessageHandler* m_instance;

};

#endif // PLMMESSAGEHANDLER_H

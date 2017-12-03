/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtaskerror.h                                 *
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
#ifndef PLMTASKERROR_H
#define PLMTASKERROR_H

#include <QObject>


#define plmTaskError PLMTaskError::instance()

class PLMTaskError : public QObject
{
    Q_OBJECT
public:
    explicit PLMTaskError(QObject *parent);
    static PLMTaskError* instance(){
        return m_instance;
    }

signals:
    void errorSent(const QString &errorCode, const QString &origin, const QString &message);

public slots:
private:
    static PLMTaskError* m_instance;
};

#endif // PLMTASKERROR_H

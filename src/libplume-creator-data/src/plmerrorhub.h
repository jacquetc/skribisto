/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmerrorhub.h                                 *
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
#ifndef PLMERRORHUB_H
#define PLMERRORHUB_H

#include "plmerror.h"

#include <QObject>
//class PLMErrorHub;
//struct PLMError;

//struct PLMError
//{
//    explicit PLMError()
//    {

//    }
//    explicit PLMError(const QString &_errorCode, const QString &_origin, const QString &_message)
//    {
//        m_errorCode = _errorCode;
//        m_origin = _origin;
//        m_message = _message;
//    }

//    PLMError(const PLMError &other)
//    {
//        m_errorCode = other.errorCode();
//        m_origin = other.origin();
//        m_message = other.message();
//    }
//    ~PLMError()
//    {

//    }

//    QString errorCode() const
//    {
//        return m_errorCode;
//    }
//    QString origin() const
//    {
//        return m_origin;
//    }
//    QString message() const
//    {
//        return m_message;
//    }
//    QList<QString> getAll() const{
//        QList<QString> list;
//        list << m_errorCode << m_origin << m_message;
//        return list;
//    }
//private:
//    QString m_errorCode;
//    QString m_origin;
//    QString m_message;
//};
//Q_DECLARE_METATYPE(PLMError)

////--------------------------------------------------------------------------


class PLMErrorHub : public QObject
{
    Q_OBJECT
public:
    explicit PLMErrorHub(QObject *parent);
//    QList<QString> getlatestError() const;
//    QList<QString> getError(int index) const;
//    int count();
//    void clear();
signals:
//    void errorSent(const QString &errorCode, const QString &origin, const QString &message);
    //    void errorSent();
        void errorSent(PLMError error);
//public slots:

//    void addError(const QString &errorCode, const QString &origin, const QString &message);
//private:
//    //QList<PLMError> m_errorList;

};

#endif // PLMERRORHUB_H

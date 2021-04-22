/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmerrorhub.h                                 *
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
#ifndef SKRRESULTHUB_H
#define SKRRESULTHUB_H

#include "skrresult.h"

#include <QObject>
#include "skribisto_data_global.h"

// class SKRErrorHub;
// struct SKRResult;

// struct SKRResult
// {
//    explicit SKRResult()
//    {

//    }
//    explicit SKRResult(const QString &_errorCode, const QString &_origin,
// const
// QString &_message)
//    {
//        m_errorCode = _errorCode;
//        m_origin = _origin;
//        m_message = _message;
//    }

//    SKRResult(const SKRResult &other)
//    {
//        m_errorCode = other.errorCode();
//        m_origin = other.origin();
//        m_message = other.message();
//    }
//    ~SKRResult()²²²
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
// private:
//    QString m_errorCode;
//    QString m_origin;
//    QString m_message;
// };
// Q_DECLARE_METATYPE(SKRResult)

////--------------------------------------------------------------------------


class EXPORT SKRErrorHub : public QObject {
    Q_OBJECT

public:

    explicit SKRErrorHub(QObject *parent);

    //    QList<QString> getlatestError() const;
    //    QList<QString> getError(int index) const;
    //    int count();
    //    void clear();

public slots:

    void addError(SKRResult result);

signals:

    void sendNotification(int     resultType,
                          QString content);

    //    void errorSent(const QString &errorCode, const QString &origin, const
    // QString &message);
    //    void errorSent();

    // public slots:

    //    void addError(const QString &errorCode, const QString &origin, const
    // QString &message);
    // private:
    //    //QList<SKRResult> m_errorList;

private:

    QList<SKRResult>m_resultList;
};

#endif // SKRRESULTHUB_H

/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmerror.h                                                   *
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
#ifndef SKRRESULT_H
#define SKRRESULT_H

#include <QObject>
#include <QString>
#include <QVariantList>
#include "skribisto_data_global.h"

/**
 * To facilitate the result management
 */
#define IFKO(RESULT) \
    if (Q_UNLIKELY(RESULT))

/**
 * To facilitate the result management
 */
#define IFOK(RESULT) \
    if (Q_LIKELY(!RESULT))

/**
 * To facilitate the result management
 */
#define IFOKDO(RESULT, ACTION) \
    IFOK(RESULT) {             \
        RESULT = ACTION;       \
    }

struct EXPORT SKRResult
{
    Q_GADGET
    Q_PROPERTY(bool success READ isSuccess)
    Q_PROPERTY(QString errorCode READ getLastErrorCode)

public:

    enum Status {
        OK,
        Warning,
        Critical,
        Fatal
    };
    Q_ENUM(Status)

    explicit SKRResult();
    SKRResult(const QObject *object);
    SKRResult(const QString& className);
    SKRResult(const SKRResult& result);

    SKRResult(SKRResult::Status status,
              const QObject    *object,
              const QString   & errorCodeEnd);
    SKRResult(SKRResult::Status status,
              const QString   & className,
              const QString   & errorCodeEnd);

    bool                                        operator!() const;
    operator bool() const;
    Q_INVOKABLE SKRResult                     & operator=(const SKRResult& iResult);
    bool                                        operator==(const SKRResult& otherSKRResult) const;
    bool                                        operator!=(const SKRResult& otherSKRResult) const;

    Q_INVOKABLE bool                            isSuccess() const;

    Q_INVOKABLE bool                            containsErrorCode(const QString& value) const;
    Q_INVOKABLE bool                            containsErrorCodeDetail(const QString& value) const;
    Q_INVOKABLE QString                         getLastErrorCode() const;
    Q_INVOKABLE QStringList                     getErrorCodeList() const;

    Q_INVOKABLE QList<QHash<QString, QVariant> >getDataHashList() const;
    void                                        addData(const QString & key,
                                                        const QVariant& value);

    Q_INVOKABLE QVariant                        getData(const QString & key,
                                                        const QVariant& defaultValue) const;

    SKRResult::Status                           getStatus() const;

private:

    void addErrorCode(const QString& value);
    void setDataHashList(const  QList<QHash<QString, QVariant> >& dataHashList);
    void setErrorCodeList(const QStringList& value);
    void setStatus(const SKRResult::Status& status);

private:

    QStringList                     m_errorCodeList;
    QList<QHash<QString, QVariant> >m_dataHashList;
    SKRResult::Status               m_status;
};

#endif // SKRRESULT_H

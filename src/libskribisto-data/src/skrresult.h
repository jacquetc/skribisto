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
    Q_PROPERTY(bool success MEMBER m_success)
    Q_PROPERTY(QString errorCode READ getLastErrorCode)

public:
    enum Result{
        OK,
        Warning,
        Critical,
        Fatal
    };
    Q_ENUM(Result)

    explicit SKRResult();
    SKRResult(const SKRResult& result);
    Q_INVOKABLE bool                    operator!() const;
    Q_INVOKABLE                         operator bool() const;
    Q_INVOKABLE SKRResult              & operator=(const SKRResult& iResult);
    bool                                operator==(const SKRResult& otherSKRResult) const;
    bool                                operator!=(const SKRResult& otherSKRResult) const;

    Q_INVOKABLE void                    setSuccess(bool value);
    Q_INVOKABLE bool                    isSuccess() const;

    Q_INVOKABLE bool                    containsErrorCode(const QString& value) const;
    Q_INVOKABLE QString                 getLastErrorCode() const;
    Q_INVOKABLE QStringList             getErrorCodeList() const;
    void                                setErrorCodeList(const QStringList& value);
    Q_INVOKABLE void                    addErrorCode(const QString& value);

    Q_INVOKABLE QHash<QString, QVariant>getDataHash() const;
    void                                setDataHash(const QHash<QString, QVariant>& dataHash);
    void                                addData(const QString & key,
                                                const QVariant& value);

    Q_INVOKABLE QVariant                getData(const QString & key,
                                                const QVariant& defaultValue) const;

private:

    QStringList             m_errorCodeList;
    bool                    m_success;
    QHash<QString, QVariant>m_dataHash;
};

#endif // SKRRESULT_H

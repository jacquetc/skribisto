/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/
#include "skrresult.h"

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{
using namespace Qt::StringLiterals;

SKRResult::SKRResult() : m_status(SKRResult::OK)
{
    this->addErrorCode("OK"_L1);
}

SKRResult::SKRResult(const QObject *object) : m_status(SKRResult::OK)
{
    QString className = QString::fromLatin1(object->metaObject()->className());

    *this = SKRResult(className);
}

SKRResult::SKRResult(const QString &className) : m_status(SKRResult::OK)
{
    this->addErrorCode("OK_"_L1 + className.toUpper());
}

SKRResult::SKRResult(const SKRResult &result)
{
    m_status = result.getStatus();
    m_errorCodeList = result.getErrorCodeList();
    m_dataHashList = result.getDataHashList();
}

SKRResult::SKRResult(SKRResult::Status status, const QObject *object, const QString &errorCodeEnd)
{
    QString className = QString::fromLatin1(object->metaObject()->className());

    *this = SKRResult(status, className, errorCodeEnd);
}

SKRResult::SKRResult(SKRResult::Status status, const QString &className, const QString &errorCodeEnd)
{
    m_status = status;

    // whole errorCode:
    QString errorCode;

    switch (status)
    {
    case SKRResult::OK:
        errorCode = "OK_"_L1;
        break;

    case SKRResult::Warning:
        errorCode = "W_"_L1;
        break;

    case SKRResult::Critical:
        errorCode = "C_"_L1;
        break;

    case SKRResult::Fatal:
        errorCode = "F_"_L1;
        break;
    }

    errorCode += className.toUpper() + "__"_L1;

    errorCode += errorCodeEnd;

    this->addErrorCode(errorCode);
}

bool SKRResult::operator!() const
{
    return !isSuccess();
}

SKRResult::operator bool() const
{
    return isSuccess();
}

SKRResult &SKRResult::operator=(const SKRResult &iResult)
{
    if (Q_LIKELY(&iResult != this))
    {
        m_status = iResult.getStatus();
        m_errorCodeList.append(iResult.getErrorCodeList());
        m_dataHashList.append(iResult.getDataHashList());
    }

    if (iResult.getStatus() == SKRResult::Critical || iResult.getStatus() == SKRResult::Fatal)
    {
        qFatal("A SKRResult with status Critical or Fatal cannot be assigned to another SKRResult.");
    }

    return *this;
}

bool SKRResult::operator==(const SKRResult &otherSKRResult) const
{
    return m_status == otherSKRResult.getStatus() && m_errorCodeList == otherSKRResult.getErrorCodeList() &&
           m_dataHashList == otherSKRResult.getDataHashList();
}

bool SKRResult::operator!=(const SKRResult &otherSKRResult) const
{
    return m_status != otherSKRResult.getStatus() || m_errorCodeList != otherSKRResult.getErrorCodeList() ||
           m_dataHashList != otherSKRResult.getDataHashList();
}

bool SKRResult::isSuccess() const
{
    return m_status == SKRResult::OK;
}

bool SKRResult::containsErrorCode(const QString &value) const
{
    return m_errorCodeList.contains(value);
}

bool SKRResult::containsErrorCodeDetail(const QString &value) const
{
    for (const QString &codeString : m_errorCodeList)
    {
        if (codeString.split("__"_L1).last() == value)
        {
            return true;
        }
    }

    return false;
}

QString SKRResult::getLastErrorCode() const
{
    if (m_errorCodeList.isEmpty())
    {
        return ""_L1;
    }

    return m_errorCodeList.last();
}

QStringList SKRResult::getErrorCodeList() const
{
    return m_errorCodeList;
}

void SKRResult::clear()
{
    m_dataHashList.clear();
    m_errorCodeList.clear();
    m_status = SKRResult::OK;
}

void SKRResult::setErrorCodeList(const QStringList &value)
{
    m_errorCodeList = value;
}

SKRResult::Status SKRResult::getStatus() const
{
    return m_status;
}

void SKRResult::setStatus(const SKRResult::Status &status)
{
    m_status = status;
}

void SKRResult::addErrorCode(const QString &value)
{
    m_errorCodeList.append(value);

    QHash<QString, QVariant> emptyHash;

    m_dataHashList.append(emptyHash);
}

QList<QHash<QString, QVariant>> SKRResult::getDataHashList() const
{
    return m_dataHashList;
}

void SKRResult::setDataHashList(const QList<QHash<QString, QVariant>> &dataHashList)
{
    m_dataHashList = dataHashList;
}

void SKRResult::addData(const QString &key, const QVariant &value)
{
    QHash<QString, QVariant> lastHash = m_dataHashList.takeLast();

    lastHash.insert(key, value);
    m_dataHashList.append(lastHash);
}

QVariant SKRResult::getData(const QString &key, const QVariant &defaultValue) const
{
    QVariant final = -2;

    QList<QHash<QString, QVariant>>::const_reverse_iterator i;

    for (i = m_dataHashList.crbegin(); i != m_dataHashList.crend(); ++i)
    {
        QHash<QString, QVariant> hash = *i;
        final = hash.value(key, -2);

        if (final != -2)
        {
            break;
        }
    }

    if (final == -2)
    {
        final = defaultValue;
    }

    return final;
}
} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
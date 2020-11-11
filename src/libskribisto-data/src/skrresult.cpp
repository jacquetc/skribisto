/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmresult.cpp                                                   *
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
#include "skrresult.h"

SKRResult::SKRResult() :  m_success(true)
{}

SKRResult::SKRResult(const SKRResult& result)
{
    m_success       = result.isSuccess();
    m_errorCodeList = result.getErrorCodeList();
    m_dataHash      = result.getDataHash();
}

bool SKRResult::operator!() const
{
    return isSuccess();
}

SKRResult::operator bool() const
{
    return !isSuccess();
}

SKRResult& SKRResult::operator=(const SKRResult& iResult)
{
    if (Q_LIKELY(&iResult != this)) {
        m_success = iResult.isSuccess();
        m_errorCodeList.append(iResult.getErrorCodeList());
        m_dataHash.insert(iResult.getDataHash());
    }

    // m_success = iError.isSuccess();
    return *this;
}

bool SKRResult::operator==(const SKRResult& otherSKRResult) const {
    return m_success == otherSKRResult.isSuccess()
           && m_errorCodeList == otherSKRResult.getErrorCodeList()
           && m_dataHash == otherSKRResult.getDataHash()
    ;
}

bool SKRResult::operator!=(const SKRResult& otherSKRResult) const {
    return m_success != otherSKRResult.isSuccess()
           || m_errorCodeList != otherSKRResult.getErrorCodeList()
           || m_dataHash != otherSKRResult.getDataHash()
    ;
}

void SKRResult::setSuccess(bool value)
{
    m_success = value;
}

bool SKRResult::isSuccess() const
{
    return m_success;
}

bool SKRResult::containsErrorCode(const QString& value) const
{
    return m_errorCodeList.contains(value);
}

QString SKRResult::getLastErrorCode() const
{
    if (m_errorCodeList.isEmpty()) {
        return "";
    }

    return m_errorCodeList.last();
}

QStringList SKRResult::getErrorCodeList() const
{
    return m_errorCodeList;
}

void SKRResult::setErrorCodeList(const QStringList& value)
{
    m_errorCodeList = value;
}

void SKRResult::addErrorCode(const QString& value)
{
    m_errorCodeList.append(value);
}

QHash<QString, QVariant>SKRResult::getDataHash() const
{
    return m_dataHash;
}

void SKRResult::setDataHash(const QHash<QString, QVariant>& dataHash)
{
    m_dataHash = dataHash;
}

void SKRResult::addData(const QString& key, const QVariant& value)
{
    m_dataHash.insert(key, value);
}

QVariant SKRResult::getData(const QString& key, const QVariant& defaultValue) const
{
    return m_dataHash.value(key, defaultValue);
}

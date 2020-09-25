/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmerror.cpp                                                   *
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
#include "plmerror.h"

PLMError::PLMError() :  m_success(true), m_errorCode("")
{}

PLMError::PLMError(const PLMError& error)
{
    m_success   = error.isSuccess();
    m_errorCode = error.getErrorCode();
    m_dataList  = error.getDataList();
}

bool PLMError::operator!() const
{
    return isSuccess();
}

PLMError::operator bool() const
{
    return !isSuccess();
}

PLMError& PLMError::operator=(const PLMError& iError)
{
    if (Q_LIKELY(&iError != this)) {
        m_success   = iError.isSuccess();
        m_errorCode = iError.getErrorCode();
        m_dataList  = iError.getDataList();
    }

    // m_success = iError.isSuccess();
    return *this;
}

void PLMError::setSuccess(bool value)
{
    m_success = value;
}

bool PLMError::isSuccess() const
{
    return m_success;
}

QString PLMError::getErrorCode() const
{
    return m_errorCode;
}

void PLMError::setErrorCode(const QString& value)
{
    m_errorCode = value;
}

QVariantList PLMError::getDataList() const
{
    return m_dataList;
}

void PLMError::setDataList(const QVariantList& dataList)
{
    m_dataList = dataList;
}

void PLMError::addData(const QVariant& value)
{
    m_dataList.append(value);
}

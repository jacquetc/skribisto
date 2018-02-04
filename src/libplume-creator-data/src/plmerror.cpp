/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmerror.cpp                                                   *
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
#include "plmerror.h"

PLMError::PLMError() :  m_success(true)
{

}

PLMError::PLMError(const PLMError& error)
{
    m_success = error.isSuccess();

}

bool PLMError::operator!() const
{
    return isSuccess();
}

PLMError::operator bool() const
{
    return !isSuccess();
}

PLMError& PLMError::operator= (const PLMError& iError)
{
    if (Q_LIKELY(&iError != this)) {

        m_success = iError.isSuccess();

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

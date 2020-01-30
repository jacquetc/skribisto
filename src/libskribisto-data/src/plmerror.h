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
#ifndef PLMERROR_H
#define PLMERROR_H

#include <QObject>
#include "skribisto_data_global.h"

/**
 * To facilitate the error management
 */
#define IFKO(ERROR) \
        if (Q_UNLIKELY(ERROR))

/**
 * To facilitate the error management
 */
#define IFOK(ERROR) \
        if (Q_LIKELY(!ERROR))

/**
 * To facilitate the error management
 */
#define IFOKDO(ERROR, ACTION) \
        IFOK(ERROR) {ERROR = ACTION; }

struct EXPORT PLMError
{
    Q_GADGET
    Q_PROPERTY(bool success MEMBER m_success)

public:
    explicit PLMError();
    PLMError(const PLMError &error);
    Q_INVOKABLE bool operator !() const;
    Q_INVOKABLE operator bool() const;
    Q_INVOKABLE PLMError &operator =(const PLMError &iError);

    Q_INVOKABLE void setSuccess(bool value);
    Q_INVOKABLE bool isSuccess() const;

signals:

public slots:

private:
    bool m_success;
};

#endif // PLMERROR_H

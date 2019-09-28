/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: upgrader.cpp                                                   *
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
#include "plmupgrader.h"

PLMUpgrader::PLMUpgrader(QObject *parent) : QObject(parent)
{

}

PLMError PLMUpgrader::upgradeSQLite(QSqlDatabase sqlDb)
{
    Q_UNUSED(sqlDb);

PLMError error;
error.setSuccess(true);

// from 1.5 to 1.6


return error;
}

PLMError PLMUpgrader::upgradeUserSQLite(QSqlDatabase sqlDb)
{
    Q_UNUSED(sqlDb);


PLMError error;
error.setSuccess(true);
// from 1.5 to 1.6


return error;
}

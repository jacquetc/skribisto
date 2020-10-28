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

#include <QSqlQuery>

PLMUpgrader::PLMUpgrader(QObject *parent) : QObject(parent)
{}

PLMError PLMUpgrader::upgradeSQLite(QSqlDatabase sqlDb)
{
    PLMError error;

    //find DB version :
    double dbVersion = -1;

    QSqlQuery query(sqlDb);
    QString   queryStr = "SELECT dbl_database_version FROM tbl_project";


    query.prepare(queryStr);
    query.exec();

    while (query.next()) {
        dbVersion = query.value(0).toDouble();
    }
    if(dbVersion == -1){
        error.setSuccess(false);
        error.setErrorCode("E_UPGRADER_no_version_found");
        return error;
    }


    // from 1.0 to 1.1
    if(dbVersion == 1.0){
        double newDbVersion = 1.1;

        sqlDb.transaction();

        QSqlQuery query(sqlDb);
        QString   queryStr = "CREATE TABLE tbl_project_dict (l_project_dict_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, t_word TEXT UNIQUE ON CONFLICT REPLACE NOT NULL ON CONFLICT ROLLBACK);"
                ;
        query.prepare(queryStr);
        query.exec() ? error.setSuccess(true) : error.setSuccess(false);

        sqlDb.commit();

        IFKO(error){
            error.setErrorCode("E_UPGRADER_cant_upgrade");
            error.addData("dbVersion", newDbVersion);

        }

        IFOKDO(error, error = PLMUpgrader::setDbVersion(sqlDb, newDbVersion));

        IFOK(error){
            dbVersion = newDbVersion;
        }


    }

    // from 1.1 to 1.2
    if(dbVersion == 1.1){
        double newDbVersion = 1.2;
        //fill here
    }

    return error;
}

PLMError PLMUpgrader::setDbVersion(QSqlDatabase sqlDb, double newVersion){


    PLMError error;

    sqlDb.transaction();
    QSqlQuery query(sqlDb);
    QString   queryStr = "UPDATE tbl_project SET dbl_database_version = :newVersion;";
    query.prepare(queryStr);
    query.bindValue(":newVersion", newVersion);
    query.exec() ? error.setSuccess(true) : error.setSuccess(false);

    // update date
    IFOK(error){
        queryStr = "UPDATE tbl_project SET dt_updated = CURRENT_TIMESTAMP;";
        query.prepare(queryStr);
        query.exec() ? error.setSuccess(true) : error.setSuccess(false);

    }


    sqlDb.commit();

    return error;
}

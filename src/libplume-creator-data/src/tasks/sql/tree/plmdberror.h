/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmdberror.h                                                   *
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
#ifndef PLMDBERROR_H
#define PLMDBERROR_H

#include <QString>

struct PLMDbError
{


public:
    enum ReturnError {
        Error=0,                     //Error return - check get_status(). MUST BE ZERO e.g. get_sheet_order
        OK=1                         //Success return (some functions return another non zero value e.g. a sheet_id)
    };

    enum Error {
        InvalidType=1,               //Invalid parameter type.
        InvalidParameter=2,          //Invalid parameter value.
        InternalError=3,             //Internal logic error or unexpected error
        RequestedRecordNotFound=4    //Requested record not found
    };


    PLMDbError()
    {
        status = 0;
        parameter = 0;
        errorMsg =  "";
    }

    QString getStatus() const
    {
        QString s_msg;
        if(status == InvalidType)
            s_msg = QString("Invalid parameter type (parameter %1)").arg(QString::number(parameter));
        else if(status == InvalidParameter)
            s_msg = QString("Invalid parameter value (parameter %1)").arg(QString::number(parameter));
                    else if(status == InternalError)
            s_msg = "Internal logic error";
        else
            s_msg = "Unhandled error";

        if(errorMsg != "")
            s_msg += ". " + errorMsg;

        return QString("Error: %1. ").arg(QString::number(status)) + s_msg;
    }

    void setStatus(int status, int parameter, const QString &errorMsg)
    {
        this->status = status;
        this->parameter = parameter;
        this->errorMsg = errorMsg;
    }

    int status, parameter;
    QString errorMsg;

};

#endif // PLMDBERROR_H

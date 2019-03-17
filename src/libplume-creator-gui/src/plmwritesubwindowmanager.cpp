/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmwritesubwindowmanager.cpp
*                                                  *
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
#include "plmwritesubwindowmanager.h"
#include "plmwritingwindow.h"

PLMWriteSubWindowManager::PLMWriteSubWindowManager(QBoxLayout *parentLayout) :
    PLMBaseSubWindowManager(parentLayout, "writeWindowManager")
{}

PLMBaseSubWindow * PLMWriteSubWindowManager::subWindowByName(const QString& subWindowType,
                                                             int            id)
{
    if (subWindowType == "writeWindow") return new PLMWritingWindow(id);

    return new PLMBaseSubWindow();
}

QString PLMWriteSubWindowManager::tableName() const
{
    return "tbl_user_write_subwindow";
}

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

#pragma once
#include <QString>
#include <qobjectdefs.h>

namespace Skribisto::WorkManagement
{

struct LoadWorkDto
{
    Q_GADGET
    Q_PROPERTY(QString fileName MEMBER fileName)

  public:
    QString fileName;
    LoadWorkDto() = default;
    ~LoadWorkDto() = default;
    LoadWorkDto(const LoadWorkDto &) = default;
    LoadWorkDto &operator=(const LoadWorkDto &) = default;
    explicit LoadWorkDto(const QString &fileName) : fileName(fileName)
    {
    }
};

struct SaveWorkDto
{
    Q_GADGET
    Q_PROPERTY(QString fileName MEMBER fileName)
    Q_PROPERTY(bool overwrite MEMBER overwrite)

  public:
    QString fileName;
    bool overwrite = false;
    SaveWorkDto() = default;
    ~SaveWorkDto() = default;
    SaveWorkDto(const SaveWorkDto &) = default;
    SaveWorkDto &operator=(const SaveWorkDto &) = default;
    SaveWorkDto(const QString &fileName, bool overwrite = false) : fileName(fileName), overwrite(overwrite)
    {
    }
};
} // namespace Skribisto::WorkManagement
Q_DECLARE_METATYPE(Skribisto::WorkManagement::LoadWorkDto)
Q_DECLARE_METATYPE(Skribisto::WorkManagement::SaveWorkDto)

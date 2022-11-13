/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skribformat.cpp                                                   *
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
#include "skribformat.h"


SkribFormat::SkribFormat(QObject *parent) : QObject(parent) {}

// ---------------------------------------------------

SkribFormat::~SkribFormat() {}

QString SkribFormat::formatDetailText() const
{
    return tr("Using the .skrib format, the project will be saved in one unique file. Behind the hood, this file is classic SQLite, an open and free database format.");
}

QString SkribFormat::finalFileName(const QString &path, const QString &fileBaseName) const
{
    return path + "/" + fileBaseName + ".skrib";
}

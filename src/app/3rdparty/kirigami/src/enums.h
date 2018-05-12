/*
*   Copyright (C) 2016 by Marco Martin <mart@kde.org>
*
*   This program is free software; you can redistribute it and/or modify
*   it under the terms of the GNU Library General Public License as
*   published by the Free Software Foundation; either version 2, or
*   (at your option) any later version.
*
*   This program is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU Library General Public License for more details
*
*   You should have received a copy of the GNU Library General Public
*   License along with this program; if not, write to the
*   Free Software Foundation, Inc.,
*   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

#ifndef ENUMS_H
#define ENUMS_H

#include <QObject>

class ApplicationHeaderStyle : public QObject
{
    Q_OBJECT
    Q_ENUMS(Status)

public:
    enum Status {
        Auto = 0,
        Breadcrumb,
        Titles,
        TabBar
    };
};

class MessageType : public QObject
{
    Q_OBJECT
    Q_ENUMS(Type)

public:
    enum Type {
        Information = 0,
        Positive,
        Warning,
        Error
    };
};

#endif // ENUMS_H

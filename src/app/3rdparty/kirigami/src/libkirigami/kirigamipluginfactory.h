/*
*   Copyright (C) 2017 by Marco Martin <mart@kde.org>
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

#ifndef KIRIGAMIPLUGINFACTORY_H
#define KIRIGAMIPLUGINFACTORY_H

#include "platformtheme.h"
#include <QObject>

#ifndef KIRIGAMI_BUILD_TYPE_STATIC
#include <kirigami2_export.h>
#endif

namespace Kirigami {

/**
 * @class KirigamiPluginFactory kirigamipluginfactory.h KirigamiPluginFactory
 *
 * This class is reimpleented by plugins to provide different implementations
 * of PlatformTheme
 */
#ifdef KIRIGAMI_BUILD_TYPE_STATIC
class KirigamiPluginFactory : public QObject
#else
class KIRIGAMI2_EXPORT KirigamiPluginFactory : public QObject
#endif
{
    Q_OBJECT

public:
    explicit KirigamiPluginFactory(QObject *parent = nullptr);
    ~KirigamiPluginFactory();

    /**
     * Creates an instance of PlatformTheme which can come out from
     * an implementation provided by a plugin
     * @param parent the parent object of the created PlatformTheme
     */
    virtual PlatformTheme *createPlatformTheme(QObject *parent) = 0;
};

}

QT_BEGIN_NAMESPACE
#define KirigamiPluginFactory_iid "org.kde.kirigami.KirigamiPluginFactory"
Q_DECLARE_INTERFACE(Kirigami::KirigamiPluginFactory, KirigamiPluginFactory_iid)
QT_END_NAMESPACE

#endif //KIRIGAMIPLUGINFACTORY_H

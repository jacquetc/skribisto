/*
 *   Copyright 2009 by Alan Alpert <alan.alpert@nokia.com>
 *   Copyright 2010 by Ménard Alexis <menard@kde.org>
 *   Copyright 2010 by Marco Martin <mart@kde.org>

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

#ifndef MOBILECOMPONENTSPLUGIN_H
#define MOBILECOMPONENTSPLUGIN_H

#include <QUrl>

#include <QQmlEngine>
#include <QQmlExtensionPlugin>

class KirigamiPlugin : public QQmlExtensionPlugin
{
    Q_OBJECT
    Q_PLUGIN_METADATA(IID "org.qt-project.Qt.QQmlExtensionInterface")

public:
    void registerTypes(const char *uri) Q_DECL_OVERRIDE;

#ifdef KIRIGAMI_BUILD_TYPE_STATIC
    static KirigamiPlugin& getInstance()
    {
         static KirigamiPlugin instance;
         return instance;
    }

    static void registerTypes()
    {
        static KirigamiPlugin instance;
        instance.registerTypes("org.kde.kirigami");
    }
#endif

private:
    QUrl componentUrl(const QString &fileName) const;
    QString resolveFilePath(const QString &path) const
    {
#ifdef KIRIGAMI_BUILD_TYPE_STATIC
        return QStringLiteral(":/org/kde/kirigami/") + path;
#else
        return baseUrl().toLocalFile() + QLatin1Char('/') + path;
#endif
    }
    QString resolveFileUrl(const QString &filePath) const
    {
#ifdef KIRIGAMI_BUILD_TYPE_STATIC
        return filePath;
#else
        return baseUrl().toString() + QLatin1Char('/') + filePath;
#endif
    }
    QStringList m_stylesFallbackChain;
};

#endif

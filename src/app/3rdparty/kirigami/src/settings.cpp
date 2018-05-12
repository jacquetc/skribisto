/*
 *   Copyright 2016 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

#include "settings.h"

#include <QDebug>
#include <QStandardPaths>
#include <QSettings>
#include <QFile>

#include "libkirigami/tabletmodewatcher.h"

Settings::Settings(QObject *parent)
    : QObject(parent)
{
    m_tabletModeAvailable = Kirigami::TabletModeWatcher::self()->isTabletModeAvailable();
    connect(Kirigami::TabletModeWatcher::self(), &Kirigami::TabletModeWatcher::tabletModeAvailableChanged,
            this, [this](bool tabletModeAvailable) {
                setTabletModeAvailable(tabletModeAvailable);
            });

    m_tabletMode = Kirigami::TabletModeWatcher::self()->isTabletMode();
    connect(Kirigami::TabletModeWatcher::self(), &Kirigami::TabletModeWatcher::tabletModeChanged,
            this, [this](bool tabletMode) {
                setTabletMode(tabletMode);
            });

#if defined(Q_OS_ANDROID) || defined(Q_OS_IOS)
    m_mobile = true;
#else
    //Mostly for debug purposes and for platforms which are always mobile,
    //such as Plasma Mobile
    if (qEnvironmentVariableIsSet("QT_QUICK_CONTROLS_MOBILE")) {
        m_mobile = (QString::fromLatin1(qgetenv("QT_QUICK_CONTROLS_MOBILE")) == QStringLiteral("1") ||
            QString::fromLatin1(qgetenv("QT_QUICK_CONTROLS_MOBILE")) == QStringLiteral("true"));
    } else {
        m_mobile = false;
    }
#endif

    const QString configPath = QStandardPaths::locate(QStandardPaths::ConfigLocation, QStringLiteral("kdeglobals"));
    if (QFile::exists(configPath)) {
        QSettings globals(configPath, QSettings::IniFormat);
        globals.beginGroup(QStringLiteral("KDE"));
        m_scrollLines = qMax(1, globals.value(QStringLiteral("WheelScrollLines"), 3).toInt());
    } else {
        m_scrollLines = 3;
    }
}


Settings::~Settings()
{
}

void Settings::setTabletModeAvailable(bool mobileAvailable)
{
    if (mobileAvailable == m_tabletModeAvailable) {
        return;
    }

    m_tabletModeAvailable = mobileAvailable;
    emit tabletModeAvailableChanged();
}

bool Settings::isTabletModeAvailable() const
{
    return m_tabletModeAvailable;
}

void Settings::setIsMobile(bool mobile)
{
    if (mobile == m_mobile) {
        return;
    }

    m_mobile = mobile;
    emit isMobileChanged();
}

bool Settings::isMobile() const
{
    return m_mobile;
}

void Settings::setTabletMode(bool tablet)
{
    if (tablet == m_tabletMode) {
        return;
    }

    m_tabletMode = tablet;
    emit tabletModeChanged();
}

bool Settings::tabletMode() const
{
    return m_tabletMode;
}

QString Settings::style() const
{
    return m_style;
}

void Settings::setStyle(const QString &style)
{
    m_style = style;
}

int Settings::mouseWheelScrollLines() const
{
    return m_scrollLines;
}

#include "moc_settings.cpp"


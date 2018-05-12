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
#ifndef SETTINGS_H
#define SETTINGS_H

#include <QObject>

class Settings : public QObject
{
    Q_OBJECT

    /**
     * True if the system can dynamically enter in tablet mode
     * (or the device is actually a tablet).
     * such as transformable laptops that support keyboard detachment
     */
    Q_PROPERTY(bool tabletModeAvailable READ isTabletModeAvailable NOTIFY tabletModeAvailableChanged)

    /**
     * True if we are running on a small mobile device such as a mobile phone
     * This is used when we want to do specific adaptations to our UI for
     * small screen form factors, such as having bigger touch areas.
     */
    Q_PROPERTY(bool isMobile READ isMobile NOTIFY isMobileChanged)

    /**
     * True if the device we are running on is behaving like a tablet:
     * Note that this doesn't mean exactly a tablet form factor, but
     * that the preferred input mode for the device is the touch screen
     * and that pointer and keyboard are either secondary or not available.
     */
    Q_PROPERTY(bool tabletMode READ tabletMode NOTIFY tabletModeChanged)

    /**
     * name of the QtQuickControls2 style we are using,
     * for instance org.kde.desktop, Plasma, Material, Universal etc
     */
    Q_PROPERTY(QString style READ style CONSTANT)

    //TODO: make this adapt without file watchers?
    /**
     * How many lines of text the mouse wheel should scroll
     */
    Q_PROPERTY(int mouseWheelScrollLines READ mouseWheelScrollLines CONSTANT)

public:
    Settings(QObject *parent=0);
    ~Settings();

    void setTabletModeAvailable(bool mobile);
    bool isTabletModeAvailable() const;

    void setIsMobile(bool mobile);
    bool isMobile() const;

    void setTabletMode(bool tablet);
    bool tabletMode() const;

    QString style() const;
    void setStyle(const QString &style);

    int mouseWheelScrollLines() const;

Q_SIGNALS:
    void tabletModeAvailableChanged();
    void tabletModeChanged();
    void isMobileChanged();

private:
    QString m_style;
    int m_scrollLines = 0;
    bool m_tabletModeAvailable : 1;
    bool m_mobile : 1;
    bool m_tabletMode : 1;
};

#endif

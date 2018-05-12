/*
 * Copyright 2018 Marco Martin <mart@kde.org>
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation; either version 2 of
 * the License or (at your option) version 3 or any later version
 * accepted by the membership of KDE e.V. (or its successor approved
 * by the membership of KDE e.V.), which shall act as a proxy
 * defined in Section 14 of version 3 of the license.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#ifndef KIRIGAMI_TABLETMODEWATCHER
#define KIRIGAMI_TABLETMODEWATCHER

#include <QObject>

#ifndef KIRIGAMI_BUILD_TYPE_STATIC
#include <kirigami2_export.h>
#endif

namespace Kirigami {

class TabletModeWatcherPrivate;

/**
 * This class reports on the status of certain transformable
 * devices which can be both tablets and laptops at the same time,
 * with a detachable keyboard.
 * It reports whether the device supports a tablet mode and if
 * the device is currently in such mode or not, emitting a signal
 * when the user switches.
 */
#ifdef KIRIGAMI_BUILD_TYPE_STATIC
class TabletModeWatcher : public QObject
#else
class KIRIGAMI2_EXPORT TabletModeWatcher : public QObject
#endif
{
    Q_OBJECT
    Q_PROPERTY(bool tabletModeAvailable READ isTabletModeAvailable NOTIFY tabletModeAvailableChanged)
    Q_PROPERTY(bool tabletMode READ isTabletMode NOTIFY tabletModeChanged)

public:
    ~TabletModeWatcher();
    static TabletModeWatcher *self();

    /**
     * @returns true if the device supports a tablet mode and has a switch
     * to report when the device has been transformed.
     * For debug purposes, if either the environment variable QT_QUICK_CONTROLS_MOBILE
     * or KDE_KIRIGAMI_TABLET_MODE are set to true, isTabletModeAvailable will be true
     */
    bool isTabletModeAvailable() const;

    /**
     * @returns true if the machine is now in tablet mode, such as the
     * laptop keyboard flipped away or detached.
     * Note that this doesn't mean exactly a tablet form factor, but
     * that the preferred input mode for the device is the touch screen
     * and that pointer and keyboard are either secondary or not available.
     *
     * For debug purposes, if either the environment variable QT_QUICK_CONTROLS_MOBILE
     * or KDE_KIRIGAMI_TABLET_MODE are set to true, isTabletMode will be true
     */
    bool isTabletMode() const;

Q_SIGNALS:
    void tabletModeAvailableChanged(bool tabletModeAvailable);
    void tabletModeChanged(bool tabletMode);

private:
    TabletModeWatcher(QObject *parent = 0);
    TabletModeWatcherPrivate *d;
    friend class TabletModeWatcherSingleton;
};
}

#endif // KIRIGAMI_TABLETMODEWATCHER

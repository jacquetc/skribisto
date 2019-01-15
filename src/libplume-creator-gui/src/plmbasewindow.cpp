/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasewindow.cpp
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
#include "plmbasewindow.h"

#include <QSettings>
#include <QDebug>

PLMBaseWindow::PLMBaseWindow(QWidget *parent) : QMainWindow(parent), m_detached(false)
{
    this->applySettingsState();
}

PLMBaseWindow::~PLMBaseWindow()
{
    this->saveSettingsState();
}

void PLMBaseWindow::applySettingsState()
{
    QSettings settings;

    settings.beginGroup("MainBar");

    // state

    int stateVersion = 1;
    this->restoreState(settings.value(this->property(
                                          "name").toString() + "-state",
                                      "0").toByteArray(),
                       stateVersion);


    settings.endGroup();
}

void PLMBaseWindow::saveSettingsState()
{
    QSettings settings;


    settings.beginGroup("MainBar");

    // state
    int stateVersion = 1;
    settings.setValue(this->property("name").toString() + "-state",
                      this->saveState(stateVersion));


    settings.endGroup();
}

void PLMBaseWindow::applySettingsGeometry()
{
    QSettings settings;

    settings.beginGroup("MainBar");

    // geometry
    this->restoreGeometry(settings.value(this->property(
                                             "name").toString() + "-geometry",
                                         "0").toByteArray());


    settings.endGroup();
}

void PLMBaseWindow::saveSettingsGeometry()
{
    QSettings settings;


    settings.beginGroup("MainBar");

    // geometry
    settings.setValue(this->property("name").toString() + "-geometry",
                      this->saveGeometry());


    settings.endGroup();
}

void PLMBaseWindow::closeEvent(QCloseEvent *event)
{
    if (this->detached()) {
        saveSettingsGeometry();
        emit attachmentCalled(this->property("name").toString());
    }

    // to block the kill
    event->setAccepted(false);
}

void PLMBaseWindow::moveEvent(QMoveEvent *event)
{
    if (this->detached()) {
        this->saveSettingsGeometry();
        event->setAccepted(true);
    }
}

bool PLMBaseWindow::detached() const
{
    return m_detached;
}

void PLMBaseWindow::setDetached(bool detached)
{
    m_detached = detached;
}

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
#include "plmpluginloader.h"

#include <QSettings>
#include <QDebug>
#include <QTimer>
#include <QFlags>

PLMBaseWindow::PLMBaseWindow(QWidget *parent, const QString& name) : QMainWindow(parent),
    m_name(name), m_detached(false),
    m_forceTrueClosing(false), m_rightDockDefaultCount(0), m_bottomDockDefaultCount(0),
    m_leftDockDefaultCount(0)
{
    Q_ASSERT(!name.isNull() && !name.isEmpty());
    this->loadPlugins();
    this->applyStyleSheet();

    this->setAttribute(Qt::WA_DeleteOnClose);
    QTimer::singleShot(0, this, SLOT(applyDockSettings()));
}

PLMBaseWindow::~PLMBaseWindow()
{}

void PLMBaseWindow::loadPlugins()
{
    //    PLMPluginLoader *loader = PLMPluginLoader::instance();

    //    loader->addPluginType<PLMDockWidgetInterface>();

    QList<PLMDockWidgetInterface *> pluginList =
        PLMPluginLoader::instance()->pluginsByType<PLMDockWidgetInterface>();

    for (PLMDockWidgetInterface *plugin : pluginList) {
        if (plugin->pluginEnabled()) {
            QString parentName = plugin->getParentWindowName();

            if (parentName == this->name()) {
                qDebug();
                m_DockPluginList.append(plugin);
            }
        }
    }
}

void PLMBaseWindow::applySettingsState()
{
    QSettings settings;

    settings.beginGroup("Windows");

    // state

    int stateVersion = 1;
    this->restoreState(settings.value(this->name() + "-state",
                                      "0").toByteArray(),
                       stateVersion);


    settings.endGroup();
}

void PLMBaseWindow::saveSettingsState()
{
    QSettings settings;


    settings.beginGroup("Windows");


    // state
    int stateVersion = 1;
    settings.setValue(this->name() + "-state",
                      this->saveState(stateVersion));


    settings.endGroup();
}

void PLMBaseWindow::applySettingsGeometry()
{
    QSettings settings;

    settings.beginGroup("Windows");

    // geometry
    this->restoreGeometry(settings.value(this->name() + "-geometry",
                                         "").toByteArray());


    settings.endGroup();
}

void PLMBaseWindow::saveSettingsGeometry()
{
    QSettings settings;


    settings.beginGroup("Windows");

    // geometry
    settings.setValue(this->name() + "-geometry",
                      this->saveGeometry());


    settings.endGroup();
}

void PLMBaseWindow::applyDockSettings()
{
    QSettings settings;

    settings.beginGroup("Windows");

    // docks :
    // left:
    int leftDockNumber = settings.value(this->name() + "-leftDockNumber",
                                        m_leftDockDefaultCount).toInt();

    for (int i = 0; i < leftDockNumber; ++i) {
        this->addLeftDock();
    }

    // bottom:
    int bottomDockNumber = settings.value(this->name() + "-bottomDockNumber",
                                          m_bottomDockDefaultCount).toInt();

    for (int i = 0; i < bottomDockNumber; ++i) {
        this->addBottomDock();
    }

    // right:
    int rightDockNumber = settings.value(this->name() + "-rightDockNumber",
                                         m_rightDockDefaultCount).toInt();

    for (int i = 0; i < rightDockNumber; ++i) {
        this->addRightDock();
    }

    settings.endGroup();

    this->applySettingsState();
}

void PLMBaseWindow::saveDockSettings()
{
    QSettings settings;

    settings.beginGroup("Windows");

    // docks :
    // left:
    settings.setValue(this->name() + "-leftDockNumber",   this->leftDocks().count());

    // bottom:
    settings.setValue(this->name() + "-bottomDockNumber", this->bottomDocks().count());


    // right:
    settings.setValue(this->name() + "-rightDockNumber", this->rightDocks().count());

    settings.endGroup();

    this->saveSettingsState();
}

QString PLMBaseWindow::name() const
{
    return m_name;
}

QList<PLMBaseDock *>PLMBaseWindow::rightDocks() const
{
    return m_rightDocks;
}

QList<PLMBaseDock *>PLMBaseWindow::bottomDocks() const
{
    return m_bottomDocks;
}

QList<PLMBaseDock *>PLMBaseWindow::leftDocks() const
{
    return m_leftDocks;
}

int PLMBaseWindow::leftDockDefaultCount() const
{
    return m_leftDockDefaultCount;
}

void PLMBaseWindow::setLeftDockDefaultCount(int leftDockDefaultCount)
{
    m_leftDockDefaultCount = leftDockDefaultCount;
}

PLMBaseDock * PLMBaseWindow::addRightDock()
{
    PLMBaseDock *dock = new PLMBaseDock(this, Qt::RightEdge, "SheetTree");

    dock->setAllowedAreas(Qt::RightDockWidgetArea);
    connect(dock, &PLMBaseDock::addDockCalled, this, &PLMBaseWindow::addRightDock);


    if (m_rightDocks.isEmpty()) {
        this->addDockWidget(Qt::RightDockWidgetArea, dock);
    } else {
        this->splitDockWidget(m_rightDocks.last(), dock, Qt::Vertical);
    }
    dock->setObjectName(this->name() + "-dock-" +
                        QString::number(dock->getEdge()) + "-" +
                        QString::number(m_leftDocks.indexOf(dock)));

    m_rightDocks.append(dock);

    return dock;
}

PLMBaseDock * PLMBaseWindow::addBottomDock()
{
    PLMBaseDock *dock = new PLMBaseDock(this, Qt::BottomEdge, "SheetTree");

    dock->setAllowedAreas(Qt::BottomDockWidgetArea);
    connect(dock, &PLMBaseDock::addDockCalled, this, &PLMBaseWindow::addBottomDock);

    if (m_bottomDocks.isEmpty()) {
        this->addDockWidget(Qt::BottomDockWidgetArea, dock);
    } else {
        this->splitDockWidget(m_bottomDocks.last(), dock, Qt::Horizontal);
    }
    dock->setObjectName(this->name() + "-dock-" +
                        QString::number(dock->getEdge()) + "-" +
                        QString::number(m_leftDocks.indexOf(dock)));

    m_bottomDocks.append(dock);
    return dock;
}

PLMBaseDock * PLMBaseWindow::addLeftDock()
{
    PLMBaseDock *dock = new PLMBaseDock(this, Qt::LeftEdge, "SheetTree");

    dock->setAllowedAreas(Qt::LeftDockWidgetArea);
    connect(dock, &PLMBaseDock::addDockCalled, this, &PLMBaseWindow::addLeftDock);

    if (m_leftDocks.isEmpty()) {
        this->addDockWidget(Qt::LeftDockWidgetArea, dock);
    } else {
        this->splitDockWidget(m_leftDocks.last(), dock, Qt::Vertical);

        // TODO: find a way to resize docks, beware of restorestate()
        //        int hefff        = m_leftDocks.last()->height();
        //        int he           = dock->height();
        //        int windowHeight = this->height();
        //        this->resizeDocks({ dock },
        //        {
        //            windowHeight / 2
        //        },
        //                          Qt::Vertical);
    }
    m_leftDocks.append(dock);

    dock->setObjectName(this->name() + "-dock-" +
                        QString::number(dock->getEdge()) + "-" +
                        QString::number(m_leftDocks.indexOf(dock)));


    for (PLMDockWidgetInterface *plugin : m_DockPluginList) {
        QFlags<Qt::Edge> edgeFlags(plugin->getEdges());

        if (edgeFlags.testFlag(Qt::LeftEdge)) {
            dock->addWidgetContainer(plugin->name(), plugin->clone());
        }
    }


    return dock;
}

int PLMBaseWindow::bottomDockDefaultCount() const
{
    return m_bottomDockDefaultCount;
}

void PLMBaseWindow::setBottomDockDefaultCount(int bottomDockDefaultCount)
{
    m_bottomDockDefaultCount = bottomDockDefaultCount;
}

int PLMBaseWindow::rightDockDefaultCount() const
{
    return m_rightDockDefaultCount;
}

void PLMBaseWindow::setRightDockDefaultCount(int rightDockDefaultCount)
{
    m_rightDockDefaultCount = rightDockDefaultCount;
}

void PLMBaseWindow::closeEvent(QCloseEvent *event)
{
    this->saveDockSettings();

    if (this->detached()) {
        saveSettingsGeometry();

        if (m_forceTrueClosing) {
            event->setAccepted(true);
        }
        else {
            emit attachmentCalled(this->name());

            // to block the kill
            event->setAccepted(false);
        }
    }
}

void PLMBaseWindow::moveEvent(QMoveEvent *event)
{
    if (this->detached()) {
        QTimer::singleShot(500, this, SLOT(saveSettingsGeometry()));
    }
}

void PLMBaseWindow::setForceTrueClosing(bool forceTrueClosing)
{
    m_forceTrueClosing = forceTrueClosing;
}

bool PLMBaseWindow::detached() const
{
    return m_detached;
}

void PLMBaseWindow::setDetached(bool detached)
{
    if ((m_detached == true) && (detached == false)) this->saveSettingsGeometry();

    if ((m_detached == false) && (detached == true)) this->applySettingsGeometry();


    m_detached = detached;
}

// ------------------------------------------------------------

void PLMBaseWindow::applyStyleSheet()
{
    QFile file(":/stylesheets/light.css");

    Q_ASSERT(file.exists());

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) return;

    QString content = file.readAll();
    file.close();
    this->setStyleSheet(content);
}

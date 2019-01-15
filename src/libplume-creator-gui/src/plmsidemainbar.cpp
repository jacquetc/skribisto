/***************************************************************************
*   Copyright (C) 2017 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsidemainbar.cpp
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
#include "plmsidemainbar.h"
#include "plmpluginloader.h"
#include "plmguiinterface.h"
#include "ui_plmsidemainbar.h"

#include <QMenu>
#include <QVBoxLayout>
#include <QSettings>
#include <QTimer>

PLMSideMainBar::PLMSideMainBar(QWidget *parent) : QWidget(parent),
    ui(new Ui::PLMSideMainBar)
{
    ui->setupUi(this);
    actionGroup = new QActionGroup(this);
    this->loadPlugins();

    QTimer::singleShot(0, this, SLOT(init()));
}

void PLMSideMainBar::init()
{
    this->readSettings();

    // this->readSettings();
    // load plugins
    // TEMP
}

void PLMSideMainBar::loadPlugins()
{
    // plugins are already loaded in plmpluginloader
    QList<PLMSideMainBarIconInterface *> pluginList =
        PLMPluginLoader::instance()->pluginsByType<PLMSideMainBarIconInterface>();


    // ordering
    QMap<int, QAction *> actionMap;
    foreach(PLMSideMainBarIconInterface * plugin, pluginList) {
        QList<PLMSideBarAction> actionList = plugin->sideMainBarActions(this);

        foreach(const PLMSideBarAction &sideBarAction, actionList) {
            actionMap.insert(sideBarAction.action()->property("order").toInt(),
                             sideBarAction.action());
        }
    }
    foreach(QAction * action, actionMap) {
        QToolButton *button = new QToolButton(this);

        button->setDefaultAction(action);
        button->setIconSize(QSize(48, 48));
        button->setAutoRaise(true);
        button->setProperty("linkedWindow", action->property("linkedWindow").toString());
        ui->verticalLayout->addWidget(button);
        actionGroup->addAction(action);
        hash_windowNameAndButton.insert(action->property("linkedWindow").toString(),
                                        button);
        connect(action,
                &QAction::triggered,
                this,
                &PLMSideMainBar::raiseWindow,
                Qt::UniqueConnection);

        button->setContextMenuPolicy(Qt::CustomContextMenu);
        connect(button, SIGNAL(customContextMenuRequested(const QPoint&)),
                this, SLOT(showContextMenu(const QPoint&)));
        connect(button,
                &QToolButton::triggered,
                this,
                &PLMSideMainBar::buttonChecked,
                Qt::UniqueConnection);
    }
}

void PLMSideMainBar::setButtonChecked(const QString& windowName)
{
    QToolButton *button = hash_windowNameAndButton.value(windowName);

    if (button) {
        button->defaultAction()->setChecked(true);
    }
}

void PLMSideMainBar::buttonChecked() {
    QSettings settings;

    settings.beginGroup("MainBar");

    settings.setValue("place",        this->objectName());
    settings.setValue("selectedName", this->sender()->property("linkedWindow"));

    settings.endGroup();
}

void PLMSideMainBar::showContextMenu(const QPoint& pos)
{
    QToolButton *button = dynamic_cast<QToolButton *>(this->sender());

    if (button->defaultAction()->property("detachable").toBool() == false) {
        return;
    }
    m_currentButton = button;
    QMenu contextMenu(tr("Context menu"), this);

    QAction action1(tr("Attach"), this);
    connect(&action1, SIGNAL(triggered()), this, SLOT(attachWindow()));
    contextMenu.addAction(&action1);
    QAction action2(tr("Detach"), this);
    connect(&action2, SIGNAL(triggered()), this, SLOT(detachWindow()));
    contextMenu.addAction(&action2);

    contextMenu.exec(button->mapToGlobal(pos));
}

void PLMSideMainBar::detachWindow() {
    m_currentButton->setEnabled(false);
    emit windowDetachmentCalled(m_currentButton->defaultAction()->property(
                                    "linkedWindow").toString());
}

void PLMSideMainBar::attachWindow() {
    m_currentButton->setEnabled(true);
    emit windowAttachmentCalled(m_currentButton->defaultAction()->property(
                                    "linkedWindow").toString());
}

void PLMSideMainBar::attachWindowByName(const QString& windowName) {
    QToolButton *button = hash_windowNameAndButton.value(windowName);

    button->setEnabled(true);
    button->defaultAction()->setChecked(true);
    emit windowAttachmentCalled(button->defaultAction()->property(
                                    "linkedWindow").toString());
}

void PLMSideMainBar::raiseWindow(bool checked)
{
    QAction *action = dynamic_cast<QAction *>(this->sender());

    if (!checked) {
        action->setChecked(true);
        return;
    }

    QString windowName = action->property("linkedWindow").toString();

    emit windowRaiseCalled(windowName);
}

void PLMSideMainBar::readSettings()
{
    QSettings settings;

    settings.beginGroup("MainBar");

    // bar visible ?
    if (settings.value("place", this->objectName()).toByteArray() != this->objectName()) {
        this->setVisible(false);
    }

    // checked ?
    if (settings.value("place", this->objectName()).toByteArray() == this->objectName()) {
        QString selectedName = settings.value("selectedName", "writeWindow").toString();
        this->setButtonChecked(selectedName);
    }

    // raised ?
    foreach(const QString &windowName, hash_windowNameAndButton.keys()) {
        if (settings.value(windowName + "-raised", false).toBool() == true) {
            emit windowDetachmentCalled(windowName);
        }
    }


    settings.endGroup();
}

// -------------------------------------------------------------------------
// -------------------------------------------------------------------------


QString PLMSideBarAction::windowName() const
{
    return m_windowName;
}

void PLMSideBarAction::setWindowName(const QString& windowName)
{
    m_windowName = windowName;
}

QAction * PLMSideBarAction::action() const
{
    return m_action;
}

void PLMSideBarAction::setAction(QAction *action)
{
    m_action = action;
}

// -------------------------------------------------------------------------
// -------------------------------------------------------------------------

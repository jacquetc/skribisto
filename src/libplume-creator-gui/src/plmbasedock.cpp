/***************************************************************************
*   Copyright (C) 2018 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbaseleftdock.cpp
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
#include "plmbasedock.h"
#include "ui_plmbasedockheader.h"
#include "ui_plmbasedockbody.h"
#include "plmbasedockwidget.h"
#include <QTimer>
#include <QWidget>

PLMBaseDock::PLMBaseDock(QWidget *parent, Qt::Edge edge,
                         const QString& defaultWidget) : QDockWidget(parent),
    headerUi(new Ui::PLMBaseDockHeader), bodyUi(new Ui::PLMBaseDockBody), m_edge(edge),
    m_containerIsBeingAdded(false), m_defaultWidgetName(defaultWidget)
{
    this->setFeatures(NoDockWidgetFeatures);


    QWidget *header = new QWidget();
    this->setTitleBarWidget(header);
    headerUi->setupUi(header);


    QWidget *body = new QWidget();
    this->setWidget(body);
    bodyUi->setupUi(body);
    body->show();

    connect(headerUi->closeButton, &QToolButton::clicked, this, &PLMBaseDock::close);
    connect(headerUi->addDockButton,
            &QToolButton::clicked,
            this,
            &PLMBaseDock::addDockCalled);
    connect(headerUi->dockHeaderComboBox,
            &QComboBox::currentTextChanged,
            this,
            &PLMBaseDock::changeCurrentWidget, Qt::UniqueConnection);

    QTimer::singleShot(0, this, SLOT(applyCurrentWidgetSetting()));
}

void PLMBaseDock::changeCurrentWidget(const QString& text)
{
    Q_UNUSED(text)
    QString name = headerUi->dockHeaderComboBox->currentData().toString();

    this->setCurrentWidgetByName(name);
}

void PLMBaseDock::applyCurrentWidgetSetting()
{
    QSettings settings;

    settings.beginGroup("Docks");
    QString name = settings.value(this->getDockSettingName() + "-current",
                                  this->getDefaultWidgetName()).toString();
    settings.endGroup();

    if (!m_widgetContainerHash.contains(name)) return;

    this->setCurrentWidgetByName(name);
}

QString PLMBaseDock::getDefaultWidgetName() const
{
    return m_defaultWidgetName;
}

void PLMBaseDock::setDefaultWidgetName(const QString& defaultWidgetName)
{
    m_defaultWidgetName = defaultWidgetName;
}

PLMBaseDock::~PLMBaseDock()
{
    qDeleteAll(m_widgetContainerHash.values());

    delete headerUi;
}

QString PLMBaseDock::getCurrentWidgetName() const
{
    return headerUi->dockHeaderComboBox->currentData().toString();
}

void PLMBaseDock::setCurrentWidgetByName(const QString& widgetName)
{
    if (!m_widgetContainerHash.contains(widgetName)) {
        return;
    }

    disconnect(headerUi->dockHeaderComboBox,
               &QComboBox::currentTextChanged,
               this,
               &PLMBaseDock::changeCurrentWidget);

    // body :
    bool widgetPresent      = false;
    PLMBaseDockWidget *body = m_widgetContainerHash.value(widgetName)->getDockBody();

    for (int i = 0; i < bodyUi->stackedWidget->count(); ++i) {
        QWidget *widget = bodyUi->stackedWidget->widget(i);

        if (static_cast<PLMBaseDockWidget *>(widget) == body) {
            widgetPresent = true;
        }
    }

    if (!widgetPresent) {
        bodyUi->stackedWidget->addWidget(body);
    }
    bodyUi->stackedWidget->setCurrentWidget(body);

    // header :
    bool headerPresent = false;
    QWidget *header    = m_widgetContainerHash.value(widgetName)->getDockHeader();

    for (QWidget *stockedHeader : m_widgetHeadersHash.values()) {
        if (stockedHeader == header) {
            headerPresent = true;
        }
    }

    if (!headerPresent) {
        m_widgetHeadersHash.insert(widgetName, header);
        headerUi->headerHolder->addWidget(header);
    }

    for (QWidget *stockedHeader : m_widgetHeadersHash.values()) {
        stockedHeader->setVisible(false);
    }
    m_widgetHeadersHash.value(widgetName)->setVisible(true);

    // hide all

    headerUi->dockHeaderComboBox->setCurrentText(m_widgetContainerHash.value(
                                                     widgetName)->displayedName());

    emit currentWidgetChanged(widgetName);

    // setting

    if (!m_containerIsBeingAdded) {
        QSettings settings;
        settings.beginGroup("Docks");
        settings.setValue(this->getDockSettingName() + "-current", widgetName);
        settings.endGroup();
    }

    // reconnect
    connect(headerUi->dockHeaderComboBox,
            &QComboBox::currentTextChanged,
            this,
            &PLMBaseDock::changeCurrentWidget, Qt::UniqueConnection);
}

//    headerUi->dockHeaderComboBox->addItem(container->displayedName,
// container->name);
//    headerUi->headerHolder->addWidget(dockHeader);
//    this->setWidget(dockBody);
//    dockBody->show();


Qt::Edge PLMBaseDock::getEdge() const
{
    return m_edge;
}

void PLMBaseDock::addWidgetContainer(const QString         & name,
                                     PLMDockWidgetInterface *dockPlugin)
{
    PLMWidgetContainer *container = new PLMWidgetContainer(this, dockPlugin);

    m_widgetContainerHash.insert(name, container);
    m_containerIsBeingAdded = true;
    headerUi->dockHeaderComboBox->addItem(dockPlugin->displayedName(), name);
    m_containerIsBeingAdded = false;
}

QString PLMBaseDock::getDockSettingName() const
{
    return this->objectName();
}

QString PLMWidgetContainer::name() const
{
    return m_plugin->name();
}

QString PLMWidgetContainer::displayedName() const
{
    return m_plugin->displayedName();
}

QPointer<QWidget>PLMWidgetContainer::getDockHeader()
{
    return m_plugin->dockHeaderWidget(this->getParent());
}

QPointer<PLMBaseDockWidget>PLMWidgetContainer::getDockBody()
{
    return m_plugin->dockBodyWidget(this->getParent());
}

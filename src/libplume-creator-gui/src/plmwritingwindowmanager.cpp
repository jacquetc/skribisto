/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmwritingwindowmanager.cpp
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
#include "plmwritingwindowmanager.h"

#include <QVariant>
#include <QDebug>
#include <QSettings>

PLMWritingWindowManager::PLMWritingWindowManager(QObject       *parent,
                                                 QBoxLayout    *baseLayout,
                                                 const QString& objectName) : QObject(
        parent), m_baseBoxLayout(baseLayout)
{
    qRegisterMetaType<QList<WindowContainer> >("QList<WindowContainer>");

    this->setObjectName(objectName);


    WindowContainer container;
    container.setId(0);
    container.setParentId(-1);
    container.addLayout(m_baseBoxLayout);
    container.setOrientation(Qt::Vertical);
    m_windowContainerList << container;

    //    addWritingWindow(Qt::Vertical,   0);
    //    addWritingWindow(Qt::Vertical,   1);
    //    addWritingWindow(Qt::Horizontal, 2);

    this->applySettings();
}

PLMWritingWindow * PLMWritingWindowManager::addWritingWindow(Qt::Orientation orientation,
                                                             int parentZone, int forcedId)
{
    QBoxLayout::Direction boxOrientation =
        QBoxLayout::Direction::TopToBottom;

    if (orientation == Qt::Vertical) boxOrientation =
            QBoxLayout::Direction::TopToBottom;

    if (orientation ==
        Qt::Horizontal) boxOrientation =
            QBoxLayout::Direction::LeftToRight;


    WindowContainer container;

    if (forcedId == -2) container.setId(m_windowContainerList.count());
    else {
        container.setId(forcedId);
    }
    container.setOrientation(orientation);
    container.setParentId(parentZone);

    QPointer<PLMWritingWindow> parentWindow = nullptr;
    QPointer<QBoxLayout> parentLayout       = nullptr;
    WindowContainer parentContainer;
    int i = 0;


    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() == parentZone) {
            parentLayout    = item.layoutList().last();
            parentWindow    = item.window();
            parentContainer = WindowContainer(item);
            break;
        }
        ++i;
    }

    container.addLayout(parentLayout);

    QBoxLayout *layout = new QBoxLayout(boxOrientation, nullptr);
    container.addLayout(layout);
    parentContainer.addLayout(layout);


    m_windowContainerList.replace(i, parentContainer);


    PLMWritingWindow *window = new PLMWritingWindow(container.id());
    window->setObjectName(container.name());
    container.setWindow(window);

    if (parentZone != 0) {
        int index = parentLayout->indexOf(parentWindow);
        parentLayout->removeWidget(parentWindow);
        layout->addWidget(parentWindow);
        parentLayout->insertLayout(index, layout);
    }
    else {
        parentLayout->addLayout(layout);
    }
    layout->addWidget(window);

    m_windowContainerList << container;

    connect(window, &PLMWritingWindow::writingWindowClosed, this,
            &PLMWritingWindowManager::closeWritingWindow);

    connect(window,
            &PLMWritingWindow::splitCalled, [this](Qt::Orientation orientation,
                                                   int parentZone) {
        this->addWritingWindow(orientation, parentZone);
    }
            );

    qDebug() << "added :" << container.name();
    this->writeSettings();

    return window;

    //    if (m_windowContainerList.isEmpty() || (parentZone == 0)) {
    //        WindowContainer container;
    //        container.setId(1);
    //        container.setOrientation(Qt::Vertical);
    //        container.setParentId(0);
    //        container.setLayout(m_baseBoxLayout);

    //        //        QBoxLayout *layout = new
    //        // QBoxLayout(QBoxLayout::Direction::TopToBottom,
    //        //                                            nullptr);
    //        PLMWritingWindow *window = new PLMWritingWindow(container.id());
    //        window->setObjectName(container.name());
    //        container.setWindow(window);
    //        m_windowContainerList << container;
    //        m_baseBoxLayout->addWidget(window);

    //        connect(window, &PLMWritingWindow::writingWindowClosed, this,
    //                &PLMWritingWindowManager::closeWritingWindow);

    //        connect(window,
    //                &PLMWritingWindow::splitCalled,
    //                this,
    //                &PLMWritingWindowManager::addWritingWindow);

    //        qDebug() << "added :" << container.name();
    //        return window;
    //    }

    //    // create layout


    //    WindowContainer container;
    //    container.setId(m_windowContainerList.count() + 1);
    //    container.setOrientation(orientation);
    //    container.setParentId(parentZone);


    //    QBoxLayout *layout = new QBoxLayout(boxOrientation, nullptr);
    //    container.setLayout(layout);

    //    // determine parent layout and parent window
    //    QPointer<PLMWritingWindow> parentWindow;
    //    QBoxLayout *parentLayout = nullptr;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() == parentZone) {
    //            parentLayout = item.layout();
    //            parentWindow = item.window();
    //        }
    //    }


    //    if (parentWindow.isNull() || (parentLayout == nullptr)) {
    //        layout->deleteLater();
    //        return nullptr;
    //    }

    //    // set layout orientation :
    //    // Q_ASSERT(!container.parentLayout().isNull());
    //    parentLayout->addLayout(container.layout());


    //    PLMWritingWindow *window = new PLMWritingWindow(container.id());
    //    window->setObjectName(container.name());
    //    container.setWindow(window);
    //    parentLayout->removeWidget(parentWindow);
    //    layout->addWidget(parentWindow);
    //    layout->addWidget(window);

    //    m_windowContainerList << container;

    //    connect(window, &PLMWritingWindow::writingWindowClosed, this,
    //            &PLMWritingWindowManager::closeWritingWindow);

    //    connect(window,
    //            &PLMWritingWindow::splitCalled,
    //            this,
    //            &PLMWritingWindowManager::addWritingWindow);

    //    qDebug() << "added :" << container.name();

    //    return window;
}

// ------------------------------------------------------------

void PLMWritingWindowManager::writeSettings()
{
    QSettings settings;

    // windows :
    QStringList windowNames;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() != 0) windowNames << item.name();
    }

    settings.setValue("WritingWindowManager/" + this->objectName() + "-windows",
                      windowNames);

    // windows sizes :
    QStringList sizeList;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() !=
            0) sizeList << QString::number(item.windowSize().width()) + "x" +
                QString::number(
                item.windowSize().height());
    }


    settings.setValue("WritingWindowManager/" + this->objectName() + "-sizes",
                      sizeList);
}

// ------------------------------------------------------------

void PLMWritingWindowManager::applySettings()
{
    QSettings settings;

    // windows :
    QStringList windowNames;

    windowNames = settings.value(
        "WritingWindowManager/" + this->objectName() + "-windows",
        "").toStringList();

    windowNames.removeAll("");

    // windows sizes :
    QStringList sizeList;
    sizeList = settings.value(
        "WritingWindowManager/" + this->objectName() + "-sizes",
        "").toStringList();
    int index = 0;

    while (index < windowNames.count() && index <  sizeList.count())
    {
        // windows :

        WindowContainer falseContainer;
        falseContainer.setName(windowNames.at(index));

        if (falseContainer.id() != 0) this->addWritingWindow(falseContainer.orientation(),
                                                             falseContainer.parentId(),
                                                             falseContainer.id());

        // windows sizes :

        QString sizeString = sizeList.at(index);
        QStringList parts  = sizeString.split("x");
        QSize size(parts.at(0).toInt(), parts.at(1).toInt());

        WindowContainer container(m_windowContainerList.at(index + 1));
        container.setWindowSize(size);
        ++index;
    }
}

// ------------------------------------------------------------

void PLMWritingWindowManager::closeWritingWindow(int id)
{
    WindowContainer container;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() == id) {
            container = item;
        }
    }

    if (!container) return;

    //    QPointer<QBoxLayout> layout       = container.layout();
    //    QPointer<QBoxLayout> parentLayout = container.parentLayout();
    //    QPointer<PLMWritingWindow> window = container.window();
    //    int parentId                      = container.parentId();
    //    Qt::Orientation orientation       = container.orientation();

    m_windowContainerList.removeAll(container);

    WindowContainer itemClone;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.parentId() == id) {
            itemClone = WindowContainer(item);
            itemClone.setOrientation(container.orientation());
            itemClone.setParentId(container.parentId());

            //            itemClone.setLayoutList(container.parentLayout());
            m_windowContainerList.replace(m_windowContainerList.indexOf(item),
                                          itemClone);
        }
    }


    //    // if (!container.parentLayout().isNull()) {
    //    if (itemClone) {
    //        // container.parentLayout()->addLayout(itemClone.layout());
    //    }
    container.window()->close();

    this->writeSettings();

    //    container.layout()->removeWidget(container.window());

    //    //    container.parentLayout()->removeItem(container.layout());
    //    container.window()->deleteLater();

    //    //    container.layout()->deleteLater();

    //    // }

    //    //    else { // if is top window
    //    //        container.layout()->removeWidget(container.window());
    //    //    }

    //    qDebug() << "removed :" << container.name();
}

// 1V0-2V1-3V1-4H2

// ---------------------------------------------------------
// ---------------------------------------------------------
// ---------------------------------------------------------


WindowContainer::WindowContainer() {}

WindowContainer::WindowContainer(const WindowContainer& otherContainer) {
    this->setWindow(otherContainer.window());
    this->setId(otherContainer.id());
    this->setOrientation(otherContainer.orientation());
    this->setParentId(otherContainer.parentId());
    this->setLayoutList(otherContainer.layoutList());
}

WindowContainer::~WindowContainer()
{}

bool WindowContainer::operator!() const {
    return this->m_window.isNull() || this->layoutList().isEmpty();
}

WindowContainer& WindowContainer::operator=(const WindowContainer& otherContainer) {
    this->setWindow(otherContainer.window());
    this->setId(otherContainer.id());
    this->setOrientation(otherContainer.orientation());
    this->setParentId(otherContainer.parentId());
    this->setLayoutList(otherContainer.layoutList());
    return *this;
}

bool WindowContainer::operator==(const WindowContainer& otherContainer) const
{
    if (this->window() != otherContainer.window()) return false;

    if (this->id() != otherContainer.id()) return false;

    if (this->parentId() != otherContainer.parentId()) return false;

    if (this->layoutList() != otherContainer.layoutList()) return false;

    if (this->orientation() != otherContainer.orientation()) return false;

    return true;
}

WindowContainer::operator bool() const {
    return !this->m_window.isNull() && !this->layoutList().isEmpty();
}

QPointer<PLMWritingWindow>WindowContainer::window() const
{
    return m_window;
}

void WindowContainer::setWindow(const QPointer<PLMWritingWindow>& window)
{
    m_window = window;
}

int WindowContainer::parentId() const
{
    return m_parentId;
}

void WindowContainer::setParentId(int value)
{
    m_parentId = value;
}

int WindowContainer::id() const
{
    return m_id;
}

void WindowContainer::setId(int value)
{
    m_id = value;
}

QString WindowContainer::name() const
{
    QString orientation;

    if (m_orientation == Qt::Vertical) orientation = "V";

    if (m_orientation == Qt::Horizontal) orientation = "H";

    return QString::number(this->id()) + orientation + QString::number(this->parentId());
}

bool WindowContainer::setName(const QString& name)
{
    QStringList stringList;

    if (name.contains("V")) {
        this->setOrientation(Qt::Vertical);
        stringList = name.split("V");
    }

    if (name.contains("H")) {
        this->setOrientation(Qt::Horizontal);
        stringList = name.split("H");
    }

    if (stringList.count() != 2) return false;

    bool ok  = true;
    int  pre = stringList.first().toInt(&ok);

    if (!ok) return false;

    int post = stringList.last().toInt(&ok);

    if (!ok) return false;

    this->setId(pre);
    this->setParentId(post);


    return true;
}

Qt::Orientation WindowContainer::orientation() const
{
    return m_orientation;
}

void WindowContainer::setOrientation(const Qt::Orientation& orientation)
{
    m_orientation = orientation;
}

void WindowContainer::setOrientation(const QString& orientation)
{
    if (orientation == "V") m_orientation = Qt::Vertical;

    if (orientation == "H") m_orientation = Qt::Horizontal;
}

QList<QPointer<QBoxLayout> >WindowContainer::layoutList() const
{
    return m_layoutList;
}

void WindowContainer::addLayout(const QPointer<QBoxLayout>& layout)
{
    m_layoutList.append(layout);
}

void WindowContainer::setLayoutList(const QList<QPointer<QBoxLayout> >& layoutList)
{
    m_layoutList = layoutList;
}

QSize WindowContainer::windowSize() const
{
    if (!m_window.isNull()) return m_window->size();

    return QSize();
}

void WindowContainer::setWindowSize(const QSize& size)
{
    if (!m_window.isNull()) {
        // m_window->resize(size);
        //        QSizePolicy policy;
        //        policy.setVerticalPolicy();

        //        m_window->setFixedSize(size);
        //        m_window->setMinimumSize(QSize(100, 100));
        //        m_window->setMaximumSize(QSize(QWIDGETSIZE_MAX,
        // QWIDGETSIZE_MAX));
    }
}

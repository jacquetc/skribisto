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
#include "plmbasesubwindowmanager.h"

#include <QVariant>
#include <QDebug>
#include <QSettings>
#include <QTimer>

PLMBaseSubWindowManager::PLMBaseSubWindowManager(QWidget       *parent,
                                                 const QString& objectName) : QObject(
        parent), m_lastNumber(0)
{
    qRegisterMetaType<QList<WindowContainer> >("QList<WindowContainer>");

    this->setObjectName(objectName);


    WindowContainer container;
    container.setId(0);
    container.setParentId(-1);
    container.setSubWindowType("");
    QPointer<QSplitter> splitter = new QSplitter(Qt::Orientation::Vertical, parent);
    splitter->setChildrenCollapsible(false);
    container.addSplitter(splitter);
    parent->layout()->addWidget(splitter);
    container.setOrientation(Qt::Vertical);
    m_windowContainerList << container;

    QTimer::singleShot(0, this, SLOT(applySettings()));
}

PLMBaseSubWindowManager::~PLMBaseSubWindowManager()
{
    this->writeSettings();
}

bool PLMBaseSubWindowManager::haveOneWindow(const QString& subWindowType)
{
    if (m_windowContainerList.count() <= 1) return false;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.subWindowType() == subWindowType) {
            return true;
        }
    }
    return false;
}

///
/// \brief PLMBaseSubWindowManager::getWindowByType
/// \param subWindowType
/// \return
/// get the one with focus else get the first. Else create one subwindow
PLMBaseSubWindow * PLMBaseSubWindowManager::getWindowByType(const QString& subWindowType)
{
    QList<PLMBaseSubWindow *> list;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.subWindowType() == subWindowType) {
            if (item.isLastActive()) return item.window();
        }
    }

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.subWindowType() == subWindowType) list << item.window();
    }

    if (!list.isEmpty()) return list.last();

    return this->addSubWindow(subWindowType, Qt::Horizontal, 0);
}

void PLMBaseSubWindowManager::setLastActiveWindowByType(int id)
{
    QString subWindowType = subWindowTypeById(id);

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.subWindowType() == subWindowType) {
            WindowContainer clone(item);

            if (item.id() == id) {
                clone.setIsLastActive(true);
            }
            else {
                clone.setIsLastActive(false);
            }
            m_windowContainerList.replace(m_windowContainerList.indexOf(item), clone);
        }
    }
}

PLMBaseSubWindow * PLMBaseSubWindowManager::addSubWindow(const QString & subWindowType,
                                                         Qt::Orientation orientation,
                                                         int             parentZone,
                                                         int             forcedId)
{
    QBoxLayout::Direction boxOrientation =
        QBoxLayout::Direction::TopToBottom;

    if (orientation == Qt::Vertical) boxOrientation =
            QBoxLayout::Direction::TopToBottom;

    if (orientation ==
        Qt::Horizontal) boxOrientation =
            QBoxLayout::Direction::LeftToRight;


    WindowContainer container;

    if (forcedId == -2) {
        container.setId(m_lastNumber + 1);
    }
    else {
        container.setId(forcedId);
    }

    if (container.id() > m_lastNumber) m_lastNumber = container.id();

    container.setOrientation(orientation);
    container.setParentId(parentZone);

    QPointer<PLMBaseSubWindow> parentWindow = nullptr;
    QPointer<QSplitter> parentSplitter      = nullptr;
    WindowContainer     parentContainer;
    int i = 0;


    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() == parentZone) {
            parentSplitter  = item.splitterList().last();
            parentWindow    = item.window();
            parentContainer = WindowContainer(item);
            break;
        }
        ++i;
    }

    container.addSplitter(parentSplitter);

    QSplitter *splitter = new QSplitter(orientation, nullptr);
    splitter->setChildrenCollapsible(false);

    container.addSplitter(splitter);
    parentContainer.addSplitter(splitter);


    m_windowContainerList.replace(i, parentContainer);


    PLMBaseSubWindow *window = this->subWindowByName(subWindowType, container.id());
    window->setObjectName(container.name());
    container.setWindow(window);
    container.setSubWindowType(subWindowType);

    if (parentZone != 0) {
        int index = parentSplitter->indexOf(parentWindow);
        splitter->addWidget(parentWindow);
        parentSplitter->insertWidget(index, splitter);
    }
    else {
        parentSplitter->addWidget(splitter);
    }
    splitter->addWidget(window);

    m_windowContainerList << container;

    connect(window, &PLMBaseSubWindow::subWindowClosed, this,
            &PLMBaseSubWindowManager::closeSubWindow);

    connect(window,
            &PLMBaseSubWindow::splitCalled, [this](Qt::Orientation orientation,
                                                   int parentZone) {
        this->addSubWindow(subWindowTypeById(parentZone), orientation, parentZone);
    }
            );

    connect(window,
            &PLMBaseSubWindow::subWindowFocusActived,
            this,
            &PLMBaseSubWindowManager::setLastActiveWindowByType);

    qDebug() << "added :" << container.name();
    QStringList names;
    QStringList splitterCount;

    for (const WindowContainer& item : m_windowContainerList) {
        names << item.name();
        splitterCount << QString::number(item.splitterList().count());
    }
    qDebug() << "names" << names;
    qDebug() << "splitterCount" << splitterCount;

    // reorder


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
    //        PLMBaseSubWindow *window = new PLMWritingWindow(container.id());
    //        window->setObjectName(container.name());
    //        container.setWindow(window);
    //        m_windowContainerList << container;
    //        m_baseBoxLayout->addWidget(window);

    //        connect(window, &PLMWritingWindow::writingWindowClosed, this,
    //                &PLMBaseSubWindowManager::closeWritingWindow);

    //        connect(window,
    //                &PLMWritingWindow::splitCalled,
    //                this,
    //                &PLMBaseSubWindowManager::addWritingWindow);

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
    //    QPointer<PLMBaseSubWindow> parentWindow;
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


    //    PLMBaseSubWindow *window = new PLMWritingWindow(container.id());
    //    window->setObjectName(container.name());
    //    container.setWindow(window);
    //    parentLayout->removeWidget(parentWindow);
    //    layout->addWidget(parentWindow);
    //    layout->addWidget(window);

    //    m_windowContainerList << container;

    //    connect(window, &PLMWritingWindow::writingWindowClosed, this,
    //            &PLMBaseSubWindowManager::closeWritingWindow);

    //    connect(window,
    //            &PLMWritingWindow::splitCalled,
    //            this,
    //            &PLMBaseSubWindowManager::addWritingWindow);

    //    qDebug() << "added :" << container.name();

    //    return window;
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::writeSettings()
{
    QSettings settings;

    // windows :
    QStringList windowNames;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() != 0) windowNames << item.name();
    }

    settings.setValue("WindowManager/" + this->objectName() + "-windows",
                      windowNames);

    // windows sizes :
    QStringList sizeList;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() !=
            0) {
            QList<int> sizes = item.splitterSizes();
            QString    sizeString;

            for (const int& size : sizes) {
                sizeString.append(QString::number(size) + "|");
            }
            sizeString.chop(1);
            sizeList << sizeString;
        }
    }


    settings.setValue("WindowManager/" + this->objectName() + "-sizes",
                      sizeList);

    // windows types :

    QStringList typeList;

    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() !=
            0) typeList << item.subWindowType();
    }

    settings.setValue("WindowManager/" + this->objectName() + "-types",
                      typeList);
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::applySettings()
{
    QSettings settings;

    // windows :
    QStringList windowNames;

    windowNames = settings.value(
        "WindowManager/" + this->objectName() + "-windows",
        QStringList()).toStringList();

    windowNames.removeAll("");

    // windows sizes :
    QStringList sizeList;
    sizeList = settings.value(
        "WindowManager/" + this->objectName() + "-sizes",
        QStringList()).toStringList();

    // windows types :
    QStringList typeList;
    typeList = settings.value(
        "WindowManager/" + this->objectName() + "-types",
        QStringList()).toStringList();


    int index = 0;

    while (index < windowNames.count() && index <  sizeList.count() &&
           index <  typeList.count())
    {
        // windows types :
        QString typeString = typeList.at(index);


        // windows :

        WindowContainer falseContainer;
        falseContainer.setName(windowNames.at(index));

        if (falseContainer.id() != 0) this->addSubWindow(typeString,
                                                         falseContainer.orientation(),
                                                         falseContainer.parentId(),
                                                         falseContainer.id());

        // windows sizes :

        QString sizeString = sizeList.at(index);
        QStringList parts  =
            sizeString.split("|", QString::SplitBehavior::SkipEmptyParts);

        QList<int> sizeList;

        for (const QString& part : parts) {
            bool ok;
            int  size = part.toInt(&ok);

            if (ok) sizeList << size;
        }


        WindowContainer container(m_windowContainerList.at(index + 1));
        container.setSplitterSizes(sizeList);


        ++index;
    }
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::closeSubWindow(int id)
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
    //    QPointer<PLMBaseSubWindow> window = container.window();
    //    int parentId                      = container.parentId();
    //    Qt::Orientation orientation       = container.orientation();


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
    QStringList names;


    //    // if (!container.parentLayout().isNull()) {
    //    if (itemClone) {
    //        // container.parentLayout()->addLayout(itemClone.layout());
    //    }

    //    QList<QPointer<QSplitter> > list = container.splitterList();
    //    if(list.count() >= 1){

    //    } else {
    //        list.at(list.count() - 2); // second last
    //    }


    // clean all splitters


    QList<QPointer<QSplitter> > list = container.splitterList();

    if (list.count() > 1) {
        QPointer<QSplitter> parentSplitter      = list.last();
        QPointer<QSplitter> grandParentSplitter = list.at(list.count() - 2);

        int index = parentSplitter->indexOf(container.window());

        if (m_windowContainerList.count() == 2) { // one window only
            delete container.window();
        }
        else if (index == 0) {                    // means he has children
                                                  // splitters
            QWidget *sibling = parentSplitter->widget(1);
            grandParentSplitter->addWidget(sibling);
            parentSplitter->close();
            delete parentSplitter;
        } else { // means is end of line
            QWidget *sibling = parentSplitter->widget(0);
            grandParentSplitter->addWidget(sibling);
            parentSplitter->close();
            delete parentSplitter;
        }

        // grandParentSplitter
    }


    int index = 0;

    for (const WindowContainer& item : m_windowContainerList) {
        WindowContainer clone(item);
        QList<QPointer<QSplitter> > list = clone.splitterList();

        // qDebug() << "null w :" << item.window().isNull();
        clone.cleanSplitters();

        // item.setSplitterList(splitterList);
        m_windowContainerList.replace(index, clone);
        ++index;
    }

    m_windowContainerList.removeAll(container);


    this->writeSettings();


    qDebug() << "removed :" << container.name();
    QStringList _names;
    QStringList splitterCount;

    for (const WindowContainer& item : m_windowContainerList) {
        _names << item.name();
        splitterCount << QString::number(item.splitterList().count());
    }
    qDebug() << "names" << _names;
    qDebug() << "splitterCount" << splitterCount;

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

QString PLMBaseSubWindowManager::subWindowTypeById(int id) const
{
    for (const WindowContainer& item : m_windowContainerList) {
        if (item.id() == id) {
            if (item.subWindowType() == "") return this->defaultSubWindowType();
            else return item.subWindowType();
        }
    }

    return QString();
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
    this->setSplitterList(otherContainer.splitterList());
    this->setSubWindowType(otherContainer.subWindowType());
    this->setIsLastActive(otherContainer.isLastActive());
}

WindowContainer::~WindowContainer()
{}

bool WindowContainer::operator!() const {
    return this->m_window.isNull() || this->splitterList().isEmpty();
}

WindowContainer& WindowContainer::operator=(const WindowContainer& otherContainer) {
    this->setWindow(otherContainer.window());
    this->setId(otherContainer.id());
    this->setOrientation(otherContainer.orientation());
    this->setParentId(otherContainer.parentId());
    this->setSplitterList(otherContainer.splitterList());
    this->setSubWindowType(otherContainer.subWindowType());
    this->setIsLastActive(otherContainer.isLastActive());
    return *this;
}

bool WindowContainer::operator==(const WindowContainer& otherContainer) const
{
    if (this->window() != otherContainer.window()) return false;

    if (this->id() != otherContainer.id()) return false;

    if (this->parentId() != otherContainer.parentId()) return false;

    if (this->splitterList() != otherContainer.splitterList()) return false;

    if (this->orientation() != otherContainer.orientation()) return false;

    if (this->subWindowType() != otherContainer.subWindowType()) return false;

    if (this->isLastActive() != otherContainer.isLastActive()) return false;

    return true;
}

WindowContainer::operator bool() const {
    return !this->m_window.isNull() && !this->splitterList().isEmpty();
}

QPointer<PLMBaseSubWindow>WindowContainer::window() const
{
    return m_window;
}

void WindowContainer::setWindow(const QPointer<PLMBaseSubWindow>& window)
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

QList<QPointer<QSplitter> >WindowContainer::splitterList() const
{
    return m_splitterList;
}

void WindowContainer::addSplitter(const QPointer<QSplitter>& splitter)
{
    m_splitterList.append(splitter);
}

void WindowContainer::cleanSplitters()
{
    for (const QPointer<QSplitter>& splitter : m_splitterList) {
        if (splitter.isNull()) m_splitterList.removeAll(splitter);
    }
}

void WindowContainer::setSplitterList(const QList<QPointer<QSplitter> >& splitterList)
{
    m_splitterList = splitterList;
}

QList<int>WindowContainer::splitterSizes() const
{
    if (!m_splitterList.isEmpty()) return m_splitterList.first()->sizes();

    return QList<int>();
}

void WindowContainer::setSplitterSizes(QList<int>sizes)
{
    if (!m_splitterList.isEmpty()) {
        m_splitterList.first()->setSizes(sizes);
    }
}

QString WindowContainer::subWindowType() const
{
    return m_subWindowType;
}

void WindowContainer::setSubWindowType(const QString& subWindowType)
{
    m_subWindowType = subWindowType;
}

bool WindowContainer::isLastActive() const
{
    return m_isLastActive;
}

void WindowContainer::setIsLastActive(bool isLastActive)
{
    m_isLastActive = isLastActive;
}

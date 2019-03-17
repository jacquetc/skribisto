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
#include "plmdata.h"

#include <QVariant>
#include <QDebug>
#include <QSettings>
#include <QTimer>
#include <QHash>


PLMBaseSubWindowManager::PLMBaseSubWindowManager(QBoxLayout    *parentLayout,
                                                 const QString& objectName) : QObject(
        parentLayout), m_parentLayout(parentLayout)
{
    qRegisterMetaType<QList<Widget> >("QList<WindowContainer>");

    this->setObjectName(objectName);


    //    WindowContainer container;
    //    container.setId(0);
    //    container.setParentId(-1);
    //    container.setSubWindowType("");
    //    QPointer<QSplitter> splitter = new
    // QSplitter(Qt::Orientation::Vertical, parent);
    //    splitter->setChildrenCollapsible(false);
    //    container.addSplitter(splitter);
    //    parent->layout()->addWidget(splitter);
    //    container.setOrientation(Qt::Vertical);
    //    m_windowContainerList << container;

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMBaseSubWindowManager::applyUserSettings);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMBaseSubWindowManager::clear);

    QTimer::singleShot(0, this, SLOT(applySettings()));
}

PLMBaseSubWindowManager::~PLMBaseSubWindowManager()
{
    this->writeSettings();
}

bool PLMBaseSubWindowManager::haveOneSubWindow()
{
    for (Widget widget : m_widgetList)
        if (widget.subWindowType() == Widget::SubWindow) return true;

    return false;
}

int PLMBaseSubWindowManager::addSplitter(int parentId, Qt::Orientation orientation)
{
    int newId = -1;

    QHash<QString, QVariant> values;

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        plmdata->userHub()->add(projectId, tableName(), values, newId);
    }

    Widget widget = Widget(tableName(), newId, Widget::Splitter);
    widget.setParentId(parentId);
    widget.setOrientation(orientation);
    m_widgetList.append(widget);

    if (parentId == 0) {
        m_parentLayout->addWidget(widget.getSplitter());
    }
    else {
        for (Widget parentWidget : m_widgetList) {
            if ((parentWidget.id() == parentId) &&
                (parentWidget.subWindowType() == Widget::Splitter)) {
                parentWidget.getSplitter()->addWidget(widget.getSplitter());
                break;
            }
        }
    }


    return newId;
}

int PLMBaseSubWindowManager::addSubWindow(int parentId)
{
    int newId = -1;

    QHash<QString, QVariant> values;

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        plmdata->userHub()->add(projectId, tableName(), values, newId);
    }

    Widget widget = Widget(tableName(), newId, Widget::SubWindow);
    widget.setParentId(parentId);
    m_widgetList.append(widget);


    if (parentId == 0) {
        // can't be directly added to m_parentLayout, so creating a
        // splitter between. Adapt if one splitter is found
        int splitterId = -1;

        for (Widget widget : m_widgetList) {
            if ((widget.subWindowType() == Widget::Splitter) &&
                (widget.parentId() == 0)) {
                splitterId = widget.id();
            }
        }

        if (splitterId == -1) {
            splitterId = this->addSplitter(0, Qt::Horizontal);
        }
        Widget splitterWidget = this->getWidgetById(splitterId);
        m_parentLayout->addWidget(widget.getSplitter());
        parentId = splitterId;
    }

    for (Widget parentWidget : m_widgetList) {
        if ((parentWidget.id() == parentId) &&
            (parentWidget.subWindowType() == Widget::Splitter)) {
            parentWidget.getSplitter()->addWidget(widget.getWindow());
            break;
        }
    }

    return newId;
}

///
/// \brief PLMBaseSubWindowManager::getWindowByType
/// \param subWindowType
/// \return
/// get the one with focus else get the first. Else create one subwindow
PLMBaseSubWindow * PLMBaseSubWindowManager::getWindowByType(const QString& subWindowType)
{
    //    QList<PLMBaseSubWindow *> list;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.subWindowType() == subWindowType) {
    //            if (item.isLastActive()) return item.window();
    //        }
    //    }

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.subWindowType() == subWindowType) list << item.window();
    //    }

    //    if (!list.isEmpty()) return list.last();

    //    return this->addSubWindow(subWindowType, Qt::Horizontal, 0);
}

// ---------------------------------------------------------

void PLMBaseSubWindowManager::setLastActiveWindowByType(int id)
{
    //    QString subWindowType = subWindowTypeById(id);

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.subWindowType() == subWindowType) {
    //            WindowContainer clone(item);

    //            if (item.id() == id) {
    //                clone.setIsLastActive(true);
    //            }
    //            else {
    //                clone.setIsLastActive(false);
    //            }
    //
    //
    //       m_windowContainerList.replace(m_windowContainerList.indexOf(item),
    // clone);
    //        }
    //    }
}

// ---------------------------------------------------------

PLMBaseSubWindow * PLMBaseSubWindowManager::getfirstSubWindow()
{
    for (Widget widget : m_widgetList) {
        if (widget.subWindowType() == Widget::SubWindow) {
            return widget.getWindow().data();
        }
    }

    // else if no SubWindow
    return this->getSubWindowById(addSubWindow(0));
}

// ---------------------------------------------------------

int PLMBaseSubWindowManager::addSubWindow(Qt::Orientation orientation,
                                          int             callerSubWindowId)
{
    // is a subWindow :
    Widget callerWidget = this->getWidgetById(callerSubWindowId);


    int newId = this->addSubWindow(callerWidget.parentId());


    // set orientation of parent splitter :
    Widget callerWidgetParent = this->getWidgetById(callerWidget.parentId());

    callerWidgetParent.getSplitter()->setOrientation(orientation);

    return newId;


    //    QBoxLayout::Direction boxOrientation =
    //        QBoxLayout::Direction::TopToBottom;

    //    if (orientation == Qt::Vertical) boxOrientation =
    //            QBoxLayout::Direction::TopToBottom;

    //    if (orientation ==
    //        Qt::Horizontal) boxOrientation =
    //            QBoxLayout::Direction::LeftToRight;


    //    WindowContainer container;

    //    if (forcedId == -2) {
    //        container.setId(m_lastNumber + 1);
    //    }
    //    else {
    //        container.setId(forcedId);
    //    }

    //    if (container.id() > m_lastNumber) m_lastNumber = container.id();

    //    container.setOrientation(orientation);
    //    container.setParentId(parentZone);

    //    QPointer<PLMBaseSubWindow> parentWindow = nullptr;
    //    QPointer<QSplitter> parentSplitter      = nullptr;
    //    WindowContainer     parentContainer;
    //    int i = 0;


    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() == parentZone) {
    //            parentSplitter  = item.splitterList().last();
    //            parentWindow    = item.window();
    //            parentContainer = WindowContainer(item);
    //            break;
    //        }
    //        ++i;
    //    }

    //    container.addSplitter(parentSplitter);

    //    QSplitter *splitter = new QSplitter(orientation, nullptr);
    //    splitter->setChildrenCollapsible(false);

    //    container.addSplitter(splitter);
    //    parentContainer.addSplitter(splitter);


    //    m_windowContainerList.replace(i, parentContainer);


    //    PLMBaseSubWindow *window = this->subWindowByName(subWindowType,
    // container.id());
    //    window->setObjectName(container.name());
    //    container.setWindow(window);
    //    container.setSubWindowType(subWindowType);

    //    if (parentZone != 0) {
    //        int index = parentSplitter->indexOf(parentWindow);
    //        splitter->addWidget(parentWindow);
    //        parentSplitter->insertWidget(index, splitter);
    //    }
    //    else {
    //        parentSplitter->addWidget(splitter);
    //    }
    //    splitter->addWidget(window);

    //    m_windowContainerList << container;

    //    connect(window, &PLMBaseSubWindow::subWindowClosed, this,
    //            &PLMBaseSubWindowManager::closeSubWindow);

    //    connect(window,
    //            &PLMBaseSubWindow::splitCalled, [this](Qt::Orientation
    // orientation,
    //                                                   int parentZone) {
    //        this->addSubWindow(subWindowTypeById(parentZone), orientation,
    // parentZone);
    //    }
    //            );

    //    connect(window,
    //            &PLMBaseSubWindow::subWindowFocusActived,
    //            this,
    //            &PLMBaseSubWindowManager::setLastActiveWindowByType);

    //    qDebug() << "added :" << container.name();
    //    QStringList names;
    //    QStringList splitterCount;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        names << item.name();
    //        splitterCount << QString::number(item.splitterList().count());
    //    }
    //    qDebug() << "names" << names;
    //    qDebug() << "splitterCount" << splitterCount;

    // reorder


    //    this->writeSettings();

    //    return window;
    // --------------------------------------------
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

PLMBaseSubWindow * PLMBaseSubWindowManager::getSubWindowById(int id)
{
    if (this->getSubWindowTypeById(id) ==
        Widget::SubWindow) return this->getWidgetById(id).getWindow();

    // else
    if (this->haveOneSubWindow()) return this->getfirstSubWindow();

    return
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::writeSettings()
{
    //    QSettings settings;

    //    // windows :
    //    QStringList windowNames;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() != 0) windowNames << item.name();
    //    }

    //    settings.setValue("WindowManager/" + this->objectName() + "-windows",
    //                      windowNames);

    //    // windows sizes :
    //    QStringList sizeList;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() !=
    //            0) {
    //            QList<int> sizes = item.splitterSizes();
    //            QString    sizeString;

    //            for (const int& size : sizes) {
    //                sizeString.append(QString::number(size) + "|");
    //            }
    //            sizeString.chop(1);
    //            sizeList << sizeString;
    //        }
    //    }


    //    settings.setValue("WindowManager/" + this->objectName() + "-sizes",
    //                      sizeList);

    //    // windows types :

    //    QStringList typeList;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() !=
    //            0) typeList << item.subWindowType();
    //    }

    //    settings.setValue("WindowManager/" + this->objectName() + "-types",
    //                      typeList);
}

void PLMBaseSubWindowManager::applyUserSettings()
{
    // do nothing if more than one project is opened
    if (plmdata->projectHub()->getProjectIdList().count() > 1) return;

    // apply settings from first project :
    int projectId = plmdata->projectHub()->getProjectIdList().first();

    QList<int> result;
    plmdata->userHub()->getIds(projectId, tableName(), result);

    this->clear();

    for (int id : result) {
        Widget::SubWindowType subWindowType = plmdata->userHub()->get(projectId,
                                                                      tableName(),
                                                                      id,
                                                                      "l_type").value<Widget::SubWindowType>();
        m_widgetList.append(Widget(tableName(), id, subWindowType));
    }

    for (Widget widget : m_widgetList) {
        if (widget.subWindowType() == Widget::Splitter) {
            QPointer<QSplitter> parentSplitter = widget.getSplitter();

            if (widget.parentId() == 0) {
                m_parentLayout->addWidget(parentSplitter);
            }

            for (int childId : widget.getChildWidgetList()) {
                for (Widget childWidget : m_widgetList) {
                    if (childWidget.id() == childId) {
                        if (childWidget.subWindowType() == Widget::Splitter) {
                            parentSplitter->addWidget(childWidget.getSplitter());
                        }
                        else if (childWidget.subWindowType() == Widget::SubWindow) {
                            parentSplitter->addWidget(childWidget.getWindow());
                        }
                    }
                }
            }
        }
    }

    // set sizes :
    for (Widget widget : m_widgetList) {
        if (widget.subWindowType() == Widget::Splitter) {
            QPointer<QSplitter> parentSplitter = widget.getSplitter();
            parentSplitter->setSizes(widget.splitterSizes());
        }
    }


    // if no window saved, create one. And one widget saved means problem
    if (result.count() <= 1) this->addSubWindow(0);
}

void PLMBaseSubWindowManager::clear()
{
    for (Widget widget : m_widgetList) {
        if (widget.parentId() == 0) widget.getSplitter()->close();
        widget.getSplitter()->deleteLater();
    }
    m_widgetList.clear();
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::writeUserSettingsOnOtherProjects()
{
    for (Widget widget : m_widgetList) {
        if (widget.parentId() == 0) widget.getSplitter()->close();
        widget.getSplitter()->deleteLater();
    }
    m_widgetList.clear();
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::applySettings()
{
    //    QSettings settings;

    //    // windows :
    //    QStringList windowNames;

    //    windowNames = settings.value(
    //        "WindowManager/" + this->objectName() + "-windows",
    //        QStringList()).toStringList();

    //    windowNames.removeAll("");

    //    // windows sizes :
    //    QStringList sizeList;
    //    sizeList = settings.value(
    //        "WindowManager/" + this->objectName() + "-sizes",
    //        QStringList()).toStringList();

    //    // windows types :
    //    QStringList typeList;
    //    typeList = settings.value(
    //        "WindowManager/" + this->objectName() + "-types",
    //        QStringList()).toStringList();


    //    int index = 0;

    //    while (index < windowNames.count() && index <  sizeList.count() &&
    //           index <  typeList.count())
    //    {
    //        // windows types :
    //        QString typeString = typeList.at(index);


    //        // windows :

    //        WindowContainer falseContainer;
    //        falseContainer.setName(windowNames.at(index));

    //        if (falseContainer.id() != 0) this->addSubWindow(typeString,
    //
    //
    //
    //
    //
    //                                           falseContainer.orientation(),
    //
    //
    //
    //
    //                                              falseContainer.parentId(),
    //
    //
    //                                                    falseContainer.id());

    //        // windows sizes :

    //        QString sizeString = sizeList.at(index);
    //        QStringList parts  =
    //            sizeString.split("|", QString::SplitBehavior::SkipEmptyParts);

    //        QList<int> sizeList;

    //        for (const QString& part : parts) {
    //            bool ok;
    //            int  size = part.toInt(&ok);

    //            if (ok) sizeList << size;
    //        }


    //        WindowContainer container(m_windowContainerList.at(index + 1));
    //        container.setSplitterSizes(sizeList);


    //        ++index;
    //    }
}

// ------------------------------------------------------------

void PLMBaseSubWindowManager::closeSubWindow(int id)
{
    //    WindowContainer container;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.id() == id) {
    //            container = item;
    //        }
    //    }

    //    if (!container) return;

    //    //    QPointer<QBoxLayout> layout       = container.layout();
    //    //    QPointer<QBoxLayout> parentLayout = container.parentLayout();
    //    //    QPointer<PLMBaseSubWindow> window = container.window();
    //    //    int parentId                      = container.parentId();
    //    //    Qt::Orientation orientation       = container.orientation();


    //    WindowContainer itemClone;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        if (item.parentId() == id) {
    //            itemClone = WindowContainer(item);
    //            itemClone.setOrientation(container.orientation());
    //            itemClone.setParentId(container.parentId());

    //            //
    //            itemClone.setLayoutList(container.parentLayout());
    //
    //
    //       m_windowContainerList.replace(m_windowContainerList.indexOf(item),
    //                                          itemClone);
    //        }
    //    }
    //    QStringList names;


    //    //    // if (!container.parentLayout().isNull()) {
    //    //    if (itemClone) {
    //    //        // container.parentLayout()->addLayout(itemClone.layout());
    //    //    }

    //    //    QList<QPointer<QSplitter> > list = container.splitterList();
    //    //    if(list.count() >= 1){

    //    //    } else {
    //    //        list.at(list.count() - 2); // second last
    //    //    }


    //    // clean all splitters


    //    QList<QPointer<QSplitter> > list = container.splitterList();

    //    if (list.count() > 1) {
    //        QPointer<QSplitter> parentSplitter      = list.last();
    //        QPointer<QSplitter> grandParentSplitter = list.at(list.count() -
    // 2);

    //        int index = parentSplitter->indexOf(container.window());

    //        if (m_windowContainerList.count() == 2) { // one window only
    //            delete container.window();
    //        }
    //        else if (index == 0) {                    // means he has children
    //                                                  // splitters
    //            QWidget *sibling = parentSplitter->widget(1);
    //            grandParentSplitter->addWidget(sibling);
    //            parentSplitter->close();
    //            delete parentSplitter;
    //        } else { // means is end of line
    //            QWidget *sibling = parentSplitter->widget(0);
    //            grandParentSplitter->addWidget(sibling);
    //            parentSplitter->close();
    //            delete parentSplitter;
    //        }

    //        // grandParentSplitter
    //    }


    //    int index = 0;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        WindowContainer clone(item);
    //        QList<QPointer<QSplitter> > list = clone.splitterList();

    //        // qDebug() << "null w :" << item.window().isNull();
    //        clone.cleanSplitters();

    //        // item.setSplitterList(splitterList);
    //        m_windowContainerList.replace(index, clone);
    //        ++index;
    //    }

    //    m_windowContainerList.removeAll(container);


    //    this->writeSettings();


    //    qDebug() << "removed :" << container.name();
    //    QStringList _names;
    //    QStringList splitterCount;

    //    for (const WindowContainer& item : m_windowContainerList) {
    //        _names << item.name();
    //        splitterCount << QString::number(item.splitterList().count());
    //    }
    //    qDebug() << "names" << _names;
    //    qDebug() << "splitterCount" << splitterCount;

    // -------------------------------------------------------

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

Widget::SubWindowType PLMBaseSubWindowManager::getSubWindowTypeById(int id) const
{
    for (const Widget& item : m_widgetList) {
        if (item.id() == id) {
            return item.subWindowType();
        }
    }

    return Widget::SubWindow;
}

Widget PLMBaseSubWindowManager::getWidgetById(int id)
{
    for (const Widget& item : m_widgetList) {
        if (item.id() == id) {
            return item;
        }
    }

    return Widget();
}

// ---------------------------------------------------------
int PLMBaseSubWindowManager::getFreeNumber()
{
    int freeId = 1;

    for (const Widget& item : m_widgetList) {
        if (item.id() != freeId) {
            break;
        }
        freeId += 1;
    }

    return freeId;
}

// 1V0-2V1-3V1-4H2

// ---------------------------------------------------------
// ---------------------------------------------------------
// ---------------------------------------------------------

Widget::Widget() {
    m_tableName = "";
    m_id        = -1;
}

Widget::Widget(const QString& tableName, int id, Widget::SubWindowType type) {
    m_tableName     = tableName;
    m_id            = id;
    m_subWindowType = type;
    loadValues();
}

Widget::Widget(const Widget& otherWidget) {
    m_id              = otherWidget.id();
    m_parentId        = otherWidget.parentId();
    m_subWindowType   = otherWidget.subWindowType();
    m_childWidgetList = otherWidget.getChildWidgetList();

    if (m_subWindowType == SubWindowType::Splitter) {
        m_orientation   = otherWidget.getOrientation();
        m_splitterSizes = otherWidget.splitterSizes();
        this->setSplitter(otherWidget.getSplitter());
    }
    else {
        this->setWindow(otherWidget.getWindow());
    }
}

Widget::~Widget()
{}

Widget& Widget::operator=(const Widget& otherWidget) {
    m_tableName   = otherWidget.tableName();
    m_id          = otherWidget.id();
    m_splitter    = otherWidget.getSplitter();
    m_window      = otherWidget.getWindow();
    m_orientation = otherWidget.getOrientation();
    return *this;
}

bool Widget::operator==(const Widget& otherContainer) const
{
    if (this->id() != otherContainer.id()) return false;

    if (this->parentId() != otherContainer.parentId()) return false;

    if (this->splitterSizes() != otherContainer.splitterSizes()) return
            false;

    if (this->getOrientation() != otherContainer.getOrientation()) return false;

    if (this->subWindowType() != otherContainer.subWindowType()) return
            false;

    if (this->getChildCount() != otherContainer.getChildCount()) return
            false;

    if (this->getWindow() != otherContainer.getWindow()) return false;

    if (this->getSplitter() != otherContainer.getSplitter()) return false;

    return true;
}

int Widget::parentId() const
{
    return m_parentId;
}

void Widget::setParentId(int value)
{
    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMError error = plmdata->userHub()->set(projectId,
                                                 m_tableName,
                                                 m_id,
                                                 "l_parent",
                                                 value,
                                                 true);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO;
        }
    }
    m_parentId = value;
}

int Widget::id() const
{
    return m_id;
}

void Widget::setId(int value)
{
    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMError error = plmdata->userHub()->set(projectId,
                                                 m_tableName,
                                                 m_id,
                                                 "l_id",
                                                 value,
                                                 true);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO;
        }
    }
    m_id = value;
}

void Widget::addChildWidget(int id)
{
    if (!m_childWidgetList.contains(id)) {
        m_childWidgetList.append(id);

        for (int projectId : plmdata->projectHub()->getProjectIdList()) {
            PLMError error = plmdata->userHub()->set(projectId,
                                                     m_tableName,
                                                     m_id,
                                                     "m_children",
                                                     QVariant::fromValue<QList<int> >(
                                                         m_childWidgetList),
                                                     true);

            if (error) {
                qDebug() << "error :" << Q_FUNC_INFO;
            }
        }
    }
}

int Widget::getChildWidgetId(int index) const
{
    if (m_childWidgetList.count() > index) return m_childWidgetList.at(index);

    return -1;
}

QList<int>Widget::getChildWidgetList() const
{
    return m_childWidgetList;
}

int Widget::getChildCount() const
{
    return m_childWidgetList.count();
}

QString Widget::tableName() const
{
    return m_tableName;
}

void Widget::setSplitterSizes(const QList<int>sizes)
{
    if (m_subWindowType == Widget::SubWindow) return;

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMError error = plmdata->userHub()->set(projectId,
                                                 m_tableName,
                                                 m_id,
                                                 "m_sizes",
                                                 QVariant::fromValue<QList<int> >(sizes),
                                                 true);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO;
        }
    }
    m_splitter->setSizes(sizes);
}

QList<int>Widget::splitterSizes() const
{
    if (m_subWindowType == Widget::SubWindow) return QList<int>();

    return m_splitter->sizes();
}

void Widget::setSubWindowType(const SubWindowType& type)
{
    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMError error = plmdata->userHub()->set(projectId,
                                                 m_tableName,
                                                 m_id,
                                                 "l_type",
                                                 type,
                                                 true);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO;
        }
    }

    m_subWindowType = type;
}

Widget::SubWindowType Widget::subWindowType() const
{
    return m_subWindowType;
}

QPointer<PLMBaseSubWindow>Widget::getWindow() const
{
    return m_window;
}

void Widget::setWindow(const QPointer<PLMBaseSubWindow>& window)
{
    m_window = window;
}

QPointer<QSplitter>Widget::getSplitter() const
{
    return m_splitter;
}

void Widget::setSplitter(const QPointer<QSplitter>& splitter)
{
    m_splitter = splitter;
}

void Widget::loadValues()
{
    if ((m_id == -1) || m_tableName.isEmpty()) return;

    QStringList valueList;
    valueList << "l_type" << "m_children" << "l_parent" << "m_sizes" << "l_orientation";

    QHash<QString, QVariant> result;

    int projectId = plmdata->projectHub()->getProjectIdList().first();
    plmdata->userHub()->getMultipleValues(projectId, m_tableName, m_id, valueList,
                                          result);

    m_parentId        = result.value("l_parent", QVariant::fromValue(-1)).toInt();
    m_childWidgetList =
        result.value("m_children",
                     QVariant::fromValue(QList<int>())).value<QList<int> >();

    if (m_subWindowType == SubWindowType::Splitter) {
        if (!m_splitter) setSplitter(new QSplitter());

        QList<int> sizes = result.value("m_sizes",
                                        QVariant::fromValue(
                                            QList<int>())).value<QList<int> >();
        m_splitterSizes = sizes;
        m_splitter->setSizes(sizes);

        Qt::Orientation orientation = result.value("l_orientation",
                                                   QVariant::fromValue(
                                                       Qt::Horizontal)).value<Qt::Orientation>();
        m_orientation = orientation;
        m_splitter->setOrientation(orientation);
    }
    else {
        if (!m_window) setWindow(new PLMBaseSubWindow());
    }
}

void Widget::setOrientation(const Qt::Orientation& orientation)
{
    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMError error = plmdata->userHub()->set(projectId,
                                                 m_tableName,
                                                 m_id,
                                                 "l_orientation",
                                                 orientation,
                                                 true);

        if (error) {
            qDebug() << "error :" << Q_FUNC_INFO;
        }
    }

    if (m_splitter) m_splitter->setOrientation(orientation);
    m_orientation = orientation;
}

Qt::Orientation Widget::getOrientation() const
{
    return m_orientation;
}

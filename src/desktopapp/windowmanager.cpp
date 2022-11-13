#include "mainwindow.h"
#include "skrdata.h"
#include "windowmanager.h"

#include <QSettings>

WindowManager::WindowManager(QObject *parent)
    : QObject{parent}
{

}

//----------------------------------------------

WindowManager::~WindowManager()
{

}
//----------------------------------------------

WindowManager *WindowManager::m_instance = nullptr;

//----------------------------------------------
int WindowManager::subscribeWindow(MainWindow *windowObject)
{
    QObject::connect(windowObject, &QObject::destroyed, this, &WindowManager::unSubscribeWindow);

    m_windowList.append(windowObject);
    emit windowIdsChanged();
    return getWindowId(windowObject);
}
//----------------------------------------------

void WindowManager::unSubscribeWindow(QObject *windowObject)
{
    m_windowList.removeAll(windowObject);

    for (int i=0; i> m_windowList.count() ; i++ ) {
        QObject *object = m_windowList.at(i);
        object->setProperty("windowId", i);
    }

    emit windowIdsChanged();
}
//----------------------------------------------

MainWindow *WindowManager::restoreWindows()
{
    MainWindow *window = nullptr;

    QSettings settings;
    int numberOfWindows = settings.value("window/numberOfWindows", 1).toInt();

    for (int i = 0; i < numberOfWindows; i++) {
        window = addEmptyWindow(true);
    }

    return window;
}

//----------------------------------------------

int WindowManager::getWindowId(MainWindow *windowObject)
{
    return m_windowList.indexOf(windowObject);

}

//----------------------------------------------

int WindowManager::getNumberOfWindows()
{
    return m_windowList.count();

}

//----------------------------------------------

void WindowManager::closeAllWindows(){

    QSettings settings;
    qDebug() << this->getNumberOfWindows();
    settings.setValue("window/numberOfWindows", this->getNumberOfWindows());

    for (MainWindow *window : m_windowList) {

        this->closeWindow(window);
    }
}

//----------------------------------------------

void WindowManager::closeWindow(MainWindow *window){


    QSettings settings;

    settings.beginGroup("window_" + QString::number(window->windowId()));
    settings.setValue("geometry", window->saveGeometry());
    settings.setValue("dockState", window->saveState());
    settings.setValue("visibility", window->windowState().toInt());
    settings.endGroup();

    m_windowList.removeAll(window);

    emit window->aboutToBeDestroyed();
    window->deleteLater();
}

//----------------------------------------------

MainWindow *WindowManager::addEmptyWindow(bool restoreViewEnabled)
{
 return addWindow(restoreViewEnabled);
}

//----------------------------------------------

void WindowManager::addWindowForItemId(int projectId, int treeItemId)
{

    QString type = skrdata->treeHub()->getType(projectId, treeItemId);

    addWindow(false, type, projectId, treeItemId);
}
//----------------------------------------------

void WindowManager::addWindowForProjectIndependantPageType(const QString &pageType)
{
    addWindow(false, pageType);
}
//----------------------------------------------

void WindowManager::addWindowForProjectDependantPageType(int projectId, const QString &pageType)
{
    addWindow(false, pageType,projectId);

}
//----------------------------------------------

void WindowManager::insertAdditionalProperty(const QString &key, const QVariant &value)
{

}
//----------------------------------------------

void WindowManager::insertAdditionalPropertyForViewManager(const QString &key, const QVariant &value)
{

}

MainWindow * WindowManager::addWindow(bool restoreViewEnabled, const QString &pageType, int projectId, int treeItemId)
{
    int nextFreeWindowId = m_windowList.count();

    MainWindow *window = new MainWindow(nextFreeWindowId, restoreViewEnabled);
    this->subscribeWindow(window);


    QSettings settings;
    settings.beginGroup("window_" + QString::number(nextFreeWindowId));
    auto dockState = settings.value("dockState", QByteArray()).toByteArray();
    auto geometry = settings.value("geometry", QByteArray()).toByteArray();
    Qt::WindowStates visibility = Qt::WindowStates::fromInt(settings.value("visibility", Qt::WindowNoState).toInt());
    settings.endGroup();


    if (geometry.isEmpty())
        window->setGeometry(200, 200, 800, 800);
    else
        window->restoreGeometry(geometry);

    window->setWindowState(visibility);



    if(!pageType.isEmpty() || projectId >=0 || treeItemId >= 0)
        window->viewManager()->openSpecificView(pageType, projectId, treeItemId);

    window->show();
    window->raise();
    window->restoreState(dockState);

    return window;
}

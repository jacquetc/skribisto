#include "plmwindow.h"
#include "plmwritewindow.h"
#include "plugininterface.h"

#include <QTimer>

PLMWriteWindow::PLMWriteWindow(QObject *parent) : QObject(parent),
                                                m_name("writeWindow")
{
    this->setProperty("name", m_name);

   // QTimer::singleShot(0, this, SLOT(init()));
}

// -------------------------------------------------------------------

void PLMWriteWindow::init()
{


    PLMPluginLoader *loader = PLMPluginLoader::instance();
    loader->addPluginType<PLMWriteLeftDockInterface>();

}

// -------------------------------------------------------------------

PLMWriteWindow::~PLMWriteWindow()
{}

// -------------------------------------------------------------------

QString PLMWriteWindow::use() const
{
    return m_name;
}

// -------------------------------------------------------------------

PLMBaseWindow *PLMWriteWindow::window()
{
    PLMWindow *window = new PLMWindow;

    window->setProperty("name", m_name);
    window->setWindowTitle(tr("Write"));

    return window;
}


// -------------------------------------------------------------------

QList<PLMSideBarAction>PLMWriteWindow::sideMainBarActions(QObject *parent)
{
    QList<PLMSideBarAction> list;
    QAction *action = new QAction(QIcon(":/pics/48x48/scribus.png"), tr("Write"), parent);
    action->setProperty("linkedWindow", m_name);
    action->setProperty("detachable", true);
    action->setProperty("order", 20);
    action->setCheckable(true);
    PLMSideBarAction mAction(m_name, action);
    list.append(mAction);
    return list;
}

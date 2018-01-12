#include "plmwindow.h"
#include "plmwelcomewindow.h"
#include "plugininterface.h"

#include <QTimer>

PLMWelcomeWindow::PLMWelcomeWindow(QObject *parent) : QObject(parent),
                                                m_name("welcomeWindow")
{
    this->setProperty("name", m_name);

    QTimer::singleShot(0, this, SLOT(init()));
}

// -------------------------------------------------------------------

void PLMWelcomeWindow::init()
{
    PluginInterface::addPlugins();

}

// -------------------------------------------------------------------

PLMWelcomeWindow::~PLMWelcomeWindow()
{}

// -------------------------------------------------------------------

QString PLMWelcomeWindow::use() const
{
    return m_name;
}

// -------------------------------------------------------------------

PLMBaseWindow *PLMWelcomeWindow::window()
{
    PLMWindow *window = new PLMWindow;

    window->setProperty("name", m_name);
    window->setWindowTitle(tr("Welcome"));

    return window;
}


// -------------------------------------------------------------------

QList<PLMSideBarAction>PLMWelcomeWindow::sideMainBarActions(QObject *parent)
{
    QList<PLMSideBarAction> list;
    QAction *action = new QAction(QIcon(":/pics/48x48/scribus.png"), tr("Welcome"), parent);
    action->setProperty("linkedWindow", m_name);
    action->setProperty("detachable", true);
    action->setProperty("order", 10);
    action->setCheckable(true);
    PLMSideBarAction mAction(m_name, action);
    list.append(mAction);
    return list;
}

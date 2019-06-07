#include "plmwindow.h"
#include "plmwelcomewindow.h"
#include "plugininterface.h"

#include <QTimer>

PLMWelcomeWindow::PLMWelcomeWindow(QObject *parent) : QObject(parent),
    m_name("WelcomeWindow")
{
    this->setProperty("name", m_name);

    // QTimer::singleShot(0, this, SLOT(init()));
}

// -------------------------------------------------------------------

void PLMWelcomeWindow::init()
{
    //    PluginInterface::addPlugins();
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

PLMBaseWindow * PLMWelcomeWindow::window()
{
    PLMWindow *window = new PLMWindow(nullptr, m_name);

    // window->setProperty("name", m_name);

    return window;
}

// -------------------------------------------------------------------

QList<PLMSideBarAction>PLMWelcomeWindow::sideMainBarActions(QObject *parent)
{
    QList<PLMSideBarAction> list;
    QAction *action =
        new QAction(QIcon(":/pics/plume-creator.png"), tr("Welcome"), parent);
    action->setProperty("linkedWindow", m_name);
    action->setProperty("detachable",   false);
    action->setProperty("order",        10);
    action->setCheckable(true);
    PLMSideBarAction mAction(m_name, action);
    list.append(mAction);
    return list;
}

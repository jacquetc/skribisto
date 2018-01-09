#include "plmpanel.h"
#include "plmtestwindow.h"
#include "testwritingzone.h"

PLMTestWindow::PLMTestWindow(QObject *parent) : QObject(parent),
                                                m_name("testWindow")
{
    this->setProperty("name", m_name);
}

// -------------------------------------------------------------------

PLMTestWindow::~PLMTestWindow()
{}

// -------------------------------------------------------------------

QString PLMTestWindow::panelName() const
{
    return m_name;
}

// -------------------------------------------------------------------

PLMPanelWindow * PLMTestWindow::panel()
{
    PLMPanel *window = new PLMPanel;

    window->setObjectName(m_name);
    window->setProperty("name", m_name);
    window->setWindowTitle(tr("Test"));

    // window->setCentralWidget(new QLabel("Window test"));
    window->setCentralWidget(new TestWritingZone());
    return window;
}

// -------------------------------------------------------------------

QList<PLMSideBarAction>PLMTestWindow::mainBarActions(QObject *parent)
{
    QList<PLMSideBarAction> list;
    QAction *action =
        new QAction(QIcon(":/pics/48x48/scribus.png"), tr("Test Window"), parent);
    action->setProperty("linkedPanel", m_name);
    PLMSideBarAction mAction("testwindow", action);
    list.append(mAction);
    return list;
}

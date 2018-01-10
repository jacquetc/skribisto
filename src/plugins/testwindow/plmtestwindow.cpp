#include "plmwindow.h"
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

QString PLMTestWindow::use() const
{
    return m_name;
}

// -------------------------------------------------------------------

PLMBaseWindow *PLMTestWindow::window()
{
    PLMWindow *window = new PLMWindow;

    window->setProperty("name", m_name);
    window->setWindowTitle(tr("Test"));

    window->setCentralWidget(new TestWritingZone());
    return window;
}


// -------------------------------------------------------------------

QList<PLMSideBarAction>PLMTestWindow::sideMainBarActions(QObject *parent)
{
    QList<PLMSideBarAction> list;
    QAction *action = new QAction(QIcon(":/pics/48x48/scribus.png"), tr("Test Window"), parent);
    action->setProperty("linkedWindow", m_name);
    action->setProperty("detachable", true);
    action->setCheckable(true);
    PLMSideBarAction mAction(m_name, action);
    list.append(mAction);
    return list;
}

#include "plmwindow.h"
#include "ui_plmwindow.h"
#include "plmdata.h"
#include "plmbasedock.h"

#include <QSettings>
#include <QTimer>
#include <QToolButton>

// #include "menubar.h"

PLMWindow::PLMWindow(QWidget *parent, const QString& name) :
    PLMBaseWindow(parent, name),
    ui(new Ui::PLMWindow)
{
    ui->setupUi(this);

    this->setupStatusBar();

    this->setRightDockDefaultCount(0);
    this->setBottomDockDefaultCount(0);
    this->setLeftDockDefaultCount(2);

    this->setMenuActions();
}

// -------------------------------------------------------------------

PLMWindow::~PLMWindow()
{
    delete ui;
}

// -------------------------------------------------------------------

void PLMWindow::setMenuActions()
{
    // Project Menu :

    // QAction *actionSave;
    // actionSave = new QAction(menuBar);
    // actionSave->setObjectName(QStringLiteral("actionSave"));
    // actionSave->setText(QApplication::translate("WritePanel", "&Save", 0));
    // actionSave->setShortcut(QApplication::translate("WritePanel", "Ctrl+S",
    // 0));
    // menuBar->addActionToMenu(actionSave, MenuBar::Project, "actionSaveAll");

    // connect(actionSave, SIGNAL(triggered()), this,
    // SLOT(actionSave_triggered()));

    // Edit Menu :

    QAction *menuAdvanced;

    menuAdvanced = new QAction(tr("Advanced"), this);
    menuAdvanced->setProperty("path", "/" + tr("Blabla"));

    //    menuAdvanced->addAction(actionFind_Replace_in_project);

    actionList << menuAdvanced;

    //    // Sheet Menu :

    //    QAction *actionView_Source;

    //    actionView_Source = new QAction(menuBar);
    //    actionView_Source->setObjectName(QStringLiteral("actionView_Source"));
    // actionView_Source->setCheckable(true);

    //    menuBar->addActionToMenu(actionView_Source, MenuBar::Sheet);
    //    actionView_Source->setText(QApplication::translate("WritePanel",
    // "Source", 0));

    //    connect(actionView_Source, SIGNAL(triggered(bool)),
    // ui->mainTextWritingZone, SLOT(setSourceEditEnabled(bool)));

    //    QAction *actionView_MiniMap;

    //    actionView_MiniMap = new QAction(menuBar);
    //
    //  actionView_MiniMap->setObjectName(QStringLiteral("actionView_MiniMap"));
    // actionView_MiniMap->setCheckable(true);

    //    menuBar->addActionToMenu(actionView_MiniMap, MenuBar::Sheet);
    //    actionView_MiniMap->setText(QApplication::translate("WritePanel",
    // "MiniMap", 0));

    //    connect(actionView_MiniMap, SIGNAL(triggered(bool)),
    // ui->mainTextWritingZone, SLOT(setMiniMapEnabled(bool)));

    // View Menu :

    //    QAction *actionAdd_Dock;

    //    actionAdd_Dock = new QAction(menuBar);
    //    actionAdd_Dock->setObjectName(QStringLiteral("actionAdd_Dock"));
    //  this->connect(actionAdd_Dock, SIGNAL(triggered()), dockManager,
    // SLOT(addDock()), Qt::UniqueConnection);

    //          QAction *actionHide_Show_Docks;
    //    actionHide_Show_Docks = new QAction(menuBar);
    //
    //  actionHide_Show_Docks->setObjectName(QStringLiteral("actionHide_Show_Docks"));

    //    menuBar->addSectionToMenu("", MenuBar::View);
    //    menuBar->addActionToMenu(actionAdd_Dock, MenuBar::View);
    //    menuBar->addActionToMenu(actionHide_Show_Docks, MenuBar::View);

    //    actionAdd_Dock->setText(QApplication::translate("WritePanel", "Add
    // Dock", 0));
    //    actionHide_Show_Docks->setText(QApplication::translate("WritePanel",
    // "Hide / Show Docks", 0));

    // apply current menubar settings :
}

void PLMWindow::setupStatusBar()
{
    QAction *showLeftDockAct =
        new QAction(QIcon(":/pics/plume-creator.svg"), tr("Show Left Sidebar"), this);

    showLeftDockAct->setCheckable(true);
    connect(showLeftDockAct, &QAction::toggled, [showLeftDockAct](bool value) {
        if (value) {
            showLeftDockAct->setText(tr("Hide Left Sidebar"));
            showLeftDockAct->setToolTip(tr("Hide Left Sidebar"));
        }
        else { showLeftDockAct->setText(tr("Show Left Sidebar"));
               showLeftDockAct->setToolTip(tr("Show Left Sidebar")); }
    });

    connect(showLeftDockAct, &QAction::toggled, this,
            &PLMWindow::setLeftSidebarVisible);

    QToolButton *showLeftDock = new QToolButton();
    showLeftDock->setAutoRaise(true);
    showLeftDock->setDefaultAction(showLeftDockAct);
    ui->statusBar->addWidget(showLeftDock);
}

// ---------------------------------------------------------------------------------------------------

// void TestWindow::actionSave_triggered()
// {
//    cmd("SHEET --save=\"" + ui->mainTextWritingZone->currentSheet()->path() +
// "\"");

// }

// ---------------------------------------------------------------------------------------------------

// void PLMPanel::focusInEvent(QFocusEvent *event)
// {
////    if(event->gotFocus())
//    this->setMenuActionList(actionList);

//    PLMPanelWindow::focusInEvent(event);
// }

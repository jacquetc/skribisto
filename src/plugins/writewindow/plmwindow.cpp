#include "plmwindow.h"
#include "plmdata.h"
#include "plmbasedock.h"

#include <QSettings>
#include <QStatusBar>
#include <QTimer>
#include <QToolButton>

 #include "plmwritingwindowmanager.h"

PLMWindow::PLMWindow(QWidget *parent, const QString& name) :
    PLMBaseWindow(parent, name)
{
    this->setupStatusBar();

    this->setRightDockDefaultCount(0);
    this->setBottomDockDefaultCount(0);
    this->setLeftDockDefaultCount(2);

    this->setMenuActions();

    QWidget *widget    = new QWidget(this);
    QBoxLayout *layout = new QBoxLayout(QBoxLayout::Direction::TopToBottom, widget);
    this->setCentralWidget(widget);
    widget->setLayout(layout);

    PLMWritingWindowManager *writeWindowManager =
        new PLMWritingWindowManager(this, layout);
    writeWindowManager->setObjectName("writeWindowManager");
}

// -------------------------------------------------------------------

PLMWindow::~PLMWindow()
{}

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
    QStatusBar *statusBar = this->statusBar();

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
            &PLMWindow::setLeftSidebarVisible, Qt::UniqueConnection);


    // save settings :
    connect(showLeftDockAct, &QAction::destroyed, [ = ]() {
        QSettings settings;
        settings.setValue("Docks/" + this->name() + "_leftSidebarVisible",
                          showLeftDockAct->isChecked());
    });

    // load settings :

    QSettings settings;
    bool value =
        settings.value("Docks/" + this->name() + "_leftSidebarVisible", true).toBool();
    showLeftDockAct->setChecked(value);


    QToolButton *showLeftDockButton = new QToolButton();
    showLeftDockButton->setAutoRaise(true);
    showLeftDockButton->setDefaultAction(showLeftDockAct);
    showLeftDockButton->hide();

    connect(this, &PLMBaseWindow::leftDockAdded, showLeftDockButton, &QToolButton::show);
    connect(this, &PLMBaseWindow::noLeftDock,    showLeftDockButton, &QToolButton::hide);

    statusBar->addWidget(showLeftDockButton);

    // addLeftDock :

    QAction *addLeftDockAct =
        new QAction(QIcon(":/pics/plume-creator.svg"), tr("Add Left Dock"), this);


    QToolButton *addLeftDockButton = new QToolButton();
    addLeftDockButton->setAutoRaise(true);
    addLeftDockButton->setDefaultAction(addLeftDockAct);
    addLeftDockButton->show();
    statusBar->addWidget(addLeftDockButton);

    connect(addLeftDockAct,
            &QAction::triggered,
            this,
            &PLMBaseWindow::addLeftDock);

    connect(this,
            &PLMBaseWindow::leftDockAdded,
            addLeftDockButton,
            &QToolButton::hide);
    connect(this,
            &PLMBaseWindow::noLeftDock,
            addLeftDockButton,
            &QToolButton::show);

    // stretcher :
    statusBar->addWidget(new QWidget(), 1);

    QAction *showRightDockAct =
        new QAction(QIcon(":/pics/plume-creator.svg"), tr("Show Right Sidebar"), this);

    showRightDockAct->setCheckable(true);
    connect(showRightDockAct, &QAction::toggled, [showRightDockAct](bool value) {
        if (value) {
            showRightDockAct->setText(tr("Hide Right Sidebar"));
            showRightDockAct->setToolTip(tr("Hide Right Sidebar"));
        }
        else { showRightDockAct->setText(tr("Show Right Sidebar"));
               showRightDockAct->setToolTip(tr("Show Right Sidebar")); }
    });

    connect(showRightDockAct, &QAction::toggled, this,
            &PLMWindow::setRightSidebarVisible);

    // save settings :
    connect(showRightDockAct, &QAction::destroyed, [ = ]() {
        QSettings settings;
        settings.setValue("Docks/" + this->name() + "_rightSidebarVisible",
                          showRightDockAct->isChecked());
    });

    // load settings :

    bool rightValue =
        settings.value("Docks/" + this->name() + "_rightSidebarVisible", true).toBool();
    showRightDockAct->setChecked(rightValue);


    QToolButton *showRightDockButton = new QToolButton();
    showRightDockButton->setAutoRaise(true);
    showRightDockButton->setDefaultAction(showRightDockAct);
    showRightDockButton->hide();

    connect(this, &PLMBaseWindow::rightDockAdded, showRightDockButton,
            &QToolButton::show);
    connect(this, &PLMBaseWindow::noRightDock,    showRightDockButton,
            &QToolButton::hide);

    statusBar->addWidget(showRightDockButton);

    // addRightDock :

    QAction *addRightDockAct =
        new QAction(QIcon(":/pics/plume-creator.svg"), tr("Add Right Dock"), this);


    QToolButton *addRightDockButton = new QToolButton();
    addRightDockButton->setAutoRaise(true);
    addRightDockButton->setDefaultAction(addRightDockAct);
    addRightDockButton->show();
    statusBar->addWidget(addRightDockButton);

    connect(addRightDockAct,
            &QAction::triggered,
            this,
            &PLMBaseWindow::addRightDock);

    connect(this,
            &PLMBaseWindow::rightDockAdded,
            addRightDockButton,
            &QToolButton::hide);
    connect(this,
            &PLMBaseWindow::noRightDock,
            addRightDockButton,
            &QToolButton::show);
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

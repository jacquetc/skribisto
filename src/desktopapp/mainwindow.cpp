#include "windowmanager.h"
#include "mainwindow.h"
#include "./ui_mainwindow.h"

#include <QCloseEvent>
#include <QSettings>
#include <QTimer>

MainWindow::MainWindow(int newWindowId)
    : QMainWindow()
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    this->setWindowId(newWindowId);
    this->setAttribute(Qt::WA_DeleteOnClose);



    this->setCorner(Qt::Corner::BottomLeftCorner, Qt::LeftDockWidgetArea);


    m_viewManager = new ViewManager(this, ui->centralwidget);

    QObject::connect(m_viewManager, &ViewManager::currentViewChanged, this, [this](View *view){
        ui->viewDock->setToolboxes(view->toolboxes());
    });




    // tool bar
    m_topToolBar = new TopToolBar();
    ui->toolBar->setContentsMargins(0, 0, 0, 0);
    ui->toolBar->addWidget(m_topToolBar);

    //status bar
    m_statusBar = new StatusBar();
    ui->statusbar->setContentsMargins(0, 0, 0, 0);
    ui->statusbar->addPermanentWidget(m_statusBar, 1);

QTimer::singleShot(0, this, &MainWindow::init);
}

//---------------------------------------

void MainWindow::init(){
    // tool bar
    ui->actionShow_Project_Dock->setChecked(ui->projectDock->isVisible());
    connect(ui->projectDock, &QDockWidget::visibilityChanged, ui->actionShow_Project_Dock, &QAction::setChecked);

    //status bar
    ui->actionShow_View_Dock->setChecked(ui->viewDock->isVisible());
    connect(ui->viewDock, &QDockWidget::visibilityChanged, ui->actionShow_View_Dock, &QAction::setChecked);

    m_projectDockBackend = new ProjectDockBackend(this, ui->projectDock);
}
//---------------------------------------

MainWindow::~MainWindow()
{
    delete ui;
}


//---------------------------------------


void MainWindow::on_actionAdd_Window_triggered()
{
windowManager->addEmptyWindow();
}

//---------------------------------------


void MainWindow::on_actionQuit_triggered()
{
    // accepted
    windowManager->closeAllWindows();
}

//---------------------------------------

ViewManager *MainWindow::viewManager() const
{
    return m_viewManager;
}

//---------------------------------------

int MainWindow::windowId() const
{
    return this->property("windowId").toInt();
}

//---------------------------------------

void MainWindow::setWindowId(int newWindowId)
{
    this->setProperty("windowId", newWindowId);
}

//---------------------------------------

void MainWindow::closeEvent(QCloseEvent *event)
{
    windowManager->closeWindow(this);
    event->accept();

}

void MainWindow::on_actionShow_View_Dock_triggered(bool checked)
{
    ui->viewDock->setVisible(checked);
}

void MainWindow::on_actionShow_Project_Dock_triggered(bool checked)
{
    ui->projectDock->setVisible(checked);
}


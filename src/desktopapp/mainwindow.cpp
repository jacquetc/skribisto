#include "projecttreecommands.h"
#include "thememanager.h"
#include "windowmanager.h"
#include "mainwindow.h"
#include "./ui_mainwindow.h"
#include "view.h"
#include "settingsdialog.h"
#include "newprojectwizard.h"

#include <QCloseEvent>
#include <QFileDialog>
#include <QSettings>
#include <QTimer>
#include "projectcommands.h"
#include <skrdata.h>
#include <QMessageBox>

MainWindow::MainWindow(int newWindowId)
    : QMainWindow()
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    this->setWindowId(newWindowId);
    this->setAttribute(Qt::WA_DeleteOnClose);

    this->setCorner(Qt::Corner::BottomLeftCorner, Qt::LeftDockWidgetArea);

    this->setUnifiedTitleAndToolBarOnMac(true);

    m_viewManager = new ViewManager(this, ui->centralwidget);

    QObject::connect(m_viewManager, &ViewManager::currentViewChanged, this, [this](View *view){
        ui->viewDock->setToolboxes(view->toolboxes());
    });

    QObject::connect(m_viewManager, &ViewManager::aboutToRemoveView, this, [this](View *view){
        ui->viewDock->setToolboxes(QList<Toolbox *>());
    });


    // tool bar

    m_topToolBar = new TopToolBar();
    ui->toolBar->setContentsMargins(0, 0, 0, 0);
    ui->toolBar->addWidget(m_topToolBar);

    //status bar
    m_statusBar = new StatusBar();
    ui->statusbar->setContentsMargins(0, 0, 0, 0);
    ui->statusbar->addPermanentWidget(m_statusBar, 1);


    // menu
    ui->actionClose_project->setEnabled(false);
    ui->actionSave->setEnabled(false);
    ui->actionSaveAs->setEnabled(false);
    ui->actionBack_up->setEnabled(false);
    ui->actionPrint->setEnabled(false);
    ui->actionExport->setEnabled(false);

    QObject::connect(skrdata->projectHub(), &PLMProjectHub::projectCountChanged, this, [this](int count){

        bool enable = count > 0;
        ui->actionClose_project->setEnabled(enable);
        ui->actionSave->setEnabled(enable);
        ui->actionSaveAs->setEnabled(enable);
        ui->actionBack_up->setEnabled(enable);
        ui->actionPrint->setEnabled(enable);
        ui->actionExport->setEnabled(enable);
    });

    ui->menuEdit->insertAction(ui->actionUndo, projectTreeCommands->undoStack()->createUndoAction(this));
    ui->menuEdit->insertAction(ui->actionRedo, projectTreeCommands->undoStack()->createRedoAction(this));
    ui->actionUndo->deleteLater();
    ui->actionRedo->deleteLater();


    changeSwitchThemeActionText(themeManager->currentThemeType());

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

    themeManager->scanChildrenAndAddWidgetsHoldingIcons(this);
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

void MainWindow::addWindowForItemId(int projectId, int treeItemId)
{
    windowManager->addWindowForItemId(projectId, treeItemId);
}

void MainWindow::addWindowForProjectIndependantPageType(const QString &pageType)
{
    windowManager->addWindowForProjectIndependantPageType(pageType);

}

void MainWindow::addWindowForProjectDependantPageType(int projectId, const QString &pageType)
{
    windowManager->addWindowForProjectDependantPageType(projectId, pageType);

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
    QList<int> projectList = skrdata->projectHub()->getProjectIdList();
    QList<int> notSavedProjectList;

    for(int projectId : projectList){
        if(!skrdata->projectHub()->isProjectSaved(projectId)){
            notSavedProjectList.append(projectId);
        }
    }


    if(windowManager->getNumberOfWindows() > 1){
        windowManager->closeWindow(this);
    }
    else if(!notSavedProjectList.isEmpty()){

        SKRResult result;

        for(int projectId : notSavedProjectList){
                   QMessageBox::StandardButton toggledButton = QMessageBox::question(this, tr("Save"),
                                                                                     tr("Do you want save changes to %1?").arg(skrdata->projectHub()->getProjectName(projectId)),
                                                                                     QMessageBox::Cancel | QMessageBox::Discard | QMessageBox::Save);

                   switch (toggledButton) {
                   case QMessageBox::Cancel:

                       event->ignore();
                       return;

                       break;
                   case QMessageBox::Save:
                       result.clear();
                       result = projectCommands->save(projectId);
                       if(!result.isSuccess() && result.containsErrorCode("no_path")){
                            this->openSaveAsDialog(projectId);
                       }

                       break;
                   case QMessageBox::Discard:
                       break;
                   default:
                       break;
                   }

        }

        return;

    }
    else{
        QMessageBox::StandardButton toggledButton = QMessageBox::question(this, tr("Quit"),tr("Do you really want to quit?"));
        switch (toggledButton) {
        case QMessageBox::No:
            event->ignore();
            return;
            break;
        case QMessageBox::Yes:
            event->accept();
            return;
            break;
        default:
            break;
        }
    }



    event->ignore();

}

void MainWindow::on_actionShow_View_Dock_triggered(bool checked)
{
    ui->viewDock->setVisible(checked);
}

void MainWindow::on_actionShow_Project_Dock_triggered(bool checked)
{
    ui->projectDock->setVisible(checked);
}


void MainWindow::on_actionPreferences_triggered()
{
    SettingsDialog settings(this);
    settings.exec();
}


void MainWindow::on_actionSaveAs_triggered()
{
    int activeProject = skrdata->projectHub()->getActiveProject();

    this->openSaveAsDialog(activeProject);
}

void MainWindow::openSaveAsDialog(int projectId){

    QString filter = projectCommands->getSaveFilter();


    QString selectedFilter;
    QUrl saveFileNameUrl = QFileDialog::getSaveFileUrl(this, tr("Save project"),
                                                       skrdata->projectHub()->getPath(projectId),
                                                       filter, &selectedFilter, QFileDialog::Options()
                                                       );

    projectCommands->saveAs(projectId, saveFileNameUrl, QFileInfo(saveFileNameUrl.toLocalFile()).suffix());

}


void MainWindow::on_actionLoad_Project_triggered()
{
    QUrl openFileNameUrl = QFileDialog::getOpenFileUrl(this, "Open a project", QUrl(), tr("Skribisto project (*.skrib)"));

    projectCommands->loadProject(openFileNameUrl);

}


void MainWindow::on_actionSave_triggered()
{
    int activeProject = skrdata->projectHub()->getActiveProject();
    projectCommands->save(activeProject);

}


void MainWindow::on_actionClose_project_triggered()
{
    int activeProject = skrdata->projectHub()->getActiveProject();
    projectCommands->closeProject(activeProject);

}


void MainWindow::on_actionNew_project_triggered()
{
    NewProjectWizard newWizard(this);
    newWizard.exec();
}


void MainWindow::on_actionSwitch_theme_triggered()
{
    changeSwitchThemeActionText(themeManager->switchThemeType());
}

void MainWindow::changeSwitchThemeActionText(ThemeManager::ThemeType type){
    if(type == ThemeManager::Light){
        ui->actionSwitch_theme->setText(tr("Switch to dark theme"));
    }
    else {
        ui->actionSwitch_theme->setText(tr("Switch to light theme"));

    }
}


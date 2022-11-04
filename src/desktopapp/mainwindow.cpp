#include "export/exportdialog.h"
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

    // view manager

    m_viewManager = new ViewManager(this, ui->centralwidget);

    QObject::connect(m_viewManager, &ViewManager::currentViewChanged, this, [this](View *view){
        ui->viewDock->setToolboxes(view->toolboxes());
    });

    QObject::connect(m_viewManager, &ViewManager::aboutToRemoveView, this, [this](View *view){
        ui->viewDock->setToolboxes(QList<Toolbox *>());
    });

    this->setupMenuActions();



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

    QAction *undoAction = projectTreeCommands->undoStack()->createUndoAction(this);
    undoAction->setShortcut(QKeySequence::Undo);
    undoAction->setShortcutContext(Qt::WindowShortcut);

    QAction *redoAction = projectTreeCommands->undoStack()->createRedoAction(this);
    undoAction->setShortcut(QKeySequence::Redo);
    undoAction->setShortcutContext(Qt::WindowShortcut);

    ui->menuEdit->insertAction(ui->actionUndo, undoAction);
    ui->menuEdit->insertAction(ui->actionRedo, redoAction);
    ui->actionUndo->deleteLater();
    ui->actionRedo->deleteLater();


    // recent projects:

    QMenu *menu = new QMenu(this);
    ui->actionRecent_projects->setMenu(menu);

    this->populateRecentProjectsMenu();
    connect(skrdata->projectHub(), &PLMProjectHub::projectClosed, this, &MainWindow::populateRecentProjectsMenu);

    // insert in recent projects
    connect(skrdata->projectHub(), &PLMProjectHub::projectLoaded, this, [](int projectId){


        QString projectName = skrdata->projectHub()->getProjectName(projectId);
        QUrl    projectPath = skrdata->projectHub()->getPath(projectId);

        bool alreadyHere      = false;
        int  alreadyHereIndex = -1;

        QSettings settings;

        settings.beginGroup("welcome");
        int size = settings.beginReadArray("recentProjects");


        for (int i = 0; i < size; ++i) {
            settings.setArrayIndex(i);

            QUrl settingFileName = settings.value("fileNameUrl").toUrl();

            if (settingFileName == projectPath) {
                alreadyHere      = true;
                alreadyHereIndex = i;
            }
        }
        settings.endArray();


        if (alreadyHere) {
            // write again the title in case it was changed
            settings.beginWriteArray("recentProjects", size);
            settings.setArrayIndex(alreadyHereIndex);
            settings.setValue("title", projectName);
        }
        else {
            // add a new recent project
            settings.beginWriteArray("recentProjects", size + 1);
            settings.setArrayIndex(size);
            settings.setValue("title",       projectName);
            settings.setValue("fileNameUrl", projectPath);
        }


        settings.endArray();

        settings.endGroup();

        settings.sync();

    });





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
                                                                              tr("Do you want to save changes to %1?").arg(skrdata->projectHub()->getProjectName(projectId)),
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


void MainWindow::openSaveAsDialog(int projectId){

    QString filter = projectCommands->getSaveFilter();


    QString selectedFilter;
    QUrl saveFileNameUrl = QFileDialog::getSaveFileUrl(this, tr("Save project"),
                                                       skrdata->projectHub()->getPath(projectId),
                                                       filter, &selectedFilter, QFileDialog::Options()
                                                       );

    projectCommands->saveAs(projectId, saveFileNameUrl, QFileInfo(saveFileNameUrl.toLocalFile()).suffix());

}

//------------------------------------------------------------------------------

void MainWindow::setupMenuActions()
{
    ui->actionNew_project->setShortcuts(QKeySequence::New);
    connect(ui->actionNew_project, &QAction::triggered, this, [this](){
        NewProjectWizard newWizard(this);
        newWizard.exec();
    });

    //-------------------

    ui->actionClose_project->setShortcuts(QKeySequence::Close);
    connect(ui->actionClose_project, &QAction::triggered, this, [this](){
        int activeProject = skrdata->projectHub()->getActiveProject();


        if(!skrdata->projectHub()->isProjectSaved(activeProject)){

            SKRResult result;

            QMessageBox::StandardButton toggledButton = QMessageBox::question(this, tr("Save"),
                                                                              tr("Do you want to save changes to %1?").arg(skrdata->projectHub()->getProjectName(activeProject)),
                                                                              QMessageBox::Cancel | QMessageBox::Discard | QMessageBox::Save);

            switch (toggledButton) {
            case QMessageBox::Cancel:
                return;

                break;
            case QMessageBox::Save:
                result.clear();
                result = projectCommands->save(activeProject);
                if(!result.isSuccess() && result.containsErrorCode("no_path")){
                    this->openSaveAsDialog(activeProject);
                }

                break;
            case QMessageBox::Discard:
                break;
            default:
                break;
            }


            projectCommands->closeProject(activeProject);
            return;

        }
        else{
            QMessageBox::StandardButton toggledButton = QMessageBox::question(this,
                                                                              tr("Quit"),
                                                                              tr("Do you really want to close %1?").arg(skrdata->projectHub()->getProjectName(activeProject)));
            switch (toggledButton) {
            case QMessageBox::No:

                return;
                break;
            case QMessageBox::Yes:
                projectCommands->closeProject(activeProject);
                return;
                break;
            default:
                break;
            }
        }


    });
    //-------------------

    ui->actionLoad_Project->setShortcuts(QKeySequence::Open);
    connect(ui->actionLoad_Project, &QAction::triggered, this, [this](){
        QUrl openFileNameUrl = QFileDialog::getOpenFileUrl(this, "Open a project", QUrl(), tr("Skribisto project (*.skrib)"));

        projectCommands->loadProject(openFileNameUrl);
    });
    //-------------------

    ui->actionSave->setShortcuts(QKeySequence::Save);
    connect(ui->actionSave, &QAction::triggered, this, [](){
        int activeProject = skrdata->projectHub()->getActiveProject();
        projectCommands->save(activeProject);
    });
    //-------------------

    ui->actionSaveAs->setShortcuts(QKeySequence::SaveAs);
    connect(ui->actionSaveAs, &QAction::triggered, this, [this](){
        int activeProject = skrdata->projectHub()->getActiveProject();

        this->openSaveAsDialog(activeProject);
    });
    //-------------------

    ui->actionPreferences->setShortcuts(QKeySequence::Preferences);
    connect(ui->actionPreferences, &QAction::triggered, this, [this](){
        SettingsDialog settings(this);
        settings.exec();
    });
    //-------------------

    ui->actionPrint->setShortcuts(QKeySequence::Print);
    connect(ui->actionPrint, &QAction::triggered, this, [this](){
        ExportDialog dialog(this, true);
        dialog.exec();

    });

    //-------------------

    ui->actionExport->setShortcut(QKeySequence(tr("Ctrl+E", "export shortcut")));
    connect(ui->actionExport, &QAction::triggered, this, [this](){
        ExportDialog dialog(this, false);
        dialog.exec();

    });
    //-------------------

    ui->actionQuit->setShortcuts(QKeySequence::Quit);
    connect(ui->actionQuit, &QAction::triggered, this, [](){
        // accepted
        windowManager->closeAllWindows();
    });
    //-------------------

    connect(ui->actionAdd_Window, &QAction::triggered, this, [](){
        windowManager->addEmptyWindow();

    });
    //-------------------

    connect(ui->actionSwitch_theme, &QAction::triggered, this, [this](){
        changeSwitchThemeActionText(themeManager->switchThemeType());

    });

    //-------------------

    connect(ui->actionShow_View_Dock, &QAction::triggered, this, [this](bool checked){
        ui->viewDock->setVisible(checked);

    });

    //-------------------

    connect(ui->actionShow_Project_Dock, &QAction::triggered, this, [this](bool checked){
        ui->projectDock->setVisible(checked);

    });

    //-------------------

}


//------------------------------------------------------------------------------


void MainWindow::populateRecentProjectsMenu()
{
    qDeleteAll(ui->actionRecent_projects->menu()->actions());

    QList<QAction *> allRecentProjects;

    QSettings settings;

    settings.beginGroup("welcome");
    int size = settings.beginReadArray("recentProjects");

    for (int i = 0; i < size; ++i) {
        settings.setArrayIndex(i);

        QUrl fileName = settings.value("fileNameUrl").toUrl();


        // exists ?
        QFileInfo info(fileName.toLocalFile());
        if(!info.exists() || !info.isWritable()){
            continue;
        }


        QAction *action = new QAction(settings.value("title").toString(), this);
        action->setProperty("fileName", fileName);

        connect(action, &QAction::triggered, this, [this](){
            QAction *action = static_cast<QAction *>(this->sender());
            projectCommands->loadProject(action->property("fileName").toUrl());
        });


        // is project opened ?
        allRecentProjects.append(action);

        for (int projectId : skrdata->projectHub()->getProjectIdList()) {
            QString projectName = skrdata->projectHub()->getProjectName(projectId);
            QUrl    projectPath = skrdata->projectHub()->getPath(projectId);

            if ((projectName == action->text()) &&
                    (projectPath == action->property("fileName").toUrl())) {


                allRecentProjects.removeLast();
                allRecentProjects.prepend(action);
            }
        }
    }


    settings.endArray();
    settings.endGroup();

    for(QAction *action : allRecentProjects){
        ui->actionRecent_projects->menu()->addAction(action);
    }


}

void MainWindow::changeSwitchThemeActionText(ThemeManager::ThemeType type){
    if(type == ThemeManager::Light){
        ui->actionSwitch_theme->setText(tr("Switch to dark theme"));
    }
    else {
        ui->actionSwitch_theme->setText(tr("Switch to light theme"));

    }
}


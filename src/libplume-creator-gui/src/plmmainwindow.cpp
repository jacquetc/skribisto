#include "plmmainwindow.h"

#include "plmmessagehandler.h"
#include "ui_plmmainwindow.h"
#include "plmguiinterface.h"
#include "plmpluginloader.h"
#include "plmguiplugins.h"
#include "plmmodels.h"

#include <QFileDialog>
#include <QMessageBox>
#include <QSettings>
#include <QByteArray>
#include <QTimer>
#include <QApplication>
#include <QDesktopWidget>

// #include <QQuickWidget>

PLMMainWindow::PLMMainWindow(PLMData *data) :
    QMainWindow(nullptr), ui(new Ui::PLMMainWindow), m_data(data)
{
    new PLMModels(this);

    this->applyStyleSheet();


    PLMGuiPlugins::addGuiPlugins();
    ui->setupUi(this);
    this->show();
    connect(ui->leftSideMainBar,
            &PLMSideMainBar::windowRaiseCalled,
            this,
            &PLMMainWindow::raiseWindow);
    connect(ui->leftSideMainBar,
            &PLMSideMainBar::windowAttachmentCalled,
            this,
            &PLMMainWindow::attachWindow);
    connect(ui->leftSideMainBar,
            &PLMSideMainBar::windowDetachmentCalled,
            this,
            &PLMMainWindow::detachWindow);

    //    connect(PLMMessageHandler::instance(),
    // &PLMMessageHandler::messageSent, this,
    //            &PLMMainWindow::displayMessage);
    //    QQuickWidget *mQQuickWidget = new
    // QQuickWidget(QUrl(QStringLiteral("qrc:///qml/sidePanelBar.qml")), this);
    //    mQQuickWidget->setResizeMode(QQuickWidget::SizeRootObjectToView);
    //    ui->centralHorLayout->insertWidget(0,mQQuickWidget);
    //    connect(PLMMessageHandler::instance(),
    // &PLMMessageHandler::messageSent, this,
    //            &PLMMainWindow::displayMessage);
    connect(plmdata->projectHub(), SIGNAL(allProjectsClosed()), this,
            SLOT(clearFromAllProjects()));
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMMainWindow::activate);

    this->loadPlugins();
    this->applySettings();
    this->applyRaiseWindowSetting();
    QTimer::singleShot(0, this, SLOT(init()));
}

void PLMMainWindow::init()
{
    // load plugins
    // TEMP
}

PLMMainWindow::~PLMMainWindow()
{
    delete ui;
}

void PLMMainWindow::applyStyleSheet()
{
    QFile file(":/stylesheets/light.css");

    Q_ASSERT(file.exists());

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) return;

    QString content = file.readAll();
    file.close();
    this->setStyleSheet(content);
}

void PLMMainWindow::clearFromAllProjects()
{}

void PLMMainWindow::activate()
{}

// ------------------------------------------
void PLMMainWindow::loadPlugins()
{
    // plugins are already loaded in plmpluginloader
    QList<PLMWindowInterface *> pluginList =
        PLMPluginLoader::instance()->pluginsByType<PLMWindowInterface>();

    // setup :
    foreach(PLMWindowInterface * plugin, pluginList) {
        if (!plugin->pluginEnabled()) continue;

        plugin->init();
        PLMBaseWindow *window = plugin->window();
        connect(window,
                &PLMBaseWindow::attachmentCalled,
                ui->leftSideMainBar,
                &PLMSideMainBar::attachWindowByName);
        ui->stackedWidget->addWidget(window);
        QString windowName = window->name();
        hash_nameAndWindow.insert(windowName, window);
    }
}

// ------------------------------------------

void PLMMainWindow::applyRaiseWindowSetting()
{
    QSettings settings;

    settings.beginGroup("Windows");

    for (PLMBaseWindow *window : hash_nameAndWindow.values()) {
        if (settings.value(window->name() + "-raised",
                           false).toBool() == true) this->raiseWindow(window->name());
    }
    settings.endGroup();
}

// ------------------------------------------

void PLMMainWindow::raiseWindow(const QString& windowName)
{
    PLMBaseWindow *window = hash_nameAndWindow.value(windowName);

    if (window->detached()) {
        window->activateWindow();
        window->raise();
    }
    else {
        ui->stackedWidget->setCurrentWidget(window);
    }

    QSettings settings;
    settings.beginGroup("Windows");
    settings.setValue(windowName + "-raised", true);

    for (PLMBaseWindow *window : hash_nameAndWindow.values()) {
        if (windowName != window->name()) settings.setValue(window->name() + "-raised",
                                                            false);
    }
    settings.endGroup();
}

void PLMMainWindow::attachWindow(const QString& windowName)
{
    PLMBaseWindow *window = hash_nameAndWindow.value(windowName);

    window->setDetached(false);
    ui->stackedWidget->addWidget(window);
    ui->stackedWidget->setCurrentWidget(window);
    ui->leftSideMainBar->setButtonChecked(windowName);

    QSettings settings;
    settings.beginGroup("Windows");
    settings.setValue(windowName + "-detached", false);
    settings.endGroup();
}

void PLMMainWindow::detachWindow(const QString& windowName)
{
    PLMBaseWindow *window = hash_nameAndWindow.value(windowName);

    ui->stackedWidget->removeWidget(window);
    window->setParent(nullptr);
    window->setDetached(true);
    window->show();

    // window->applySettingsState();

    //

    QString key =
        hash_nameAndWindow.key(dynamic_cast<PLMBaseWindow *>(ui->stackedWidget->
                                                             currentWidget()));
    ui->leftSideMainBar->setButtonChecked(key);

    QSettings settings;
    settings.beginGroup("Windows");
    settings.setValue(windowName + "-detached", true);
    settings.endGroup();
}

/*
   void PLMMainWindow::addPluginPanels()
   {

    // load plugins :

    QList<PanelInterface *> panels =
       PluginLoader::instance()->pluginsByType<PanelInterface>();


    foreach (PanelInterface *panel, panels){
        panelListHash.insert(panel->panelName(), panel->panel());



    //addToStack
    ui->stackedWidget->addWidget(panel->panel());
    }

   }
 */

// ---------------------------------------------------------------------------

// ------------------------------------------

// ---------------------------------------------------------------------------
void PLMMainWindow::closeEvent(QCloseEvent *event)
{
    //    if(!core->isProjectStarted())
    //    {
    //        //writeSettings();
    //        event->accept();
    //        return;
    //    }
    if (plmdata->projectHub()->isThereAnyOpenedProject() == false) {
        this->writeSettings();
        foreach(PLMBaseWindow * window, hash_nameAndWindow.values()) {
            window->setForceTrueClosing(true);
            window->close();
        }
        event->accept();
    }

    QMessageBox msgBox(this);

    msgBox.setText(tr("Do you want to quit ?"));
    msgBox.setInformativeText(tr(""));
    msgBox.setStandardButtons(QMessageBox::Ok | QMessageBox::Cancel);
    msgBox.setDefaultButton(QMessageBox::Cancel);
    int ret = msgBox.exec();

    switch (ret) {
    case QMessageBox::Ok:

        this->writeSettings();
        foreach(QPointer<PLMBaseWindow>window, hash_nameAndWindow.values()) {
            window->setForceTrueClosing(true);
            window->close();
        }

        event->accept();

        //        hub->closeCurrentProject();
        break;

    case QMessageBox::Cancel:
        event->ignore();
        break;

    default:

        // should never be reached
        break;
    }
}

// ---------------------------------------------------------------------------

/*


   QString PLMMainWindow::currentPanel()
   {

   return ui->stackedWidget->currentWidget()->objectName();
   }

   //---------------------------------------------------------------------------


   bool PLMMainWindow::setCurrentPanel(const QString &objectName)
   {
   for(int i = 0 ; i < ui->stackedWidget->count() ; ++i){
     if(ui->stackedWidget->widget(i)->objectName() == objectName){
         ui->stackedWidget->setCurrentIndex(i);
         ui->stackedWidget->currentWidget()->setFocus();
         return true;
     }
   }

   return false;
   }

 */

// ---------------------------------------------------------------------------

void PLMMainWindow::writeSettings()
{
    // hub->writeSettings();

    QSettings settings;

    settings.beginGroup("MainWindow");


    settings.setValue("geometry",    this->saveGeometry());
    settings.setValue("isMaximised", this->isMaximized());

    settings.setValue("firstStart",  false);
    settings.endGroup();

    //    qDebug() << "main settings saved";
}

// ---------------------------------------------------------------------------

void PLMMainWindow::applySettings()
{
    QSettings settings;

    settings.beginGroup("MainWindow");


    if (settings.value("isMaximised", false).toBool())
    {
        // qDebug() << QApplication::desktop()->availableGeometry(this);
        this->setGeometry(QApplication::desktop()->availableGeometry(this));
        this->showMaximized();
    } else {
        this->restoreGeometry(settings.value("geometry", "0").toByteArray());
    }


    //    m_firstStart = settings.value("firstStart", true).toBool();

    settings.endGroup();
}

void PLMMainWindow::on_actionSave_project_triggered()
{
    PLMError error = plmdata->projectHub()->saveProjectAs(1,
                                                          "SQLITE",
                                                          "/home/cyril/Devel/plume/test1.plume");

    Q_ASSERT(error.isSuccess());
}

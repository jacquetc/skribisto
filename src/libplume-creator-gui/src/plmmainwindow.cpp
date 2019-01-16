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

// #include <QQuickWidget>

PLMMainWindow::PLMMainWindow(PLMData *data) :
    QMainWindow(nullptr), ui(new Ui::PLMMainWindow), m_data(data)
{
    new PLMModels(this);

    PLMGuiPlugins::addGuiPlugins();
    ui->setupUi(this);
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


    QTimer::singleShot(0, this, SLOT(init()));
}

void PLMMainWindow::init()
{
    this->applySettings();

    // load plugins
    // TEMP
}

PLMMainWindow::~PLMMainWindow()
{
    delete ui;
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
        plugin->init();
        PLMBaseWindow *window = plugin->window();
        connect(window,
                &PLMBaseWindow::attachmentCalled,
                ui->leftSideMainBar,
                &PLMSideMainBar::attachWindowByName);
        ui->stackedWidget->addWidget(window);
        QString windowName = window->property("name").toString();
        hash_nameAndWindow.insert(windowName, window);
    }
}

// ------------------------------------------

void PLMMainWindow::raiseWindow(const QString& windowName)
{
    QMainWindow *window = hash_nameAndWindow.value(windowName);

    ui->stackedWidget->setCurrentWidget(window);
}

void PLMMainWindow::attachWindow(const QString& windowName)
{
    PLMBaseWindow *window = hash_nameAndWindow.value(windowName);

    if (window->detached()) window->saveSettingsGeometry();
    window->setDetached(false);
    ui->stackedWidget->addWidget(window);
    ui->stackedWidget->setCurrentWidget(window);
    ui->leftSideMainBar->setButtonChecked(windowName);
}

void PLMMainWindow::detachWindow(const QString& windowName)
{
    PLMBaseWindow *window = hash_nameAndWindow.value(windowName);

    ui->stackedWidget->removeWidget(window);
    window->setParent(0);
    window->setDetached(true);

    window->show();
    window->applySettingsGeometry();


    QString key =
        hash_nameAndWindow.key(dynamic_cast<PLMBaseWindow *>(ui->stackedWidget->
                                                             currentWidget()));
    ui->leftSideMainBar->setButtonChecked(key);
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
        qApp->closeAllWindows();
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


    settings.setValue("geometry",   this->saveGeometry());

    settings.setValue("firstStart", false);
    settings.endGroup();

    //    qDebug() << "main settings saved";
}

// ---------------------------------------------------------------------------

void PLMMainWindow::applySettings()
{
    QSettings settings;

    settings.beginGroup("MainWindow");

    this->restoreGeometry(settings.value("geometry", "0").toByteArray());

    //    m_firstStart = settings.value("firstStart", true).toBool();

    settings.endGroup();
}

#include "plmmainwindow.h"

#include "plmmessagehandler.h"
#include "ui_plmmainwindow.h"
#include "plmguiinterface.h"
#include "plmpluginloader.h"

#include <QFileDialog>
#include <QMessageBox>
#include <QSettings>
#include <QByteArray>
//#include <QQuickWidget>

PLMMainWindow::PLMMainWindow(PLMData *data) :
    QMainWindow(0), m_data(data),
    ui(new Ui::PLMMainWindow)
{
    ui->setupUi(this);
    this->loadPlugins();
    connect(ui->sideMainBar, &PLMSideMainBar::windowRaiseCalled, this, &PLMMainWindow::raiseWindow);
    connect(ui->sideMainBar, &PLMSideMainBar::windowAttachmentCalled, this, &PLMMainWindow::attachWindow);
    connect(ui->sideMainBar, &PLMSideMainBar::windowDetachmentCalled, this, &PLMMainWindow::detachWindow);

//    connect(PLMMessageHandler::instance(), &PLMMessageHandler::messageSent, this,
//            &PLMMainWindow::displayMessage);
    //    QQuickWidget *mQQuickWidget = new QQuickWidget(QUrl(QStringLiteral("qrc:///qml/sidePanelBar.qml")), this);
    //    mQQuickWidget->setResizeMode(QQuickWidget::SizeRootObjectToView);
    //    ui->centralHorLayout->insertWidget(0,mQQuickWidget);
//    connect(PLMMessageHandler::instance(), &PLMMessageHandler::messageSent, this,
//            &PLMMainWindow::displayMessage);
    connect(plmdata->projectHub(), SIGNAL(allProjectsClosed()), this, SLOT(clearFromAllProjects()));
    connect(plmdata->projectHub(), &PLMProjectHub::projectLoaded, this, &PLMMainWindow::activate);
    // restore saved geometry
    QSettings settings;
    this->restoreGeometry(settings.value("geometry", "0").toByteArray());
}

PLMMainWindow::~PLMMainWindow()
{
    delete ui;
}

void PLMMainWindow::clearFromAllProjects()
{
}

void PLMMainWindow::activate()
{
}

void PLMMainWindow::init()
{
    //this->readSettings();
    //load plugins
    //TEMP
}

//------------------------------------------
void PLMMainWindow::loadPlugins()
{
    // plugins are already loaded in plmpluginloader
    QList<PLMWindowInterface *> pluginList = PLMPluginLoader::instance()->pluginsByType<PLMWindowInterface>();

    foreach (PLMWindowInterface *plugin, pluginList) {
        PLMBaseWindow *window = plugin->window();
        connect(window, &PLMBaseWindow::attachmentCalled, ui->sideMainBar, &PLMSideMainBar::attachWindowByName);
        ui->stackedWidget->addWidget(window);
        QString windowName = window->property("name").toString();
        hash_nameAndWindow.insert(windowName, window);

    }
}
//------------------------------------------

void PLMMainWindow::raiseWindow(const QString &windowName)
{
    QMainWindow *window = hash_nameAndWindow.value(windowName);
    ui->stackedWidget->setCurrentWidget(window);

}

void PLMMainWindow::attachWindow(const QString &windowName)
{
    QMainWindow *window = hash_nameAndWindow.value(windowName);
    ui->stackedWidget->addWidget(window);
    ui->stackedWidget->setCurrentWidget(window);
    ui->sideMainBar->setButtonChecked(windowName);

}

void PLMMainWindow::detachWindow(const QString &windowName)
{
    QMainWindow *window = hash_nameAndWindow.value(windowName);
    ui->stackedWidget->removeWidget(window);
    window->setParent(0);
    window->show();

    QString key = hash_nameAndWindow.key(dynamic_cast<QMainWindow *>(ui->stackedWidget->currentWidget()));
    ui->sideMainBar->setButtonChecked(key);


}
/*
   void PLMMainWindow::addPluginPanels()
   {

    // load plugins :

    QList<PanelInterface *> panels = PluginLoader::instance()->pluginsByType<PanelInterface>();


    foreach (PanelInterface *panel, panels){
        panelListHash.insert(panel->panelName(), panel->panel());



    //addToStack
    ui->stackedWidget->addWidget(panel->panel());
    }

   }
 */
//---------------------------------------------------------------------------

//------------------------------------------

//---------------------------------------------------------------------------
void PLMMainWindow::closeEvent(QCloseEvent *event)
{
    //TODO: temp :
    //writeSettings();
    qApp->closeAllWindows();
    event->accept();
    return;
    //    if(!core->isProjectStarted())
//    {
//        //writeSettings();
//        event->accept();
//        return;
//    }
    QMessageBox msgBox(this);
    msgBox.setText(tr("Do you want to quit ?"));
    msgBox.setInformativeText(tr("Your changes are already saved."));
    msgBox.setStandardButtons(QMessageBox::Ok | QMessageBox::Cancel);
    msgBox.setDefaultButton(QMessageBox::Cancel);
    int ret = msgBox.exec();

    switch (ret) {
    case QMessageBox::Ok:
        //writeSettings();
        //        hub->closeCurrentProject();
        qApp->closeAllWindows();
        event->accept();
        break;

    case QMessageBox::Cancel:
        event->ignore();
        break;

    default:
        // should never be reached
        break;
    }
}

//---------------------------------------------------------------------------
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


   //---------------------------------------------------------------------------

   void PLMMainWindow::writeSettings()
   {
   hub->writeSettings();

   QSettings settings;
   settings.beginGroup( "PLMMainWindow" );


   settings.setValue( "geometry", this->saveGeometry());

   settings.setValue( "firstStart", false);
   settings.endGroup();

   //    qDebug() << "main settings saved";
   }

   //---------------------------------------------------------------------------

   void PLMMainWindow::readSettings()
   {
   QSettings settings;
   settings.beginGroup( "PLMMainWindow" );

   this->restoreGeometry(settings.value( "geometry", "").toByteArray());

   //    m_firstStart = settings.value("firstStart", true).toBool();

   settings.endGroup();



   }
 */

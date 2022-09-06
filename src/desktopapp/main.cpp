#include <QGuiApplication>
#include <QApplication>
#include <QSettings>
#include <QStyleFactory>
#include <interfaces/projecttoolboxinterface.h>
#include "skrdata.h"
#include "windowmanager.h"
#include "projecttreemodel.h"

int main(int argc, char *argv[])
{
    QGuiApplication::setHighDpiScaleFactorRoundingPolicy(
        Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);



    QApplication app(argc, argv);
    app.setStyle(QStyleFactory::create("Fusion"));
    // Names for the QSettings
    QCoreApplication::setOrganizationName("skribisto");
    QCoreApplication::setOrganizationDomain("skribisto.eu");
    QCoreApplication::setApplicationVersion(QString::number(
                                                SKR_VERSION_MAJOR) + "." + QString::number(
                                                SKR_VERSION_MINOR));
    qDebug() << QCoreApplication::applicationVersion();
    QString appName = "Skribisto-desktop";
    QCoreApplication::setApplicationName(appName);
    QSettings::setDefaultFormat(QSettings::IniFormat);

    // command line parser:

    QCommandLineParser parser;
         parser.setApplicationDescription("Software for writers");
         parser.addHelpOption();
         parser.addVersionOption();
         parser.addPositionalArgument("source", QCoreApplication::tr("Project to open"));

         parser.addOptions({
                  // Open test project (-t, --testProject)
                  {{"t", "testProject"},
                      QCoreApplication::translate("main", "Show progress during copy")}
                           });
parser.process(app);


const QStringList args = parser.positionalArguments();


// load libraries :
    new SKRData;
    new ProjectTreeModel;

// load plugins:
skrpluginhub->addPluginType<ProjectToolboxInterface>();




    windowManager->restoreWindows();

    if(parser.isSet("testProject")){
        skrdata->projectHub()->loadProject(QUrl("qrc:/testfiles/skribisto_test_project.skrib"));
    }


    return app.exec();

}

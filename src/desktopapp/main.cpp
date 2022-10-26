#include "commands.h"
#include "interfaces/newprojectformatinterface.h"
#include "interfaces/pagetypeiconinterface.h"
#include "projectcommands.h"
#include "tagcommands.h"
#include "projecttreecommands.h"
#include "treemodels/projecttreemodel.h"
#include "skrdata.h"
#include "skribistostyle.h"
#include "thememanager.h"
#include "windowmanager.h"
#include <QApplication>
#include <QGuiApplication>
#include <QSettings>
#include <QStyleFactory>
#include <interfaces/itemexporterinterface.h>
#include <interfaces/pageinterface.h>
#include <interfaces/pagedesktopinterface.h>
#include <interfaces/projecttoolboxinterface.h>
#include "interfaces/projectpageinterface.h"
#include <text/textbridge.h>

int main(int argc, char *argv[]) {
  QGuiApplication::setHighDpiScaleFactorRoundingPolicy(
      Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);

  // QIcon::setFallbackSearchPaths(QIcon::fallbackSearchPaths() <<
  // ":/icons/backup/");
  QApplication app(argc, argv);
  QApplication::setStyle(new SkribistoStyle);


  // Names for the QSettings
  QCoreApplication::setOrganizationName("skribisto");
  QCoreApplication::setOrganizationDomain("skribisto.eu");
  QCoreApplication::setApplicationVersion(QString::number(SKR_VERSION_MAJOR) +
                                          "." +
                                          QString::number(SKR_VERSION_MINOR));
  qDebug() << QCoreApplication::applicationVersion();
  QString appName = "Skribisto-desktop";
  QCoreApplication::setApplicationName(appName);
  QSettings::setDefaultFormat(QSettings::IniFormat);

  // command line parser:

  QCommandLineParser parser;
  parser.setApplicationDescription("Software for writers");
  parser.addHelpOption();
  parser.addVersionOption();
  parser.addPositionalArgument("source",
                               QCoreApplication::tr("Project to open"));

  parser.addOptions(
      {// Open test project (-t, --testProject)
       {{"t", "testProject"},
        QCoreApplication::translate("main", "Show progress during copy")}});
  parser.process(app);

  const QStringList args = parser.positionalArguments();

  QUndoStack *undoStack = new QUndoStack;

  auto *skrData = new SKRData(qApp);


  // load plugins:
  skrpluginhub->addPluginType<ProjectToolboxInterface>();
  skrpluginhub->addPluginType<PageInterface>();
  skrpluginhub->addPluginType<PageDesktopInterface>();
  skrpluginhub->addPluginType<PageTypeIconInterface>();
  skrpluginhub->addPluginType<ItemExporterInterface>();
  skrpluginhub->addPluginType<NewProjectFormatInterface>();
  skrpluginhub->addPluginType<ProjectPageInterface>();

  // load singletons :
  new ProjectTreeModel(skrData);
  new Commands(skrData, undoStack);
  new ProjectCommands(skrData, undoStack);
  new ProjectTreeCommands(skrData, undoStack, projectTreeModel);
  new TagCommands(skrData, undoStack);
  ThemeManager::instance();
  TextBridge::instance();


  windowManager->restoreWindows();

  if (parser.isSet("testProject")) {
    projectCommands->loadProject(
        QUrl("qrc:/testfiles/skribisto_test_project.skrib"));
  }

  return app.exec();
}

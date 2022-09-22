#include "projectcommands.h"
#include "projecttreecommands.h"
#include "projecttreemodel.h"
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
#include <interfaces/projecttoolboxinterface.h>

int main(int argc, char *argv[]) {
  QGuiApplication::setHighDpiScaleFactorRoundingPolicy(
      Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);

  // QIcon::setFallbackSearchPaths(QIcon::fallbackSearchPaths() <<
  // ":/icons/backup/");
  QApplication::setStyle(new SkribistoStyle);

  QApplication app(argc, argv);

  //    QPalette palette = app.palette();
  //    palette.setColor(QPalette::All, QPalette::Window, QColor(Qt::white));
  // app.setPalette(palette);

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

  // load singletons :
  auto *skrData = new SKRData(qApp);
  new ProjectTreeModel(skrData);
  new ProjectCommands(skrData, undoStack);
  new ProjectTreeCommands(skrData, undoStack, projectTreeModel);
  ThemeManager::instance();

  // load plugins:
  skrpluginhub->addPluginType<ProjectToolboxInterface>();
  skrpluginhub->addPluginType<PageInterface>();
  skrpluginhub->addPluginType<ItemExporterInterface>();

  windowManager->restoreWindows();

  if (parser.isSet("testProject")) {
    skrdata->projectHub()->loadProject(
        QUrl("qrc:/testfiles/skribisto_test_project.skrib"));
  }

  return app.exec();
}

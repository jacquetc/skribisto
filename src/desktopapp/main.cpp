#include "backupmanager.h"
#include "commands.h"
#include "desktopapplication.h"
#include "dictcommands.h"
#include "interfaces/newprojectformatinterface.h"
#include "interfaces/pagetypeiconinterface.h"
#include "interfaces/projectpageinterface.h"
#include "projectcommands.h"
#include "projecttreecommands.h"
#include "skrdata.h"
#include "skribistostyle.h"
#include "tagcommands.h"
#include "thememanager.h"
#include "treemodels/projecttreemodel.h"
#include "welcomedialog.h"
#include "windowmanager.h"
#include <QApplication>
#include <QGuiApplication>
#include <QSettings>
#include <QSplashScreen>
#include <QStyleFactory>
#include <interfaces/itemexporterinterface.h>
#include <interfaces/pagedesktopinterface.h>
#include <interfaces/pageinterface.h>
#include <interfaces/projecttoolboxinterface.h>
#include <text/textbridge.h>

int main(int argc, char *argv[])
{
    QGuiApplication::setHighDpiScaleFactorRoundingPolicy(Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);

    // QIcon::setFallbackSearchPaths(QIcon::fallbackSearchPaths() <<
    // ":/icons/backup/");

    qputenv("QT_STYLE_OVERRIDE", "");

    DesktopApplication app(argc, argv);
    QApplication::setStyle(new SkribistoStyle);

    QPixmap pixmap(":/icons/skribisto/skribisto.png");
    QSplashScreen splash(pixmap, Qt::WindowStaysOnTopHint);
    splash.show();
    app.processEvents();

    // Names for the QSettings
    QCoreApplication::setOrganizationName("skribisto");
    QCoreApplication::setOrganizationDomain("skribisto.eu");
    QCoreApplication::setApplicationVersion(QString("%1.%2.%3")
                                                .arg(QString::number(SKR_VERSION_MAJOR),
                                                     QString::number(SKR_VERSION_MINOR),
                                                     QString::number(SKR_VERSION_PATCH)));
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

    parser.addOptions({// Open test project (-t, --testProject)
                       {{"t", "testProject"}, QCoreApplication::translate("main", "Show progress during copy")}});
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
    new DictCommands(skrData, undoStack);
    new BackupManager(skrData);
    QObject::connect(&app, &DesktopApplication::settingsChanged, backupManager, &BackupManager::settingsChanged);

    ThemeManager::instance();
    TextBridge::instance();

    QMainWindow *window = windowManager->restoreWindows();

    QSettings settings;
    bool showWelcome = settings.value("main/showWelcome", true).toBool();
    bool alwaysOpenLastProjectAtStartup = settings.value("main/alwaysOpenLastProjectAtStartup", false).toBool();

    if (!alwaysOpenLastProjectAtStartup && showWelcome && args.empty())
    {
        WelcomeDialog *welcomeDialog = new WelcomeDialog();
        welcomeDialog->show();
        welcomeDialog->setFocus();
    }
    else if (!args.empty() && QUrl::fromLocalFile(args.first()).isValid())
    {
        QFileInfo fileInfo(args.first());

        if (fileInfo.exists() && fileInfo.isReadable())
        {
            projectCommands->loadProject(QUrl::fromLocalFile(args.first()));
        }
    }
    else if (parser.isSet("testProject"))
    {
        projectCommands->loadProject(QUrl("qrc:/testfiles/skribisto_test_project.skrib"));
    }
    else if (alwaysOpenLastProjectAtStartup)
    {
        QUrl lastProject = settings.value("main/lastProject", QUrl()).toUrl();

        projectCommands->loadProject(lastProject);
    }

    splash.finish(window);

    return app.exec();
}

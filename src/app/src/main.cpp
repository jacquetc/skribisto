#include "iostream"
#include <QSettings>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QtSvg>
using namespace std;

// for translator

#include <QDebug>
#include <QString>
#include <QGuiApplication>
#include <QApplication>
#include <QFileInfo>
#include <QDir>
#include <QQuickStyle>
#include <QIcon>
#include <stdio.h>
#include <stdlib.h>

#include "skrpluginloader.h"
#include "plmdata.h"
#include "skrtreehub.h"
#include "skrtaghub.h"
#include "skrresult.h"
#include "plmprojecthub.h"
#include "skrprojectdicthub.h"
#include "skrerrorhub.h"
#include "skrpropertyhub.h"
#include "skrstathub.h"
#include "documenthandler.h"
#include "skrhighlighter.h"
#include "skrspellchecker.h"
#include "plmutils.h"
#include "skrthemes.h"
#include "skrexporter.h"
#include "skrclipboard.h"
#include "skr.h"
#include "models/skrtaglistmodel.h"
#include "models/skrsearchtreelistproxymodel.h"
#include "models/skrsearchtaglistproxymodel.h"
#include "models/skrmodels.h"
#include "skrrecentprojectlistmodel.h"
#include "skrusersettings.h"
#include "skrfonts.h"
#include "skreditmenusignalhub.h"
#include "skrqmltools.h"
#include "skrrootitem.h"
#include "skrtextbridge.h"
#include "skrwindowmanager.h"
#include "skrviewmanager.h"
#include "skrtreemanager.h"

#ifdef SKR_DEBUG
# include <QQmlDebuggingEnabler>
#endif // SKR_DEBUG
// -------------------------------------------------------
void startCore()
{
    // new PLMPluginLoader(qApp);

    // Names for the QSettings
    QCoreApplication::setOrganizationName("skribisto");
    QCoreApplication::setOrganizationDomain("skribisto.eu");

    QCoreApplication::setApplicationVersion(QString::number(
                                                SKR_VERSION_MAJOR) + "." + QString::number(
                                                SKR_VERSION_MINOR));
    qDebug() << QCoreApplication::applicationVersion();
    QString appName = "Skribisto";

    QCoreApplication::setApplicationName(appName);
    QSettings::setDefaultFormat(QSettings::IniFormat);
}

// -------------------------------------------------------

void myMessageOutput(QtMsgType type, const QMessageLogContext& context, const QString& msg)
{
    QByteArray  localMsg = msg.toLocal8Bit();
    const char *file     = context.file ? context.file : "";
    const char *function = context.function ? context.function : "";

    switch (type) {
    case QtDebugMsg:
        fprintf(stderr, "Debug: %s (%s:%u, %s)\n", localMsg.constData(), file, context.line, function);
        break;

    case QtInfoMsg:
        fprintf(stderr, "Info: %s (%s:%u, %s)\n", localMsg.constData(), file, context.line, function);
        break;

    case QtWarningMsg:
        fprintf(stderr, "Warning: %s (%s:%u, %s)\n", localMsg.constData(), file, context.line, function);
        break;

    case QtCriticalMsg:
        fprintf(stderr, "Critical: %s (%s:%u, %s)\n", localMsg.constData(), file, context.line, function);
        break;

    case QtFatalMsg:
        fprintf(stderr, "Fatal: %s (%s:%u, %s)\n", localMsg.constData(), file, context.line, function);
        break;
    }
}

//// -------------------------------------------------------

// void openProjectInArgument(PLMData *data)
// {
//    // open directly a project if *.skribisto path is the first argument :
//    // TODO: add ignore --qml
//    QStringList args = qApp->arguments();

//    if (args.count() > 1) {
//        QString argument;

//        for (int i = 1; i <= args.count() - 1; ++i) {
//            if (QFileInfo(args.at(i)).exists()) {
//                argument = args.at(i);
//                break;
//            }
//        }

//        if (!argument.isEmpty()) {
// # ifdef Q_OS_WIN32
//            QTextCodec *codec =
// QTextCodec::codecForUtfText(argument.toUtf8());
//            argument = codec->toUnicode(argument.toUtf8());
// # endif // ifdef Q_OS_WIN32
//            argument = QDir::fromNativeSeparators(argument);

//            data->projectHub()->loadProject(argument);
//        }
//    }
// }


// -------------------------------------------------------


// -------------------------------------------------------

// -------------------------------------------------------

int main(int argc, char *argv[])
{
#ifdef SKR_DEBUG
    QQmlDebuggingEnabler enabler;

    QLoggingCategory::defaultCategory()->setEnabled(QtDebugMsg, true);
    qInstallMessageHandler(myMessageOutput);
#endif // SKR_DEBUG

    // Allows qml styling
    qputenv("QT_STYLE_OVERRIDE", "");


    // QIcon::setFallbackSearchPaths(QIcon::fallbackSearchPaths() << ":icons");

    // TODO : add option for UI scale

#if QT_VERSION >= 0x051400
    QGuiApplication::setHighDpiScaleFactorRoundingPolicy(
        Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);
#else // if QT_VERSION >= 0x051400

    // qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", QByteArray("0"));
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
#endif // if QT_VERSION >= 0x051400


    QApplication app(argc, argv);

    // icons :
    // qDebug() << "icon search paths :" << QIcon::themeSearchPaths();

    // if Gnome desktop :
    //    if(qgetenv("XDG_CURRENT_DESKTOP") == "GNOME"){

    //        QIcon::setThemeName("Adwaita");
    //    }
    //    else {

    // BUG preventing the use of basic breeze theme
    // https://bugreports.qt.io/browse/QTBUG-87583
    // instead, I am picking "actions" and "animations" folders from Breeze
    // QIcon::setThemeName(QStringLiteral("breeze"));

    // QIcon::setThemeName(QStringLiteral("Adwaita"));

    //    }

    startCore();


    // QQuickStyle::setStyle("org.kde.desktop");

    // -----------------------------------------------------------------------


    // Language :

    // install translation of plugins:
    //    PLMPluginLoader::instance()->installPluginTranslations();


    // -----------------------------------------------------------------------

    QQmlApplicationEngine engine(qApp);

    PLMData *data         = new PLMData(&engine);
    SKRRootItem *rootItem = new SKRRootItem(&engine);
    rootItem->applyLanguageFromSettings();


    SKRModels *models                          = new SKRModels(&engine);
    SKRFonts  *skrFonts                        = new SKRFonts(&engine);
    SKREditMenuSignalHub *skrEditMenuSignalHub = new SKREditMenuSignalHub(&engine);
    SKRQMLTools     *skrQMLTools               = new SKRQMLTools(&engine);
    SKRTextBridge   *skrTextBridge             = new SKRTextBridge(&engine);
    SKRUserSettings *skrUserSettings           = new SKRUserSettings(&engine);
    SKRTreeManager  *skrTreeManager            = new SKRTreeManager(&engine);

    qmlRegisterUncreatableType<SKRResult>("eu.skribisto.result",
                                          1,
                                          0,
                                          "SKRResult",
                                          "Can't instantiate SKRResult");


    qmlRegisterUncreatableType<PLMProjectHub>("eu.skribisto.projecthub",
                                              1,
                                              0,
                                              "PLMProjectHub",
                                              "Can't instantiate PLMProjectHub");

    qmlRegisterUncreatableType<SKRTreeHub>("eu.skribisto.treehub",
                                           1,
                                           0,
                                           "SKRTreeHub",
                                           "Can't instantiate SKRTreeHub");

    qmlRegisterUncreatableType<SKRPluginHub>("eu.skribisto.pluginhub",
                                             1,
                                             0,
                                             "SKRPluginHub",
                                             "Can't instantiate SKRPluginHub");


    qmlRegisterUncreatableType<SKRTagHub>("eu.skribisto.taghub",
                                          1,
                                          0,
                                          "SKRTagHub",
                                          "Can't instantiate SKRTagHub");

    qmlRegisterUncreatableType<SKRProjectDictHub>("eu.skribisto.projectdicthub",
                                                  1,
                                                  0,
                                                  "SKRProjectDictHub",
                                                  "Can't instantiate SKRProjectDictHub");

    qmlRegisterUncreatableType<SKRPropertyHub>("eu.skribisto.propertyhub",
                                               1,
                                               0,
                                               "SKRPropertyHub",
                                               "Can't instantiate SKRPropertyHub");

    qmlRegisterUncreatableType<SKRStatHub>("eu.skribisto.stathub",
                                           1,
                                           0,
                                           "SKRStatHub",
                                           "Can't instantiate SKRStatHub");

    qmlRegisterUncreatableType<SKRErrorHub>("eu.skribisto.errorhub",
                                            1,
                                            0,
                                            "SKRErrorHub",
                                            "Can't instantiate SKRErrorHub");


    qmlRegisterUncreatableType<SKR>("eu.skribisto.skr",
                                    1,
                                    0,
                                    "SKR",
                                    "Can't instantiate SKR");

    qmlRegisterUncreatableType<SKRModels>("eu.skribisto.models",
                                          1,
                                          0,
                                          "SKRModels",
                                          "Can't instantiate SKRModels");


    qmlRegisterUncreatableType<PLMWriteDocumentListModel>(
        "eu.skribisto.writedocumentlistmodel",
        1,
        0,
        "PLMWriteDocumentListModel",
        "Can't instantiate PLMWriteDocumentListModel");

    qmlRegisterType<SKRSearchTreeListProxyModel>("eu.skribisto.searchtreelistproxymodel",
                                                 1,
                                                 0,
                                                 "SKRSearchTreeListProxyModel");


    qmlRegisterType<SKRSearchTagListProxyModel>("eu.skribisto.searchtaglistproxymodel",
                                                1,
                                                0,
                                                "SKRSearchTagListProxyModel");

    qmlRegisterType<SKRRecentProjectListModel>("eu.skribisto.recentprojectlistmodel",
                                               1,
                                               0,
                                               "SKRRecentProjectListModel");

    qmlRegisterType<DocumentHandler>("eu.skribisto.documenthandler",
                                     1,
                                     0,
                                     "DocumentHandler");

    qmlRegisterUncreatableType<SKRHighlighter>("eu.skribisto.highlighter",
                                               1,
                                               0,
                                               "Highlighter",
                                               "Can't instantiate SKRHighlighter");

    qmlRegisterType<SKRSpellChecker>("eu.skribisto.spellchecker",
                                     1,
                                     0,
                                     "SKRSpellChecker");

    qmlRegisterType<SKRUserSettings>("eu.skribisto.usersettings",
                                     1,
                                     0,
                                     "SKRUserSettings");

    qmlRegisterType<SKRThemes>("eu.skribisto.themes",
                               1,
                               0,
                               "SKRThemes");

    qmlRegisterType<SKRExporter>("eu.skribisto.exporter",
                                 1,
                                 0,
                                 "SKRExporter");

    qmlRegisterType<SKRClipboard>("eu.skribisto.clipboard",
                                  1,
                                  0,
                                  "SKRClipboard");

    qmlRegisterType<SKRViewManager>("eu.skribisto.viewmanager",
                                    1,
                                    0,
                                    "SKRViewManager");


    const QUrl url(QStringLiteral("qrc:/qml/main.qml"));

    SKRWindowManager *skrWindowManager = new SKRWindowManager(qApp, &engine, url);

    engine.rootContext()->setContextProperty("plmData", data);
    engine.rootContext()->setContextProperty("skrRootItem", rootItem);
    engine.rootContext()->setContextProperty("skrModels", models);
    engine.rootContext()->setContextProperty("skrFonts", skrFonts);
    engine.rootContext()->setContextProperty("skrQMLTools", skrQMLTools);
    engine.rootContext()->setContextProperty("skrEditMenuSignalHub", skrEditMenuSignalHub);
    engine.rootContext()->setContextProperty("skrTextBridge", skrTextBridge);
    engine.rootContext()->setContextProperty("skrWindowManager", skrWindowManager);
    engine.rootContext()->setContextProperty("skrUserSettings", skrUserSettings);
    engine.rootContext()->setContextProperty("skrTreeManager", skrTreeManager);


    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl& objUrl) {
        if (!obj && (url == objUrl)) QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);

    skrWindowManager->restoreWindows();


    //            QCoreApplication *app = qApp;
    //            engine->connect(engine, &QQmlApplicationEngine::objectCreated,
    // [app](QObject *object, const QUrl &url){
    //                if(object == nullptr){
    //                    app->quit();
    //                }
    //            });

    if (engine.rootObjects().isEmpty()) return -1;


    int returnCode = app.exec();

    // restart :
    if (returnCode == -1)
    {
        QProcess *proc = new QProcess();
        proc->start(QCoreApplication::applicationFilePath(), app.arguments());
    }

    // restart with Fist step opened and at plugin page:
    if (returnCode == -2)
    {
        QStringList args = app.arguments();
        args << "--firstStepsAtPluginPage";
        QProcess *proc = new QProcess();
        proc->start(QCoreApplication::applicationFilePath(), args);
    }

    return returnCode;
}

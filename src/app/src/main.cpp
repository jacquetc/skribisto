#include "iostream"
#include <QSettings>
#include <QQmlApplicationEngine>
#include <QQmlContext>

using namespace std;

// for translator
#include <QTextCodec>
#include <QDebug>
#include <QString>
#include <QGuiApplication>
#include <QApplication>
#include <QFileInfo>
#include <QDir>
#include <QQuickStyle>
#include <QIcon>

#include "plmpluginloader.h"
#include "plmdata.h"
#include "plmsheethub.h"
#include "plmnotehub.h"
#include "skrtaghub.h"
#include "plmerror.h"
#include "plmprojecthub.h"
#include "skrprojectdicthub.h"
#include "plmpropertyhub.h"
#include "documenthandler.h"
#include "skrhighlighter.h"
#include "skrspellchecker.h"
#include "plmutils.h"
#include "skrthemes.h"
#include "models/plmsheetlistproxymodel.h"
#include "models/plmnotelistproxymodel.h"
#include "models/skrtaglistmodel.h"
#include "models/skrsearchsheetlistproxymodel.h"
#include "models/skrsearchnotelistproxymodel.h"
#include "models/skrsearchtaglistproxymodel.h"
#include "models/plmmodels.h"
#include "skrrecentprojectlistmodel.h"
#include "skrusersettings.h"
#include "skrfonts.h"
#include "skreditmenusignalhub.h"
#include "skrqmltools.h"
#include "skrrootitem.h"
#include "skrtextbridge.h"

#ifdef QT_DEBUG
# include <QQmlDebuggingEnabler>
#endif // QT_DEBUG
// -------------------------------------------------------
void startCore()
{
    // new PLMPluginLoader(qApp);

    // UTF-8 codec
    QTextCodec::setCodecForLocale(QTextCodec::codecForName("UTF-8"));

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
#ifdef QT_DEBUG
    QQmlDebuggingEnabler enabler;

    // QLoggingCategory::defaultCategory()->setEnabled(QtDebugMsg, true);
#endif // QT_DEBUG

    // Allows qml styling
    qputenv("QT_STYLE_OVERRIDE", "");


    QIcon::setFallbackSearchPaths(QIcon::fallbackSearchPaths() << ":icons");

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
    qDebug() << "icon search paths :" << QIcon::themeSearchPaths();

    // if Gnome desktop :
    //    if(qgetenv("XDG_CURRENT_DESKTOP") == "GNOME"){

    //        QIcon::setThemeName("Adwaita");
    //    }
    //    else {

    // BUG preventing the use of basic breeze theme
    // https://bugreports.qt.io/browse/QTBUG-87583
    // instead, I am picking "actions" and "animations" folders from Breeze
    QIcon::setThemeName(QStringLiteral("breeze-skribisto"));

    //    }

    startCore();


    // QQuickStyle::setStyle("org.kde.desktop");

    // -----------------------------------------------------------------------


    // Language :

    // install translation of plugins:
    //    PLMPluginLoader::instance()->installPluginTranslations();


    // -----------------------------------------------------------------------

    SKRRootItem *rootItem = new SKRRootItem(qApp);
    rootItem->applyLanguageFromSettings();


    PLMData   *data                            = new PLMData(qApp);
    PLMModels *models                          = new PLMModels(qApp);
    SKRFonts  *skrFonts                        = new SKRFonts(qApp);
    SKREditMenuSignalHub *skrEditMenuSignalHub = new SKREditMenuSignalHub(qApp);
    SKRQMLTools   *skrQMLTools                 = new SKRQMLTools(qApp);
    SKRTextBridge *skrTextBridge               = new SKRTextBridge(qApp);

    qmlRegisterUncreatableType<PLMError>("eu.skribisto.plmerror",
                                         1,
                                         0,
                                         "PLMError",
                                         "Can't instantiate PLMError");


    qmlRegisterUncreatableType<PLMProjectHub>("eu.skribisto.projecthub",
                                              1,
                                              0,
                                              "PLMProjectHub",
                                              "Can't instantiate PLMProjectHub");

    qmlRegisterUncreatableType<PLMNoteHub>("eu.skribisto.notehub",
                                           1,
                                           0,
                                           "PLMNoteHub",
                                           "Can't instantiate PLMNoteHub");

    qmlRegisterUncreatableType<PLMSheetHub>("eu.skribisto.sheethub",
                                            1,
                                            0,
                                            "PLMSheetHub",
                                            "Can't instantiate PLMSheetHub");

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

    qmlRegisterUncreatableType<PLMPropertyHub>("eu.skribisto.propertyhub",
                                               1,
                                               0,
                                               "PLMProjectDictHub",
                                               "Can't instantiate PLMPropertyHub");

    qmlRegisterUncreatableType<PLMModels>("eu.skribisto.models",
                                          1,
                                          0,
                                          "PLMModels",
                                          "Can't instantiate PLMModels");


    qmlRegisterUncreatableType<PLMWriteDocumentListModel>(
        "eu.skribisto.writedocumentlistmodel",
        1,
        0,
        "PLMWriteDocumentListModel",
        "Can't instantiate PLMWriteDocumentListModel");


    qmlRegisterType<PLMSheetListProxyModel>("eu.skribisto.sheetlistproxymodel",
                                            1,
                                            0,
                                            "PLMSheetListProxyModel");

    qmlRegisterType<PLMNoteListProxyModel>("eu.skribisto.notelistproxymodel",
                                           1,
                                           0,
                                           "PLMNoteListProxyModel");

    qmlRegisterType<SKRSearchNoteListProxyModel>("eu.skribisto.searchnotelistproxymodel",
                                                 1,
                                                 0,
                                                 "SKRSearchNoteListProxyModel");

    qmlRegisterType<SKRSearchSheetListProxyModel>(
        "eu.skribisto.searchsheetlistproxymodel",
        1,
        0,
        "SKRSearchSheetListProxyModel");

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

    QQmlApplicationEngine engine(qApp);
    const QUrl url(QStringLiteral("qrc:/qml/main.qml"));

    engine.rootContext()->setContextProperty("plmData", data);
    engine.rootContext()->setContextProperty("skrRootItem", rootItem);
    engine.rootContext()->setContextProperty("plmModels", models);
    engine.rootContext()->setContextProperty("skrFonts", skrFonts);
    engine.rootContext()->setContextProperty("skrQMLTools", skrQMLTools);
    engine.rootContext()->setContextProperty("skrEditMenuSignalHub",
                                             skrEditMenuSignalHub);
    engine.rootContext()->setContextProperty("skrTextBridge", skrTextBridge);

    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl& objUrl) {
        if (!obj && (url == objUrl)) QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);

    engine.load(url);

    //            QCoreApplication *app = qApp;
    //            engine->connect(engine, &QQmlApplicationEngine::objectCreated,
    // [app](QObject *object, const QUrl &url){
    //                if(object == nullptr){
    //                    app->quit();
    //                }
    //            });

    if (engine.rootObjects().isEmpty()) return -1;


    return app.exec();
}

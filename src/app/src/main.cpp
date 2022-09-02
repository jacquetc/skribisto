
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>

#include "app_environment.h"
#include "import_qml_plugins.h"
#include <QtQml/qqmlextensionplugin.h>
#include "skrthemes.h"


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
#include "skrdata.h"
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
#include "skrtreemanager.h"
#include "skrdownload.h"
#include "skrshortcutmanager.h"
#include "skrplugingetter.h"

int main(int argc, char *argv[])
{


Q_IMPORT_QML_PLUGIN(themePlugin)


    set_qt_environment();

    // Allows qml styling
    qputenv("QT_STYLE_OVERRIDE", "");

    // TODO : add option for UI scale

#if QT_VERSION >= 0x051400
    QGuiApplication::setHighDpiScaleFactorRoundingPolicy(
        Qt::HighDpiScaleFactorRoundingPolicy::RoundPreferFloor);
#else // if QT_VERSION >= 0x051400

    // qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", QByteArray("0"));
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
#endif // if QT_VERSION >= 0x051400


    QGuiApplication app(argc, argv);

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


/*    qmlRegisterSingletonType<SkrTools>(
        "backend", 1, 0, "SkrTools",
        [](QQmlEngine *, QJSEngine *) { return new SkrTools; });
*/
    qmlRegisterType<SKRThemes>("eu.skribisto.themes",
                               1,
                               0,
                               "SKRThemes");

    // -----------------------------------------------------------------------

    QQmlApplicationEngine engine(qApp);

    SKRData *data = new SKRData(&engine);

    SKRShortcutManager *shortcutManager = new SKRShortcutManager(&engine);
    SKRRootItem *rootItem               = new SKRRootItem(&engine);
    rootItem->connect(rootItem,
                      &SKRRootItem::currentTranslationLanguageCodeChanged,
                      shortcutManager,
                      &SKRShortcutManager::populateShortcutList);
    rootItem->applyLanguageFromSettings();

    SKRPluginGetter *skrPluginGetter           = new SKRPluginGetter(&engine);
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
                                          "skrResult",
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

    qmlRegisterType<SKRDownload>("eu.skribisto.download",
                                 1,
                                 0,
                                 "Download");

    qmlRegisterType<SKRThemes>("eu.skribisto.themes",
                                 1,
                                 0,
                                 "Themes");

    qmlRegisterType<SKRUserSettings>("eu.skribisto.usersettings",
                                     1,
                                     0,
                                     "SKRUserSettings");

    qmlRegisterType<SKRExporter>("eu.skribisto.exporter",
                                 1,
                                 0,
                                 "SKRExporter");

    qmlRegisterType<SKRClipboard>("eu.skribisto.clipboard",
                                  1,
                                  0,
                                  "SKRClipboard");



    const QUrl url(u"qrc:Main/main.qml"_qs);

    SKRWindowManager *skrWindowManager = new SKRWindowManager(qApp, &engine, url);

    engine.rootContext()->setContextProperty("skrData", data);
    engine.rootContext()->setContextProperty("skrShortcutManager", shortcutManager);
    engine.rootContext()->setContextProperty("skrRootItem", rootItem);
    engine.rootContext()->setContextProperty("skrPluginGetter", skrPluginGetter);
    engine.rootContext()->setContextProperty("skrModels", models);
    engine.rootContext()->setContextProperty("skrFonts", skrFonts);
    engine.rootContext()->setContextProperty("skrQMLTools", skrQMLTools);
    engine.rootContext()->setContextProperty("skrEditMenuSignalHub", skrEditMenuSignalHub);
    engine.rootContext()->setContextProperty("skrTextBridge", skrTextBridge);
    engine.rootContext()->setContextProperty("skrWindowManager", skrWindowManager);
    engine.rootContext()->setContextProperty("skrUserSettings", skrUserSettings);
    engine.rootContext()->setContextProperty("skrTreeManager", skrTreeManager);

    engine.addImportPath(QCoreApplication::applicationDirPath() + "/qml");
    engine.addImportPath(":/");


    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl& objUrl) {
        if (!obj && (url == objUrl)) QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);

#ifndef Q_OS_IOS
    skrWindowManager->restoreWindows();

#else
    skrWindowManager->addUniqueWindowForDevice();
#endif


    //            QCoreApplication *app = qApp;
    //            engine->connect(engine, &QQmlApplicationEngine::objectCreated,
    // [app](QObject *object, const QUrl &url){
    //                if(object == nullptr){
    //                    app->quit();
    //                }
    //            });

    if (engine.rootObjects().isEmpty()) return -1;


    int returnCode = app.exec();

#ifndef Q_OS_IOS
    // restart :
    if (returnCode == -1)
    {
        QProcess *proc = new QProcess();

        if (QFile("/app/manifest.json").exists()) { // means it's in Flatpak
            QStringList allArguments;
            allArguments << "--host" << "flatpak" <<  "run" << "eu.skribisto.skribisto";
            proc->start("flatpak-spawn", allArguments);
        }
        else {
            proc->start(QCoreApplication::applicationFilePath(), QStringList());
        }
    }

    // restart with Fist step opened and at plugin page:
    if (returnCode == -2)
    {
        QProcess *proc = new QProcess();

        if (QFile("/app/manifest.json").exists()) { // means it's in Flatpak
            QStringList allArguments;
            allArguments << "--host" << "flatpak" <<  "run" << "eu.skribisto.skribisto" << "--firstStepsAtPluginPage";
            proc->start("flatpak-spawn", allArguments);
        }
        else {
            QStringList args;
            args << "--firstStepsAtPluginPage";
            proc->start(QCoreApplication::applicationFilePath(), args);
        }
    }

#endif

    return returnCode;

}


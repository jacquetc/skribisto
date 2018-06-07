#include "iostream"
#include <QSettings>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QtGui/QGuiApplication>
#include <QApplication>

using namespace std;

// for translator
#include <QLibraryInfo>
#include <QLocale>
#include <QTextCodec>
#include <QDebug>
#include <QString>

#include <QTranslator>

#include "plmutils.h"
#include "plmpluginloader.h"
#include "plmdata.h"
#if FORCEQML == 0

#include <QtWidgets/QProxyStyle>
#include <QtWidgets/QSplashScreen>
#include <QtWidgets/QStyleFactory>

#include "plmutils.h"
#include "plmmainwindow.h"
#endif // if FORCEQML


// -------------------------------------------------------

void startCore()
{
    new PLMPluginLoader(qApp);
    // UTF-8 codec
    QTextCodec::setCodecForLocale(QTextCodec::codecForName("UTF-8"));

    // Names for the QSettings
    QCoreApplication::setOrganizationName("plume-creator");
    QCoreApplication::setOrganizationDomain("plume-creator.com");


    QCoreApplication::setApplicationVersion(QString::number(VERSIONSTR));
    QString appName = "Plume-Creator";
    QCoreApplication::setApplicationName(appName);
    QSettings::setDefaultFormat(QSettings::IniFormat);
}

 //-------------------------------------------------------
#if FORCEQML == 0

void selectLang()
{
    // Lang menu :
    QSettings settings;

    qApp->processEvents();
    QString qtTr    = QString("qt");
    QString plumeTr = QString("plume-creator");
    QLocale locale;


    QString langCode = settings.value("lang", "none").toString();

    if (langCode == "none") {
        // apply system locale by default
        locale = QLocale::system();
    }


    QTranslator plumeTranslator;

    if (plumeTranslator.load(locale, plumeTr, "_", ":/translations")) {
        settings.setValue("lang", locale.name());
    }
    else { // if translation not existing :
        locale = QLocale("en_EN");
        settings.setValue("lang", locale.name());
    }
    qApp->installTranslator(&plumeTranslator);

    PLMUtils::Lang::setUserLang(langCode);


    // Qt translation :
    QTranslator translator;

    if (translator.load(locale, qtTr, "_",
                        QLibraryInfo::location(QLibraryInfo::
                                               TranslationsPath))) {
        qApp->installTranslator(&translator);
    }

    // install translation of plugins:
    PLMPluginLoader::instance()->installPluginTranslations();
}

//// -------------------------------------------------------

void openProjectInArgument(PLMData *data)
{
    // open directly a project if *.plume path is the first argument :
    QStringList args = qApp->arguments();

    if (args.count() > 1) {
        QString argument;

        for (int i = 1; i <= args.count() - 1; ++i) {
            if (QFileInfo(args.at(i)).exists()) {
                argument = args.at(i);
                break;
            }
        }

        if (!argument.isEmpty()) {
# ifdef Q_OS_WIN32
            QTextCodec *codec = QTextCodec::codecForUtfText(argument.toUtf8());
            argument = codec->toUnicode(argument.toUtf8());
# endif // ifdef Q_OS_WIN32
            argument = QDir::fromNativeSeparators(argument);

            data->projectHub()->loadProject(argument);
        }
    }
}

#endif // if FORCEQML

// -------------------------------------------------------

//PLMData* startData()
//{
//

//    return data;
//}

// -------------------------------------------------------

#if FORCEQML == 0
PLMMainWindow* startGui(PLMData *data)
{
    // Q_INIT_RESOURCE(pics);
    // Q_INIT_RESOURCE(langs);
    // Q_INIT_RESOURCE(sounds);
    // splashscreen :
    QPixmap pixmap(":/pics/plume-creator.png");
    QSplashScreen *splash = new QSplashScreen(pixmap);

    splash->show();
    qApp->processEvents();
    qApp->setWindowIcon(QIcon(":/pics/plume-creator.png"));

    // ---------------------------------------style :
    splash->showMessage("Applying style");
    qApp->processEvents();
    QApplication::setStyle(QStyleFactory::create("fusion"));

    QApplication::setWheelScrollLines(1);

    splash->showMessage("Setting language");
    qApp->processEvents();
    selectLang();

    PLMMainWindow *mw = new PLMMainWindow(data);
    // install enventfilter (for Mac)
    QApplication::instance()->installEventFilter(mw);
    splash->finish(mw);
    mw->show();
    mw->setWindowState(Qt::WindowActive);





    return mw;
}

#endif // if FORCEQML

// -------------------------------------------------------

bool hasQMLArg(){
    return qApp->arguments().contains("--qml");
}

//-------------------------------------------------------

int main(int argc, char *argv[])
{
    // Allows qml styling
    qputenv("QT_STYLE_OVERRIDE", "");
    //qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", QByteArray("0"));


    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QGuiApplication::setQuitOnLastWindowClosed(true);

    QApplication app(argc, argv);

    startCore();
    PLMData *data = new PLMData(qApp);
    if(hasQMLArg() || (QString::number(FORCEQML) == "1")){
        //qmlRegisterType<PLMError>("eu.plumecreator.qml", 1, 0, "PLMError");
        qmlRegisterUncreatableType<PLMProjectHub>("eu.plumecreator.qml", 1, 0, "PLMProjectHub", "Can't instantiate PLMProjectHub");

        QQmlApplicationEngine engine;

        engine.rootContext()->setContextProperty("plmData", data);
        engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));
        if (engine.rootObjects().isEmpty())
            return -1;
    }
#if FORCEQML == 0
    else {
        PLMMainWindow *mw = startGui(data);
        Q_UNUSED(mw)
        openProjectInArgument(data);


    }
#endif // if FORCEQML



    return app.exec();
}

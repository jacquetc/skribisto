#include "iostream"
#include <QSettings>
#include <QtCore/QCoreApplication>
#include <QtWidgets/QApplication>

using namespace std;

// for translator
#include <QLibraryInfo>
#include <QLocale>
#include <QPixmap>
#include <QTextCodec>
#include <QTranslator>
#include <QtWidgets/QProxyStyle>
#include <QtWidgets/QSplashScreen>
#include <QtWidgets/QStyleFactory>
#include <QDebug>
#include <QString>

#include <QtPlugin>
#include <QtQml/QQmlApplicationEngine>
#include <QtWidgets/QInputDialog>


#if FORCEQML == 0
# include "plmutils.h"
# include "plmpluginloader.h"
# include "plmguiplugins.h"
# include "plmmainwindow.h"
#endif // ifndef FORCEQML

#include "plmdata.h"

QCoreApplication* createApplication(int& argc, char *argv[])
{
    for (int i = 1; i < argc; ++i) {
        if (!qstrcmp(argv[i], "--no-gui")) {
            return new QCoreApplication(argc, argv);
        }
    }

    return new QApplication(argc, argv);
}

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

// -------------------------------------------------------

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

// -------------------------------------------------------
#if FORCEQML == 0

void openProjectInArgument(PLMMainWindow *plmMainWindow)
{
    // open directly a project if *.plume path is the first argument :
    QStringList args = qApp->arguments();

    if (args.count() > 1) {
        QString argument;

        for (int i = 1; i <= args.count(); ++i) {
            if (QFileInfo(args.at(i)).exists()) {
                argument = args.at(i);
                break;
            }
        }

        if (!argument.isEmpty()) {
# ifdef Q_OS_WIN32
            QTextCodec *codec = QTextCodec::codecForUtfText(argument);
            argument = codec->toUnicode(argument);
# endif // ifdef Q_OS_WIN32
            argument = QDir::fromNativeSeparators(argument);

            // plmMainWindow->openProject(argument);
        }
    }
}

#endif // if FORCEQML

// -------------------------------------------------------

PLMData* startData()
{
    PLMData *data = new PLMData(qApp);

    return data;
}

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
    splash->showMessage("Loading plugins");
    qApp->processEvents();
    PLMGuiPlugins::addGuiPlugins();

    splash->showMessage("Setting language");
    qApp->processEvents();
    selectLang();

    PLMMainWindow *mw = new PLMMainWindow(data);
    // install enventfilter (for Mac)
    QApplication::instance()->installEventFilter(mw);
    splash->finish(mw);
    mw->show();
    mw->setWindowState(Qt::WindowActive);
     mw->init();





    return mw;
}

#endif // if FORCEQML

// -------------------------------------------------------

bool isQML()
{
    QStringList args = qApp->arguments();

    foreach(const QString &arg, args) {
        if (arg == "--qml") {
            return true;
        }
    }

    return false;
}

// -------------------------------------------------------

QQmlApplicationEngine* startQMLGui(PLMData *data)
{
    Q_INIT_RESOURCE(qml);
    QQmlApplicationEngine *engine = new QQmlApplicationEngine("qrc:/qml/main.qml");
    return engine;
}

// ----------------------------------------------------


int main(int argc, char *argv[])
{
    // Allows qml styling
    qputenv("QT_STYLE_OVERRIDE", "");

    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

    QApplication app(argc, argv);

    startCore();
    PLMData *data = startData();

    if (isQML() || (QString::number(FORCEQML) == 1)) {
        QQmlApplicationEngine *engine = startQMLGui(data);
    }

#if FORCEQML  == 0

    if (!isQML()) {
        PLMMainWindow *gui = startGui(data);
        openProjectInArgument(gui);
    }

#endif // ifndef FORCEQML

    // data->projectHub()->loadProject();
    return app.exec();
}

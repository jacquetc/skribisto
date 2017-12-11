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

#include <QtPlugin>
#include <QtQml/QQmlApplicationEngine>
#include <QtWidgets/QInputDialog>



#ifndef FORCEQML
#include "common/plmutils.h"
#include "plmpluginloader.h"
#include "plmguiplugins.h"
#include "plmmainwindow.h"
#endif

#include "plmdata.h"

QCoreApplication *createApplication(int &argc, char *argv[])
{
    for (int i = 1; i < argc; ++i) {
        if (!qstrcmp(argv[i], "--no-gui")) {
            return new QCoreApplication(argc, argv);
        }
    }

    return new QApplication(argc, argv);
}

//-------------------------------------------------------

void startCore()
{
    // UTF-8 codec
    QTextCodec::setCodecForLocale(QTextCodec::codecForName("UTF-8"));
    //Names for the QSettings
    QCoreApplication::setOrganizationName( "plume-creator" );
    QCoreApplication::setOrganizationDomain( "plume-creator.com" );
    QCoreApplication::setApplicationVersion(QString::number(VERSIONSTR));
    QString appName = "Plume-Creator";
    QCoreApplication::setApplicationName(appName);
    QSettings::setDefaultFormat(QSettings::IniFormat);
}

//-------------------------------------------------------

void selectLang()
{
#ifndef FORCEQML
    //TODO: since app, core and gui are separated now, adapt the lang loader
    qRegisterMetaType<QList<PLMTranslation> >("QList<PLMTranslation>");
    // Lang menu :
    QSettings settings;
    qApp->processEvents();
    QString translatorFileName = QLatin1String("qt_");
    QList<PLMTranslation> trList = PLMUtils::Lang::getTranslationsList();
    QString plumeTranslatorFileName;

    if (settings.value("firstStart",
                       true).toBool() ||
        settings.value("MainWindow/lang", "none").toString() == "none" ) {
        QStringList langs;
        QStringList langCodes;

        foreach (const PLMTranslation &trans, trList) {
            langs << trans.name;
            langCodes << trans.code;
        }

        //        QStringList langs;
        //        langs << "Français" << "English" << "Italiano" << "Deutsch" << "Português (Brasil)" << "Español (España)" << "Pусский";
        //        QStringList langCodes;
        //        langCodes << "fr_FR" << "en_US" << "it_IT" << "de_DE" << "pt_BR"<< "sp_SP" << "ru_RU";
        bool ok;
        QString selectedLang;
        selectedLang = QInputDialog::getItem(0,
                                             "Language Selector",
                                             "Please select your language : <br> "
                                             "Veuillez sélectionner votre langue : ",
                                             langs,
                                             0,
                                             false,
                                             &ok);

        if (ok && !selectedLang.isEmpty()) {
            QString langCode = langCodes.at(langs.indexOf(selectedLang));
            translatorFileName = langCode;

            foreach (const PLMTranslation &trans, trList)
                if (trans.code == langCode) {
                    plumeTranslatorFileName = trans.file;
                }

            QTranslator *translator = new QTranslator(qApp);

            if (translator->load(translatorFileName,
                                 QLibraryInfo::location(QLibraryInfo::TranslationsPath))) {
                qApp->installTranslator(translator);
            }

            QTranslator *plumeTranslator = new QTranslator(qApp);
            plumeTranslator->load(plumeTranslatorFileName);
            qApp->installTranslator(plumeTranslator);
            settings.setValue("lang", langCodes.at(langs.indexOf(selectedLang)));
            // set special translator system lang :
            PLMUtils::Lang::setUserLang(langCode);
            PLMUtils::Lang::setUserLangFile(plumeTranslatorFileName);
        } else { // if cancel dialog
            QString langCode = QLocale::system().name();

            foreach (const PLMTranslation &trans, trList)
                if (trans.code == langCode || trans.code ==
                    QLocale::languageToString(QLocale::system().language())) {
                    plumeTranslatorFileName = trans.file;
                }

            QTranslator *translator = new QTranslator(qApp);

            if (translator->load(translatorFileName,
                                 QLibraryInfo::location(QLibraryInfo::TranslationsPath))) {
                qApp->installTranslator(translator);
            }

            plumeTranslatorFileName += langCode;
            QTranslator *plumeTranslator = new QTranslator(qApp);
            plumeTranslator->load(plumeTranslatorFileName);
            qApp->installTranslator(plumeTranslator);
            settings.setValue("lang", QLocale::system().name());
            // set special translator system lang :
            PLMUtils::Lang::setUserLang(langCode);
            PLMUtils::Lang::setUserLangFile(plumeTranslatorFileName);
        }
    } else { // if there is a lang setting
        QString langCode = settings.value("lang", "none").toString();

        foreach (const PLMTranslation &trans, trList)
            if (trans.code == langCode) {
                plumeTranslatorFileName = trans.file;
            }

        QTranslator *translator = new QTranslator(qApp);

        if (translator->load(translatorFileName,
                             QLibraryInfo::location(QLibraryInfo::TranslationsPath))) {
            qApp->installTranslator(translator);
        }

        QTranslator *plumeTranslator = new QTranslator(qApp);
        plumeTranslator->load(plumeTranslatorFileName);
        qApp->installTranslator(plumeTranslator);
        // set special translator system lang :
        PLMUtils::Lang::setUserLang(langCode);
        PLMUtils::Lang::setUserLangFile(plumeTranslatorFileName);
    }

    //install translation of plugins:
    PLMPluginLoader::instance()->installPluginTranslations();
#endif
}

//-------------------------------------------------------
#ifndef FORCEQML

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
#ifdef Q_OS_WIN32
            QTextCodec *codec = QTextCodec::codecForUtfText(argument);
            argument = codec->toUnicode(argument);
#endif
            argument = QDir::fromNativeSeparators(argument);
            //plmMainWindow->openProject(argument);
        }
    }
}
#endif

//-------------------------------------------------------

PLMData *startData()
{
    PLMData *data = new PLMData(qApp);
    return data;
}

//-------------------------------------------------------

#ifndef FORCEQML
PLMMainWindow *startGui(PLMData *data)
{
    //Q_INIT_RESOURCE(pics);
    //Q_INIT_RESOURCE(langs);
    //Q_INIT_RESOURCE(sounds);
    // splashscreen :
    QPixmap pixmap(":/pics/plume-creator-splash.png");
    QSplashScreen *splash = new QSplashScreen(pixmap);
    splash->show();
    qApp->processEvents();
    qApp->setWindowIcon(QIcon(":/pics/plume-creator.png"));
    // ---------------------------------------style :
    splash->showMessage("Applying style");
    qApp->processEvents();
    QApplication::setStyle(QStyleFactory::create("fusion"));
    splash->showMessage("Load plugins");
    qApp->processEvents();
    new PLMPluginLoader(qApp);
    PLMGuiPlugins::addGuiPlugins();
    splash->showMessage("Setting language");
    qApp->processEvents();
    selectLang();
    QApplication::setWheelScrollLines(1);
    PLMMainWindow *mw = new PLMMainWindow(data);
    // install enventfilter (for Mac)
    QApplication::instance()->installEventFilter(mw);
    splash->finish(mw);
    mw->show();
    mw->setWindowState(Qt::WindowActive);
    mw->init();
    return mw;
}
#endif

//-------------------------------------------------------

bool isQML()
{
    QStringList args = qApp->arguments();

    foreach (const QString &arg, args) {
        if (arg == "--qml") {
            return true;
        }
    }

    return false;
}

//-------------------------------------------------------

QQmlApplicationEngine *startQMLGui(PLMData *data)
{
    Q_INIT_RESOURCE(qml);
    QQmlApplicationEngine *engine = new QQmlApplicationEngine("qrc:/qml/main.qml");
    return engine;
}

//----------------------------------------------------



int main(int argc, char *argv[])
{
    QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
    QApplication app(argc, argv);
    startCore();
    PLMData *data = startData();
//    if (isQML() || FORCEQML == 1) {
#ifdef FORCEQML
    QQmlApplicationEngine *engine = startQMLGui(data);
#endif
#ifndef FORCEQML
//        PLMMainWindow *gui = startGui(data);
    //    openProjectInArgument(gui);
#endif
    //data->projectHub()->loadProject();
    return app.exec();
}

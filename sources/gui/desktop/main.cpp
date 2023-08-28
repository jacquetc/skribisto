#include "domain_registration.h"
#include "mainwindow.h"

#include "persistence_registration.h"
#include "presenter_registration.h"
#include <QApplication>
#include <QLocale>
#include <QTranslator>

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);

    // Names for the QSettings
    QCoreApplication::setOrganizationName("skribisto");
    QCoreApplication::setOrganizationDomain("skribisto.eu");

    //    QCoreApplication::setApplicationVersion(QString("%1.%2.%3")
    //
    //
    //
    //
    //
    //                                  .arg(QString::number(SKR_VERSION_MAJOR),
    //
    //
    //
    //
    //
    //                                       QString::number(SKR_VERSION_MINOR),
    //
    //
    //
    //
    //
    //
    //                                    QString::number(SKR_VERSION_PATCH)));
    qDebug() << QCoreApplication::applicationVersion();
    QString appName = "Skribisto";
    QCoreApplication::setApplicationName(appName);

    // Translations

    QTranslator translator;
    const QStringList uiLanguages = QLocale::system().uiLanguages();

    for (const QString &locale : uiLanguages)
    {
        const QString baseName = "skribisto-desktop_" + QLocale(locale).name();

        if (translator.load(":/i18n/" + baseName))
        {
            app.installTranslator(&translator);
            break;
        }
    }

    auto *domainRegistration = new Domain::DomainRegistration(&app);
    auto persistence = new Persistence::PersistenceRegistration(&app, domainRegistration->entitySchema());
    new Presenter::PresenterRegistration(&app, persistence->repositoryProvider());

    MainWindow w;
    w.show();
    return app.exec();
}

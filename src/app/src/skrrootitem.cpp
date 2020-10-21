#include "skrrootitem.h"
#include <QLocale>
#include <QLibraryInfo>
#include <QApplication>
#include <QSettings>

SKRRootItem::SKRRootItem(QObject *parent) : QObject(parent)
{
skribistoTranslator = new QTranslator(this);
qtTranslator = new QTranslator(this);
}
void SKRRootItem::applyLanguageFromSettings(){
    QSettings settings;
    QString langCode = settings.value("lang", "default").toString();

    this->setCurrentTranslationLanguageCode(langCode);
}

QString SKRRootItem::getLanguageFromSettings() const{
    QString langCode;
    QSettings settings;
    langCode = settings.value("lang", "default").toString();

    if(langCode == "default"){
        langCode =   QLocale::languageToString(QLocale::system().language());
    }
    return langCode;
}


void SKRRootItem::setCurrentTranslationLanguageCode(const QString &langCode)
{

    QSettings settings;

    QLocale locale;



    if (langCode == "default") {
        // apply system locale by default
        locale = QLocale::system();
    }
    else {
        locale = QLocale(langCode);
    }


    // Qt translation :
    if(!qtTranslator->isEmpty()){
        qApp->removeTranslator(qtTranslator);
    }

    if (qtTranslator->load(locale, "qt", "_",
                        QLibraryInfo::location(QLibraryInfo::
                                               TranslationsPath))) {




        qApp->installTranslator(qtTranslator);
    }

    //Skribisto translation

    if(!skribistoTranslator->isEmpty()){
        qApp->removeTranslator(skribistoTranslator);
    }

    if (skribistoTranslator->load(locale, "skribisto", "_", ":/translations")) {
        settings.setValue("lang", locale.name());



        qApp->installTranslator(skribistoTranslator);




    }
    else { // if translation not existing, use default :
        locale = QLocale("en_US");
        settings.setValue("lang", locale.name());

        if(!skribistoTranslator->isEmpty()){
            qApp->removeTranslator(skribistoTranslator);
        }
        if(!qtTranslator->isEmpty()){
            qApp->removeTranslator(qtTranslator);
        }

    }

    // PLMUtils::Lang::setUserLang(langCode);

    m_langCode = langCode;


    emit currentTranslationLanguageCodeChanged(locale.name());


}

 QString SKRRootItem::skribistoVersion() const{

      QStringList strings;
      strings << QString::number(SKR_VERSION_MAJOR) <<  QString::number(SKR_VERSION_MINOR) <<  QString::number(SKR_VERSION_PATCH) ;


      return strings.join(".");
}

 QString SKRRootItem::toLocaleDateTimeFormat(const QDateTime &dateTime) const{

     QLocale locale(m_langCode);


     return dateTime.toString(locale.dateTimeFormat());
}

#include "skrrootitem.h"
#include "plmutils.h"
#include "plmdata.h"
#include <QLocale>
#include <QLibraryInfo>
#include <QApplication>
#include <QSettings>
#include <QFont>
#include <QDebug>

SKRRootItem::SKRRootItem(QObject *parent) : QObject(parent)
{
    skribistoTranslator = new QTranslator(this);
    qtTranslator        = new QTranslator(this);
}

void SKRRootItem::applyLanguageFromSettings() {
    QSettings settings;
    QString   langCode = settings.value("lang", "default").toString();

    this->setCurrentTranslationLanguageCode(langCode);
}

QString SKRRootItem::getLanguageFromSettings() const {
    QString   langCode;
    QSettings settings;

    langCode = settings.value("lang", "default").toString();

    if (langCode == "default") {
        langCode =   QLocale::languageToString(QLocale::system().language());
    }
    return langCode;
}


void SKRRootItem::setCurrentTranslationLanguageCode(const QString& langCode)
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
    if (!qtTranslator->isEmpty()) {
        qApp->removeTranslator(qtTranslator);
    }

    if (qtTranslator->load(locale, "qt", "_",
                           QLibraryInfo::location(QLibraryInfo::
                                                  TranslationsPath))) {
        qApp->installTranslator(qtTranslator);
    }

    // Skribisto translation

    if (!skribistoTranslator->isEmpty()) {
        qApp->removeTranslator(skribistoTranslator);
    }

    //qDebug() << "findTranslationDir" << findTranslationDir();

    if (skribistoTranslator->load(locale, "skribisto", "_", findTranslationDir())) {
        QString name = locale.name();

        if ((name == "eo_001")) {
            name = "eo";
        }

        settings.setValue("lang", name);


        qApp->installTranslator(skribistoTranslator);
    }
    else { // if translation not existing, use default :
        locale = QLocale("en_US");
        settings.setValue("lang", locale.name());

        if (!skribistoTranslator->isEmpty()) {
            qApp->removeTranslator(skribistoTranslator);
        }

        if (!qtTranslator->isEmpty()) {
            qApp->removeTranslator(qtTranslator);
        }
    }

    // PLMUtils::Lang::setUserLang(langCode);

    m_langCode = langCode;


    emit currentTranslationLanguageCodeChanged(locale.name());
}

QString SKRRootItem::findTranslationDir() const {
    QStringList addonsPathsList = PLMUtils::Dir::addonsPathsList();

    QString dirPath;
    QDir    dir;

    for (const QString& path : qAsConst(addonsPathsList)) {
        dir.setPath(path + "/translations");

        QStringList fileList = dir.entryList(QDir::Files | QDir::NoDotAndDotDot);

        for (const QString& file : qAsConst(fileList)) {
            if (file.right(3) == ".qm") {
                dirPath = path  + "/translations";
                break;
            }
        }

        if (!dirPath.isEmpty()) {
            break;
        }
    }
    return dirPath;
}

QVariantMap SKRRootItem::findAvailableTranslationsMap() const {
    QVariantMap translationMap;

    QString dirPath = findTranslationDir();

    QDir dir;

    dir.setPath(dirPath);
    QStringList filters;

    filters << "*.qm";
    QStringList fileList = dir.entryList(filters, QDir::Files | QDir::NoDotAndDotDot);
    QTranslator translator;

    for (const QString& file : qAsConst(fileList)) {
        if (translator.load(file, dirPath)) {
            QString langCode = translator.language();

            QLocale locale(langCode);

            if ((langCode == "eo_001")) {
                langCode = "eo";
            }

            QString langName = locale.nativeLanguageName();

            if ((langCode == "vls") && langName.isEmpty()) {
                langName = "Westvlams";
            }

            translationMap.insert(langCode, langName);

            // qDebug() << langCode << langName;
        }
    }

    if (translationMap.isEmpty()) {
        plmdata->errorHub()->addWarning(tr("No translation found."));
    }


    return translationMap;
}

// ------------------------------------------------------

QString SKRRootItem::cleanUpHtml(const QString& html)
{
    QString text = html;

    QStringList styleToRemoveList;

    styleToRemoveList << "font-family";
    styleToRemoveList << "font-size";
    styleToRemoveList << "font-style";
    styleToRemoveList << "margin-left";
    styleToRemoveList << "margin-right";
    styleToRemoveList << "margin-top";
    styleToRemoveList << "margin-bottom";
    styleToRemoveList << "-qt-block-indent";
    styleToRemoveList << "-qt-user-state";
    styleToRemoveList << "text-indent";

    for (const QString& style : qAsConst(styleToRemoveList)) text.remove(QRegularExpression(" " + style + ":.*?;"));
    text.remove(QRegularExpression("<h[0-9].*?>"));
    text.remove(QRegularExpression("<h[0-9]>"));
    return text;
}

// ------------------------------------------------------

QString SKRRootItem::skribistoVersion() const {
    QStringList strings;

    strings << QString::number(SKR_VERSION_MAJOR) <<
        QString::number(SKR_VERSION_MINOR) <<  QString::number(SKR_VERSION_PATCH);


    return strings.join(".");
}

QString SKRRootItem::toLocaleDateTimeFormat(const QDateTime& dateTime) const {
    QLocale locale(m_langCode);

    return dateTime.toString(locale.dateTimeFormat());
}

QString SKRRootItem::toLocaleIntString(int number) const {
    QLocale locale(m_langCode);

    return locale.toString(number);
}

QString SKRRootItem::getQtVersion() const {
    return QString(QT_VERSION_STR);
}

bool SKRRootItem::hasPrintSupport() const {
#ifdef SKR_PRINT_SUPPORT
    return true;

#else // ifdef SKR_PRINT_SUPPORT
    return false;

#endif // SKR_PRINT_SUPPORT
}

QString SKRRootItem::defaultFontFamily() const {
    return qGuiApp->font().family().replace(", ", "");
}

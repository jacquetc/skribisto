#include "skrrootitem.h"
#include "plmutils.h"
#include "skrdata.h"
#include <QLocale>
#include <QLibraryInfo>
#include <QApplication>
#include <QSettings>
#include <QFont>
#include <QDebug>
#include <QDir>
#include <QTextDocument>
#include <QTextCharFormat>
#include <QTextCursor>
#include <QBrush>

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
                           QLibraryInfo::path(QLibraryInfo::TranslationsPath))) {
        qApp->installTranslator(qtTranslator);
    }

    // Skribisto translation

    if (!skribistoTranslator->isEmpty()) {
        qApp->removeTranslator(skribistoTranslator);
    }

    // qDebug() << "findTranslationDir" << findTranslationDir();

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
        skrdata->errorHub()->addWarning(tr("No translation found."));
    }


    return translationMap;
}

// ------------------------------------------------------

QString SKRRootItem::getOnlyLanguageFromLocale(const QString& na_me) const
{
    return na_me.split("_").takeFirst();
}

// ------------------------------------------------------

QString SKRRootItem::getNativeCountryNameFromLocale(const QString& name) const {
    QLocale locale(name);

    return locale.nativeCountryName();
}

// ------------------------------------------------------

QString SKRRootItem::getNativeLanguageNameFromLocale(const QString& name) const {
    QLocale locale(name);

    return locale.nativeLanguageName();
}

// ------------------------------------------------------

bool SKRRootItem::createPath(const QString &path){
    QDir dir;
    return dir.mkpath(path);
}

// ------------------------------------------------------

QString SKRRootItem::getWritableAddonsPathsListDir() const {
    QStringList list = PLMUtils::Dir::writableAddonsPathsList();
    QString     dir  = "";

    if (!list.isEmpty()) {
        dir = list.first();
    }
    return dir;
}

// ------------------------------------------------------

QString SKRRootItem::cleanUpHtml(const QString& html)
{
    //    QString text = html;
    qDebug() << "§§§§§§§§§§§§§§§§§§§§§§§§";
    qDebug() << "pre" << html;

    QTextDocument doc;

    doc.setHtml(html);



    QString text = doc.toHtml();

    QStringList styleToRemoveList;
    styleToRemoveList << "font-family";
    styleToRemoveList << "font-size";
//    styleToRemoveList << "font-style";
    styleToRemoveList << "margin-left";
    styleToRemoveList << "margin-right";
    styleToRemoveList << "margin-top";
    styleToRemoveList << "margin-bottom";
    styleToRemoveList << "-qt-block-indent";
    styleToRemoveList << "-qt-user-state";
    styleToRemoveList << "text-indent";
    styleToRemoveList << "color";

    for (const QString& style : qAsConst(styleToRemoveList)) text.remove(QRegularExpression(style + ":.*?;"));
    text.remove(QRegularExpression("<h[0-9].*?>"));
    text.remove(QRegularExpression("<h[0-9]>"));
    return text;



    //qDebug() << "post doc" << text;

//    QXmlStreamReader xml(html);
//    QString finalHtml;
//    QXmlStreamWriter finalXml(&finalHtml);
//    //finalXml.setAutoFormatting(true);
//    QString parentElementName;
//    QStringList nameList;
//    nameList << "body" << "html" << "b" << "span" << "i" << "p" << "u" << "s"<< "ul" << "ol" << "li";

//    finalXml.writeStartDocument();
//    finalXml.writeDTD("<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" \"http://www.w3.org/TR/REC-html40/strict.dtd\">");


//    while (!xml.atEnd()) {
//        xml.readNext();

//        if(xml.tokenType() == QXmlStreamReader::StartElement){
//            parentElementName = xml.name().toString();
//        }

//        if(nameList.contains(xml.name()) && xml.tokenType() == QXmlStreamReader::StartElement){
//            finalXml.writeStartElement(xml.name().toString());

////            if(parentElementName == "meta"){
////                if(!xml.attributes().value("charset").isEmpty()){
////                    QXmlStreamAttributes attributes;
////                    attributes.append("charset", xml.attributes().value("charset").toString());
////                    finalXml.writeAttributes(attributes);
////                }
////                if(!xml.attributes().value("content").isEmpty()){
////                    QXmlStreamAttributes attributes;
////                    attributes.append("content", xml.attributes().value("content").toString());
////                    finalXml.writeAttributes(attributes);
////                }
////            }
//                        //qDebug() << "token" << xml.tokenString();
//                        //qDebug() << "name" << xml.name();
//            //qDebug() << "style" << xml.attributes().value("style");
//            if(!xml.attributes().value("style").isEmpty()){
//                QXmlStreamAttributes attributes;
//                attributes.append("style", trimStyle(xml.attributes().value("style").toString()));
//                finalXml.writeAttributes(attributes);

////                if(!xml.attributes().value("type").isEmpty()){
////                    QXmlStreamAttributes attributes;
////                    attributes.append("type", xml.attributes().value("type").toString());
////                    finalXml.writeAttributes(attributes);
////                }
//            }

//        }

//        else if(parentElementName != "html" && parentElementName != "style" && xml.tokenType() == QXmlStreamReader::Characters){

////            QString charString = xml.text().toString();
////            if(charString.contains("\t")){
////                qDebug() << "TAB HERE";
////            }


//            finalXml.writeCharacters(xml.text().toString());
//            //            qDebug() << "token" << xml.tokenString();
//            //            qDebug() << "name" << xml.name();
//            qDebug() << "parentElementName" << parentElementName;
//            qDebug() << "text" << xml.text();

//        }

//        else if(nameList.contains(xml.name()) && xml.tokenType() == QXmlStreamReader::EndElement){
//            finalXml.writeEndElement();
//        }
////                qDebug() << "token" << xml.tokenString();
////                qDebug() << "name" << xml.name();
//        //        qDebug() << "att" << xml.attributes().value("");
//        //        qDebug() << "text" << xml.text();

//        if (xml.hasError()) {
//        qDebug() << "errors" << xml.errorString() << xml.lineNumber();
//    }
//    }

//    finalXml.writeEndDocument();

//    qDebug() << "finalHtml" << finalHtml;

    return text;
}


// ------------------------------------------------------
QString SKRRootItem::trimStyle(const QString &styleValue){

    QString final;

    QStringList styleList = styleValue.split(";", Qt::SkipEmptyParts);

    QStringList styleToKeep;
    styleToKeep << "text-decoration"<< "font-style" << "font-weight";

    QMutableListIterator<QString> i(styleList);
    while (i.hasNext()) {
        QString val = i.next();
        i.setValue(val.remove("\\\"").trimmed());
    }

    QHash<QString, QString> hash;


    QStringListIterator iterator(styleList);
    while (iterator.hasNext()){

        QStringList splitted = iterator.next().split(":", Qt::KeepEmptyParts);
        if(styleToKeep.contains(splitted.at(0).trimmed())){
            if(splitted.count() == 2){
                final.append(splitted.at(0).trimmed() + ":" + splitted.at(1).trimmed() + "; ");
            }
        }
    }

    //qDebug() << "final" << final;
    return final;

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

QString SKRRootItem::getTempPath() const {
    return QDir::tempPath();
}

QStringList SKRRootItem::getDictFoldersFromGitHubTree(const QString& treeFile) const {
    QStringList list;

    QFile file(treeFile);

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return list;
    }

    QByteArray line = file.readAll();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << treeFile <<
                    "result :" << jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return list;
    }

    QJsonObject jsonObject = jsonDoc.object();
    QJsonArray  treeArray  = jsonObject.value("tree").toArray();

    QJsonArray::const_iterator i;

    QRegularExpression re("^dictionaries/.*?[^/]$");

    for (i = treeArray.constBegin(); i != treeArray.constEnd(); ++i) {
        QString path                  = i->toObject().value("path").toString();
        QRegularExpressionMatch match = re.match(path);

        if (match.hasMatch()) {
            list.append(path.split("/").takeAt(1));
        }
    }
    list.removeDuplicates();

    return list;
}

void SKRRootItem::removeFile(const QString& fileName) {
    QFile file(fileName);

    file.remove();
}

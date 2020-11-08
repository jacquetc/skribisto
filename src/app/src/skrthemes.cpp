#include "skrthemes.h"
#include "plmutils.h"

#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>

SKRThemes::SKRThemes(QObject *parent) : QObject(parent), m_currentTheme("")
{
    populate();
}

// ----------------------------------------------------------

void SKRThemes::populate()
{
    m_fileByThemeNameHash.clear();
    m_isEditableByThemeNameHash.clear();

    QStringList paths = PLMUtils::Dir::addonsPathsList();

    for (const QString& path : paths) {
        QString themePath = path + "/themes";


        QDir dir(themePath);
        QStringList nameFilters;

        nameFilters << "*.json";
        QFileInfoList entries = dir.entryInfoList(nameFilters);

        for (const QFileInfo& entryInfo : entries) {
            QFileInfo fileInfo(entryInfo);

            QFile file(fileInfo.canonicalFilePath());

            if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
                continue;
            }

            QByteArray line = file.readAll();

            QJsonParseError jsonError;
            QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

            if (jsonError.error != QJsonParseError::NoError) {
                qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() <<
                "error :" << jsonError.errorString();
            }

            if (jsonDoc.isNull()) {
                qDebug() << "jsonDoc.isNull";
                continue;
            }

            QJsonObject jsonObject = jsonDoc.object();
            QString     themeName  = jsonObject.value("themeName").toString(
                "Missing theme name");

            m_fileByThemeNameHash.insert(themeName, fileInfo.absoluteFilePath());

            bool isThemeEditable = jsonObject.value("isEditable").toBool(false);

            if (isThemeEditable && !fileInfo.isWritable()) {
                isThemeEditable = false;
            }


            m_isEditableByThemeNameHash.insert(themeName, isThemeEditable);
        }
    }
}

// ---------------------------------------------------------------

QStringList SKRThemes::getThemeList() const
{
    return m_fileByThemeNameHash.keys();
}

// ---------------------------------------------------------------

QString SKRThemes::defaultTheme() const
{
    return "Default Light";
}

// ---------------------------------------------------------------

bool SKRThemes::isThemeEditable(const QString& themeName) const
{
    return m_isEditableByThemeNameHash.value(themeName, false);
}

// ---------------------------------------------------------------

bool SKRThemes::doesThemeExist(const QString& themeName) const
{
    return m_isEditableByThemeNameHash.contains(themeName);
}

// ---------------------------------------------------------------

QString SKRThemes::currentTheme()
{
    if (m_currentTheme == "") {
        m_currentTheme = this->defaultTheme();
    }

    return m_currentTheme;
}

void SKRThemes::setCurrentTheme(const QString& themeName)
{
    m_currentTheme = themeName;

    applyTheme(themeName);

    emit currentThemeChanged(m_currentTheme);
}

// ---------------------------------------------------------------

void SKRThemes::applyTheme(const QString& themeName)
{
    QString theme = themeName;

    if (!m_fileByThemeNameHash.contains(theme)) {
        theme = defaultTheme();
    }

    QFile file(m_fileByThemeNameHash.value(theme));

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return;
    }

    QByteArray line = file.readAll();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << m_fileByThemeNameHash.value(theme) <<
        "error :" << jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return;
    }

    QJsonObject rootJsonObject = jsonDoc.object();

    QJsonObject normalObject = rootJsonObject.value("normal").toObject();

    m_mainTextAreaBackground = normalObject.value("mainTextAreaBackground").toString(
        "").toLower();
    m_mainTextAreaForeground =
        normalObject.value("mainTextAreaForeground").toString("").toLower();
    m_secondaryTextAreaBackground =
        normalObject.value("secondaryTextAreaBackground").toString("").toLower();
    m_secondaryTextAreaForeground =
        normalObject.value("secondaryTextAreaForeground").toString("").toLower();
    m_pageBackground =
        normalObject.value("pageBackground").toString("").toLower();
    m_buttonBackground =
        normalObject.value("buttonBackground").toString("").toLower();
    m_buttonForeground =
        normalObject.value("buttonForeground").toString("").toLower();
    m_buttonIcon =
        normalObject.value("buttonIcon").toString("").toLower();
    m_accent     =  normalObject.value("accent").toString("").toLower();
    m_spellcheck =
        normalObject.value("spellcheck").toString("").toLower();
    m_toolBarBackground =
        normalObject.value("toolBarBackground").toString("").toLower();
    m_divider        =  normalObject.value("divider").toString("").toLower();
    m_menuBackground =
        normalObject.value("menuBackground").toString("").toLower();

    QJsonObject distractionFreeObject =
        rootJsonObject.value("distractionFree").toObject();

    m_distractionFree_mainTextAreaBackground = distractionFreeObject.value(
        "mainTextAreaBackground").toString("").toLower();
    m_distractionFree_mainTextAreaForeground =  distractionFreeObject.value(
        "mainTextAreaForeground").toString("").toLower();
    m_distractionFree_secondaryTextAreaBackground =  distractionFreeObject.value(
        "secondaryTextAreaBackground").toString("").toLower();
    m_distractionFree_secondaryTextAreaForeground =  distractionFreeObject.value(
        "secondaryTextAreaForeground").toString("").toLower();
    m_distractionFree_pageBackground =  distractionFreeObject.value(
        "pageBackground").toString("").toLower();
    m_distractionFree_buttonBackground =  distractionFreeObject.value(
        "buttonBackground").toString("").toLower();
    m_distractionFree_buttonForeground =  distractionFreeObject.value(
        "buttonForeground").toString("").toLower();
    m_distractionFree_buttonIcon =  distractionFreeObject.value(
        "buttonIcon").toString("").toLower();
    m_distractionFree_accent =
        distractionFreeObject.value("accent").toString("").toLower();
    m_distractionFree_spellcheck =  distractionFreeObject.value(
        "spellcheck").toString("").toLower();
    m_distractionFree_toolBarBackground =  distractionFreeObject.value(
        "toolBarBackground").toString("").toLower();
    m_distractionFree_divider =
        distractionFreeObject.value("divider").toString("").toLower();
    m_distractionFree_menuBackground =  distractionFreeObject.value(
        "menuBackground").toString("").toLower();

    emit colorsChanged();
}

// ---------------------------------------------------------------


void SKRThemes::saveTheme(const QString& themeName) {
    QString theme = themeName;

    if (!m_fileByThemeNameHash.contains(theme)) {
        theme = defaultTheme();
    }


    QFileInfo fileInfo(m_fileByThemeNameHash.value(theme));
    QFile     file(m_fileByThemeNameHash.value(theme));

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return;
    }


    QByteArray line = file.readAll();

    file.close();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "error :" <<
        jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return;
    }

    // check editable

    QJsonObject rootJsonObject = jsonDoc.object();
    bool isThemeEditable       = rootJsonObject.value("isEditable").toBool(false);

    if (isThemeEditable && !fileInfo.isWritable()) {
        isThemeEditable = false;
    }

    if (!isThemeEditable) {
        qWarning() << themeName << "isn't editable";
    }


    // write properties

    QJsonObject normalObject = rootJsonObject.value("normal").toObject();

    normalObject.insert("mainTextAreaBackground",      m_mainTextAreaBackground);
    normalObject.insert("mainTextAreaForeground",      m_mainTextAreaForeground);
    normalObject.insert("secondaryTextAreaBackground", m_secondaryTextAreaBackground);
    normalObject.insert("secondaryTextAreaForeground", m_secondaryTextAreaForeground);
    normalObject.insert("pageBackground",              m_pageBackground);
    normalObject.insert("buttonBackground",            m_buttonBackground);
    normalObject.insert("buttonForeground",            m_buttonForeground);
    normalObject.insert("buttonIcon",                  m_buttonIcon);
    normalObject.insert("accent",                      m_accent);
    normalObject.insert("spellcheck",                  m_spellcheck);
    normalObject.insert("toolBarBackground",           m_toolBarBackground);
    normalObject.insert("divider",                     m_divider);
    normalObject.insert("menuBackground",              m_menuBackground);

    rootJsonObject.insert("normal", normalObject);

    QJsonObject distractionFreeObject =
        rootJsonObject.value("distractionFree").toObject();

    distractionFreeObject.insert("mainTextAreaBackground",
                                           m_distractionFree_mainTextAreaBackground);
    distractionFreeObject.insert("mainTextAreaForeground",
                                           m_distractionFree_mainTextAreaForeground);
    distractionFreeObject.insert("secondaryTextAreaBackground",
                                           m_distractionFree_secondaryTextAreaBackground);
    distractionFreeObject.insert("secondaryTextAreaForeground",
                                           m_distractionFree_secondaryTextAreaForeground);
    distractionFreeObject.insert("pageBackground",
                                           m_distractionFree_pageBackground);
    distractionFreeObject.insert("buttonBackground",
                                           m_distractionFree_buttonBackground);
    distractionFreeObject.insert("buttonForeground",
                                           m_distractionFree_buttonForeground);
    distractionFreeObject.insert("buttonIcon",
                                           m_distractionFree_buttonIcon);
    distractionFreeObject.insert("accent", m_distractionFree_accent);
    distractionFreeObject.insert("spellcheck",
                                 m_distractionFree_spellcheck);
    distractionFreeObject.insert("toolBarBackground",
                                 m_distractionFree_toolBarBackground);
    distractionFreeObject.insert("divider",
                                 m_distractionFree_divider);
    distractionFreeObject.insert("menuBackground",
                                 m_distractionFree_menuBackground);

    rootJsonObject.insert("distractionFree", distractionFreeObject);


    // write back to file
    jsonDoc.setObject(rootJsonObject);

    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QIODevice::Truncate)) {
        return;
    }


    file.write(jsonDoc.toJson());
    file.close();
}

// ----------------------------------------------------------------------

bool SKRThemes::duplicate(const QString& themeName, const QString& newThemeName) {
    if (m_fileByThemeNameHash.contains(newThemeName)) {
        return false;
    }

    if (PLMUtils::Dir::writableAddonsPathsList().isEmpty()) {
        return false;
    }


    // read :


    QFileInfo fileInfo(m_fileByThemeNameHash.value(themeName));
    QFile     file(m_fileByThemeNameHash.value(themeName));

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return false;
    }


    QByteArray lines = file.readAll();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(lines, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "error :" <<
        jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return false;
    }

    // modify name

    QJsonObject jsonObject = jsonDoc.object();

    jsonObject.insert("themeName",  QJsonValue::fromVariant(newThemeName));
    jsonObject.insert("isEditable", QJsonValue::fromVariant(true));


    // write
    jsonDoc.setObject(jsonObject);

    QString newPath = PLMUtils::Dir::writableAddonsPathsList().first();
    QDir    dir(newPath + "/themes/");

    dir.mkpath(newPath + "/themes/");

    QFileInfo newFileInfo(newPath + "/themes/" + newThemeName + ".json");

    if (newFileInfo.exists()) {
        return false;
    }


    QFile newFile(newFileInfo.absoluteFilePath());

    if (!newFile.open(QIODevice::WriteOnly | QIODevice::Text | QFile::Truncate)) {
        return false;
    }

    if (newFile.write(jsonDoc.toJson()) == -1) {
        return false;
    }

    // add
    m_fileByThemeNameHash.insert(newThemeName, newFileInfo.absoluteFilePath());
    m_isEditableByThemeNameHash.insert(newThemeName, true);

    return true;
}

// ----------------------------------------------------------------------

bool SKRThemes::remove(const QString& themeName) {
    if (!m_fileByThemeNameHash.contains(themeName)) {
        return false;
    }

    if (currentTheme() == themeName) {
        setCurrentTheme(defaultTheme());
    }

    QFile file(m_fileByThemeNameHash.value(themeName));
    bool  result = file.remove();

    if (result) {
        m_fileByThemeNameHash.remove(themeName);
        m_isEditableByThemeNameHash.remove(themeName);
    }

    return result;
}

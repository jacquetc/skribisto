#include "skrthemes.h"
#include "plmutils.h"

#include <QDir>
#include <QJsonDocument>
#include <QJsonObject>
#include <QSettings>

SKRThemes::SKRThemes(QObject *parent) : QObject(parent), m_selectedTheme("")
{
    populate();

    QSettings settings;

    settings.beginGroup("theme");
    setCurrentLightTheme(settings.value("lightThemeName", defaultLightTheme()).toString());
    setCurrentDarkTheme(settings.value("darkThemeName", defaultDarkTheme()).toString());
    setCurrentDistractionFreeTheme(settings.value("distractionFreeThemeName",
                                                  defaultDistractionFreeTheme()).toString());

    QString colorMode = settings.value("currentColorMode", "light").toString();

    if (colorMode == "light") {
        setCurrentColorMode(ColorMode::Light);
    }
    else if (colorMode == "dark") {
        setCurrentColorMode(ColorMode::Dark);
    }
    else if (colorMode == "distractionFree") {
        setCurrentColorMode(ColorMode::DistractionFree);
    }

    settings.endGroup();
}

// ----------------------------------------------------------

void SKRThemes::populate()
{
    m_fileByThemeNameHash.clear();
    m_isEditableByThemeNameHash.clear();

    QStringList paths = PLMUtils::Dir::addonsPathsList();

    for (const QString& path : qAsConst(paths)) {
        QString themePath = path + "/themes";


        QDir dir(themePath);
        QStringList nameFilters;

        nameFilters << "*.json";
        QFileInfoList entries = dir.entryInfoList(nameFilters);

        for (const QFileInfo& entryInfo : qAsConst(entries)) {
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
                    "result :" << jsonError.errorString();
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

QString SKRThemes::defaultLightTheme() const
{
    return "Default Light";
}

// ---------------------------------------------------------------

QString SKRThemes::defaultDarkTheme() const
{
    return "Default Dark";
}

// ---------------------------------------------------------------

QString SKRThemes::defaultDistractionFreeTheme() const
{
    return "Default Dark";
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

QString SKRThemes::selectedTheme()
{
    if (m_selectedTheme == "") {
        m_selectedTheme = this->defaultLightTheme();
    }

    return m_selectedTheme;
}

void SKRThemes::setSelectedTheme(const QString& themeName)
{
    m_selectedTheme = themeName;

    applyTheme(themeName, true);
    emit selectedThemeChanged(m_selectedTheme);
}

// ---------------------------------------------------------------

void SKRThemes::applyTheme(const QString& themeName, bool selected)
{
    QString theme = themeName;

    if (!m_fileByThemeNameHash.contains(theme)) {
        theme = defaultLightTheme();
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
            "result :" << jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return;
    }

    QJsonObject rootJsonObject = jsonDoc.object();

    QJsonObject normalObject = rootJsonObject.value("colors").toObject();

    if (selected) {
        m_selectedColorsMap.clear();

        for (const QString& key : normalObject.keys()) {
            m_selectedColorsMap.insert(key, normalObject.value(key).toString("").toLower());
        }
        emit selectedColorsMapChanged(m_selectedColorsMap);
    }
    else {
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
        m_buttonIconDisabled =
            normalObject.value("buttonIconDisabled").toString("").toLower();
        m_accent     =  normalObject.value("accent").toString("").toLower();
        m_spellcheck =
            normalObject.value("spellcheck").toString("").toLower();
        m_toolBarBackground =
            normalObject.value("toolBarBackground").toString("").toLower();
        m_pageToolBarBackground =
            normalObject.value("pageToolBarBackground").toString("").toLower();
        m_divider        =  normalObject.value("divider").toString("").toLower();
        m_menuBackground =
            normalObject.value("menuBackground").toString("").toLower();
        m_listItemBackground =
            normalObject.value("listItemBackground").toString("").toLower();
        emit colorsChanged();
    }
}

// ------------------------------------------------------------------

QString SKRThemes::getCurrentDistractionFreeTheme() const
{
    return m_currentDistractionFreeTheme;
}

void SKRThemes::setCurrentDistractionFreeTheme(const QString& currentDistractionFreeTheme)
{
    if (doesThemeExist(currentDistractionFreeTheme)) {
        m_currentDistractionFreeTheme = currentDistractionFreeTheme;
    }
    else {
        m_currentDistractionFreeTheme = defaultDistractionFreeTheme();
    }

    QSettings settings;

    settings.beginGroup("theme");
    settings.setValue("distractionFreeThemeName", m_currentDistractionFreeTheme);
    settings.endGroup();


    if (m_currentColorMode == DistractionFree) {
        applyTheme(currentDistractionFreeTheme);
    }

    emit currentDistractionFreeThemeChanged(m_currentDistractionFreeTheme);
}

QString SKRThemes::getCurrentLightTheme() const
{
    return m_currentLightTheme;
}

void SKRThemes::setCurrentLightTheme(const QString& currentLightTheme)
{
    if (doesThemeExist(currentLightTheme)) {
        m_currentLightTheme = currentLightTheme;
    }
    else {
        m_currentLightTheme = defaultLightTheme();
    }

    QSettings settings;

    settings.beginGroup("theme");
    settings.setValue("lightThemeName", m_currentLightTheme);
    settings.endGroup();

    if (m_currentColorMode == Light) {
        applyTheme(currentLightTheme);
    }

    emit currentLightThemeChanged(m_currentLightTheme);
}

QString SKRThemes::getCurrentDarkTheme() const
{
    return m_currentDarkTheme;
}

void SKRThemes::setCurrentDarkTheme(const QString& currentDarkTheme)
{
    if (doesThemeExist(currentDarkTheme)) {
        m_currentDarkTheme = currentDarkTheme;
    }
    else {
        m_currentDarkTheme = defaultDarkTheme();
    }

    QSettings settings;

    settings.beginGroup("theme");
    settings.setValue("darkThemeName", m_currentDarkTheme);
    settings.endGroup();

    if (m_currentColorMode == Dark) {
        applyTheme(currentDarkTheme);
    }

    emit currentDarkThemeChanged(m_currentDarkTheme);
}

SKRThemes::ColorMode SKRThemes::getCurrentColorMode() const
{
    return m_currentColorMode;
}

void SKRThemes::setCurrentColorMode(SKRThemes::ColorMode colorMode)
{
    m_currentColorMode = colorMode;

    QString colorModeString;

    switch (colorMode) {
    case ColorMode::Light:
        colorModeString = "light";
        applyTheme(getCurrentLightTheme());
        break;

    case ColorMode::Dark:
        colorModeString = "dark";
        applyTheme(getCurrentDarkTheme());
        break;

    case ColorMode::DistractionFree:
        colorModeString = "distractionFree";
        applyTheme(getCurrentDistractionFreeTheme());
        break;
    }

    QSettings settings;

    settings.beginGroup("theme");
    settings.setValue("currentColorMode", colorModeString);
    settings.endGroup();

    emit currentColorModeChanged(colorMode);
}

// ---------------------------------------------------------------


void SKRThemes::saveTheme(const QString& themeName, const QVariantMap& colorMap) {
    QString theme = themeName;

    if (!m_fileByThemeNameHash.contains(theme)) {
        theme = defaultLightTheme();
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
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
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

    QJsonObject normalObject = rootJsonObject.value("colors").toObject();


    for (const QString& key : colorMap.keys()) {
        normalObject.insert(key, colorMap.value(key).toString().toLower());
    }


    rootJsonObject.insert("colors", normalObject);


    // write back to file
    jsonDoc.setObject(rootJsonObject);

    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QIODevice::Truncate)) {
        return;
    }


    file.write(jsonDoc.toJson());
    file.close();


    // update if current

    switch (m_currentColorMode) {
    case Light:

        if (getCurrentLightTheme() == themeName) {
            applyTheme(themeName);
        }
        break;

    case Dark:

        if (getCurrentDarkTheme() == themeName) {
            applyTheme(themeName);
        }
        break;

    case DistractionFree:

        if (getCurrentDistractionFreeTheme() == themeName) {
            applyTheme(themeName);
        }
        break;
    }
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
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
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

    if (selectedTheme() == themeName) {
        setSelectedTheme(defaultLightTheme());
    }

    QFile file(m_fileByThemeNameHash.value(themeName));
    bool  value = file.remove();

    if (value) {
        m_fileByThemeNameHash.remove(themeName);
        m_isEditableByThemeNameHash.remove(themeName);
    }

    return value;
}

// ----------------------------------------------------------------------

bool SKRThemes::rename(const QString& themeName, const QString& newThemeName)
{
    if (m_fileByThemeNameHash.contains(newThemeName)) {
        return false;
    }


    // read :


    QFileInfo fileInfo(m_fileByThemeNameHash.value(themeName));
    QFile     file(m_fileByThemeNameHash.value(themeName));

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return false;
    }

    QByteArray lines = file.readAll();

    file.close();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(lines, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
            jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return false;
    }

    // modify name

    QJsonObject jsonObject = jsonDoc.object();

    jsonObject.insert("themeName", QJsonValue::fromVariant(newThemeName));


    // write
    jsonDoc.setObject(jsonObject);


    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QFile::Truncate)) {
        return false;
    }

    if (file.write(jsonDoc.toJson()) == -1) {
        return false;
    }

    file.close();
    QString   newFilePath = fileInfo.canonicalPath() + "/" + newThemeName + ".json";
    bool      success     = file.rename(newFilePath);
    QFileInfo newFileInfo(newFilePath);

    if (!success) {
        return false;
    }
    m_fileByThemeNameHash.remove(themeName);
    m_fileByThemeNameHash.insert(newThemeName, newFileInfo.absoluteFilePath());

    bool isEditable = m_isEditableByThemeNameHash.value(themeName);
    m_isEditableByThemeNameHash.remove(themeName);
    m_isEditableByThemeNameHash.insert(newThemeName, isEditable);


    return true;
}

// ----------------------------------------------------------------------

void SKRThemes::resetSelectedThemeColors()
{
    this->applyTheme(selectedTheme(), true);
}

#include "thememanager.h"
#include "QtWidgets/qcombobox.h"
#include "plmutils.h"

#include <QFileInfoList>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonParseError>
#include <QPainter>
#include <QPushButton>
#include <QSettings>
#include <QToolButton>
#include <QPointer>
#include <QTimer>
#include <QLayout>
#include <QMainWindow>

ThemeManager::ThemeManager(QObject *parent) : QObject{parent}, m_currentThemeType(ThemeManager::Light) {
    reloadThemes();

    restoreThemeState();
}

//----------------------------------------------

ThemeManager::~ThemeManager() {}
//----------------------------------------------

ThemeManager *ThemeManager::m_instance = nullptr;

const QString &ThemeManager::darkTheme() const { return m_darkTheme; }

void ThemeManager::setDarkTheme(const QString &newDarkTheme) {
  if (m_darkTheme != newDarkTheme) {
    m_darkTheme = newDarkTheme;

    if(currentThemeType() == ThemeType::Dark){
        this->applyTheme(m_darkTheme);
    }
    saveThemeState();
    emit darkThemeChanged(newDarkTheme);
  }
}

//----------------------------------------------

const QString &ThemeManager::lightTheme() const { return m_lightTheme; }

void ThemeManager::setLightTheme(const QString &newLightTheme) {
  if (m_lightTheme != newLightTheme) {
    m_lightTheme = newLightTheme;

    if(currentThemeType() == ThemeType::Light){
        this->applyTheme(m_lightTheme);
    }
    saveThemeState();
    emit lightThemeChanged(newLightTheme);
  }
}
//----------------------------------------------

const ThemeManager::ThemeType &ThemeManager::currentThemeType() const {
  return m_currentThemeType;
}

void ThemeManager::setCurrentThemeType(
    const ThemeManager::ThemeType &newCurrentThemeType) {
  if (m_currentThemeType != newCurrentThemeType) {
    m_currentThemeType = newCurrentThemeType;

    if(m_currentThemeType == ThemeType::Light){
        this->applyTheme(this->lightTheme());
    }
    if(m_currentThemeType == ThemeType::Dark){
        this->applyTheme(this->darkTheme());
    }

    saveThemeState();
    emit currentThemeTypeChanged(newCurrentThemeType);
  }
}
//----------------------------------------------

const QMap<QString, QString> &ThemeManager::darkThemeWithLocationMap() const {
  return m_darkThemeWithLocationMap;
}

const QMap<QString, QString> &ThemeManager::lightThemeWithLocationMap() const {
  return m_lightThemeWithLocationMap;
}

const QHash<QString, bool> &ThemeManager::themeWithEditableHash() const
{
    return m_themeWithEditableHash;
}

//----------------------------------------------

void ThemeManager::saveThemeState()
{
    QSettings settings;
    settings.beginGroup("theme");

    settings.setValue("light_theme", this->lightTheme());
    settings.setValue("dark_theme", this->darkTheme());
    settings.setValue("current_type", this->currentThemeType());

    settings.endGroup();
}

//----------------------------------------------

void ThemeManager::restoreThemeState()
{
    QSettings settings;
    settings.beginGroup("theme");

    QString lightTheme = settings.value("light_theme", "Default Light").toString();
    QString darkTheme = settings.value("dark_theme", "Default Dark").toString();
    ThemeManager::ThemeType themeType = static_cast<ThemeManager::ThemeType>(settings.value("current_type", 0).toInt());

    this->setLightTheme(lightTheme);
    this->setDarkTheme(darkTheme);
    this->setCurrentThemeType(themeType);

    settings.endGroup();

}


//----------------------------------------------

ThemeManager::ThemeType ThemeManager::themeType(const QString &themeName) const
{
    if(m_darkThemeWithLocationMap.contains(themeName)){
        return ThemeType::Dark;
    }
    else{
        return ThemeType::Light;

    }
}

QPalette ThemeManager::toPalette(const QMap<QString, QString> colorMap) const
{

    QPalette palette;

    QBrush window = QBrush(QColor(colorMap.value("window").toLower()));
    QBrush windowText = QBrush(QColor(colorMap.value("windowText").toLower()));
    QBrush base = QBrush(QColor(colorMap.value("base").toLower()));
    QBrush text = QBrush(QColor(colorMap.value("text").toLower()));
    QBrush brightText = QBrush(QColor(colorMap.value("brightText").toLower()));
    QBrush highlight = QBrush(QColor(colorMap.value("highlight").toLower()));
    QBrush highlightedText = QBrush(QColor(colorMap.value("highlightedText").toLower()));
    QBrush button = QBrush(QColor(colorMap.value("button").toLower()));
    QBrush buttonText = QBrush(QColor(colorMap.value("buttonText").toLower()));
    QBrush light = QBrush(QColor(colorMap.value("light").toLower()));
    QBrush mid = QBrush(QColor(colorMap.value("mid").toLower()));
    QBrush dark = QBrush(QColor(colorMap.value("dark").toLower()));
    QBrush placeholderText = QBrush(QColor(colorMap.value("placeholderText").toLower()));
    QBrush toolTipText = QBrush(QColor(colorMap.value("toolTipText").toLower()));
    QBrush toolTipBase = QBrush(QColor(colorMap.value("toolTipBase").toLower()));
//        QBrush link = QBrush(QColor(normalObject.value("link").toLower()));
//        QBrush visitedLink = QBrush(QColor(normalObject.value("visitedLink").toLower()));
    QBrush shadow = QBrush(QColor(colorMap.value("shadow").toLower()));

    palette.setColorGroup(QPalette::ColorGroup::All, windowText,
                          button, light, dark, mid, text, brightText, base, window);

    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::Highlight, highlight);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::HighlightedText, highlightedText);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::PlaceholderText, placeholderText);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::ButtonText, buttonText);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::ToolTipText, toolTipText);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::ToolTipBase, toolTipBase);
//        palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::Link, link);
//        palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::LinkVisited, visitedLink);
    palette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::Shadow, shadow);

    //deactivated:
    palette.setBrush(QPalette::ColorGroup::Disabled, QPalette::ColorRole::Text, placeholderText);
    palette.setBrush(QPalette::ColorGroup::Disabled, QPalette::ColorRole::WindowText, placeholderText);
    palette.setBrush(QPalette::ColorGroup::Disabled, QPalette::ColorRole::ButtonText, placeholderText);
    palette.setBrush(QPalette::ColorGroup::Disabled, QPalette::ColorRole::HighlightedText, placeholderText);


    return palette;
}
//----------------------------------------------

QJsonDocument ThemeManager::loadThemeJson(const QString &themeName) const
{
    QString theme = themeName;

    if (!m_darkThemeWithLocationMap.contains(theme) && !m_lightThemeWithLocationMap.contains(theme) ) {
        theme = defaultLightTheme();
    }

    QMap<QString, QString> themeMap;
    if(m_darkThemeWithLocationMap.contains(theme)){
        themeMap = m_darkThemeWithLocationMap;
    }
    else{
        themeMap = m_lightThemeWithLocationMap;
    }

    QFile file(themeMap.value(theme));

    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return QJsonDocument();
    }

    QByteArray line = file.readAll();

    QJsonParseError jsonError;
    QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

    if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << themeMap.value(theme) <<
            "result :" << jsonError.errorString();
    }

    if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        return QJsonDocument();
    }
    return jsonDoc;
}


//----------------------------------------------

QMap<QString, QString> ThemeManager::getColorMap(const QString &themeName) const
{
    QJsonDocument jsonDoc = this->loadThemeJson(themeName);

    if(jsonDoc.isNull()){
        return  QMap<QString, QString>();
    }

    QJsonObject rootJsonObject = jsonDoc.object();

    QJsonObject normalObject = rootJsonObject.value("colors").toObject();

    QMap<QString, QString> colorMap;

    colorMap.insert("window", normalObject.value("window").toString("").toLower());
    colorMap.insert("windowText", normalObject.value("windowText").toString("").toLower());
    colorMap.insert("base", normalObject.value("base").toString("").toLower());
    colorMap.insert("text", normalObject.value("text").toString("").toLower());
    colorMap.insert("brightText", normalObject.value("brightText").toString("").toLower());
    colorMap.insert("highlight", normalObject.value("highlight").toString("").toLower());
    colorMap.insert("highlightedText", normalObject.value("highlightedText").toString("").toLower());
    colorMap.insert("button", normalObject.value("button").toString("").toLower());
    colorMap.insert("buttonText", normalObject.value("buttonText").toString("").toLower());
    colorMap.insert("light", normalObject.value("light").toString("").toLower());
    colorMap.insert("mid", normalObject.value("mid").toString("").toLower());
    colorMap.insert("dark", normalObject.value("dark").toString("").toLower());
    colorMap.insert("placeholderText", normalObject.value("placeholderText").toString("").toLower());
    colorMap.insert("toolTipText", normalObject.value("toolTipText").toString("").toLower());
    colorMap.insert("toolTipBase", normalObject.value("toolTipBase").toString("").toLower());
    colorMap.insert("shadow", normalObject.value("shadow").toString("").toLower());


    return colorMap;
}

//----------------------------------------------

QString ThemeManager::themeInfo(const QString &themeName) const
{

    QJsonDocument jsonDoc = this->loadThemeJson(themeName);

    if(jsonDoc.isNull()){
        return  "";
    }


    QJsonObject rootJsonObject = jsonDoc.object();
    return rootJsonObject.value("info").toString("");
}

//----------------------------------------------

void ThemeManager::saveTheme(const QString &themeName, const QString &location, const ThemeType &themeType, const QMap<QString, QString> colorMap) const
{

//    QFileInfo fileInfo(location);

//    if(!QDir(fileInfo.path()).exists() || !fileInfo.isWritable()  ){
//        qWarning() << "theme can't save at" << location;
//    }
    QFile     file(location);


    QJsonDocument   jsonDoc;
    QJsonObject rootJsonObject = jsonDoc.object();

    // write properties
    rootJsonObject.insert("themeName", themeName);

    QString themeTypeName;
    switch (themeType) {
    case ThemeType::Light:
        themeTypeName = "light";
        break;
    case ThemeType::Dark:
        themeTypeName = "dark";
        break;
    default:
        themeTypeName = "light";
        break;
    }


    rootJsonObject.insert("themeType", themeTypeName);
    rootJsonObject.insert("isEditable", true);
    rootJsonObject.insert("isForDesktop", true);
    rootJsonObject.insert("info", "");


    QJsonObject colorObject;
    QMap<QString, QString>::const_iterator i = colorMap.constBegin();
    while (i != colorMap.constEnd()) {
        colorObject.insert(i.key(), i.value().toLower());
        ++i;
    }


    rootJsonObject.insert("colors", colorObject);


    // write back to file
    jsonDoc.setObject(rootJsonObject);

    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QIODevice::Truncate)) {
        return;
    }


    file.write(jsonDoc.toJson());
    file.close();


}

//----------------------------------------------


void ThemeManager::scanChildrenAndAddWidgetsHoldingIcons(QWidget *parentWidget)
{
    auto list = parentWidget->findChildren<QPushButton *>();
    for(auto button : list){
        if(!m_iconWidgetList.contains(button)){
            m_iconWidgetList.append(button);
        }
    }

    auto list2 = parentWidget->findChildren<QToolButton *>();
    for(auto button : list2){
        if(!m_iconWidgetList.contains(button)){
            m_iconWidgetList.append(button);
        }
    }

    auto list3 = parentWidget->findChildren<QAction *>();
    for(auto action : list3){
        if(!m_iconWidgetList.contains(action)){
            m_iconWidgetList.append(action);
        }

    }

    updateAllIconColors();
}

QIcon ThemeManager::adaptIcon(const QIcon &icon) const

{   QSize size(32,32);
    QPixmap pix = icon.pixmap(size);
    QPainter painter(&pix);

    painter.setCompositionMode(QPainter::CompositionMode_SourceIn);
    painter.fillRect(0, 0, size.width(), size.height(), m_currentPalette.buttonText());

    return QIcon(pix);

}

//----------------------------------------------


ThemeManager::ThemeType ThemeManager::switchThemeType()
{
    if(this->currentThemeType() == ThemeManager::Light){
        this->setCurrentThemeType(ThemeManager::Dark);
    }
    else{
        this->setCurrentThemeType(ThemeManager::Light);
    }

    return this->currentThemeType();
}

//----------------------------------------------


void ThemeManager::addIconWidget(QWidget *widgetWithIcon)
{
    m_iconWidgetList.append(widgetWithIcon);
}

//----------------------------------------------

void ThemeManager::updateAllIconColors()
{

    QMutableListIterator<QPointer<QObject>> i(m_iconWidgetList);
    while (i.hasNext()) {
        if (i.next().isNull())
            i.remove();
    }


    for(QObject *object : m_iconWidgetList){
        QPushButton *button = static_cast<QPushButton *>(object);
        if(QString(object->metaObject()->className()) == "QPushButton"){
            QIcon icon = button->icon();
            if(!icon.isNull()){
                button->setIcon(this->adaptIcon(icon));
            }
            continue;
        }
        QToolButton *toolButton = static_cast<QToolButton *>(object);
        if(QString(toolButton->metaObject()->className()) == "QToolButton"){
            if(nullptr == toolButton->defaultAction()){
                QIcon icon = toolButton->icon();
                if(!icon.isNull()){
                    toolButton->setIcon(this->adaptIcon(icon));
                }
            }
            continue;
        }
        if(QAction *action = static_cast<QAction *>(object)){
                QIcon icon = action->icon();
                if(!icon.isNull()){
                    action->setIcon(this->adaptIcon(icon));
                }

                continue;
        }


    }
}

//----------------------------------------------

QString ThemeManager::getWritablePathForTheme() const{

    QStringList paths = PLMUtils::Dir::writableAddonsPathsList();

    for (const QString &path : qAsConst(paths)) {
      QString themePath = path + "/themes";
      QFileInfo fileInfo(themePath);      
      if(!fileInfo.exists()){
          QDir dir(themePath);
          dir.mkpath(dir.path());
      }
      if(fileInfo.isWritable() && fileInfo.isDir()){
          return themePath;
      }

    }

    return QString();
}

//----------------------------------------------

void ThemeManager::reloadThemes() {
  QStringList paths = PLMUtils::Dir::addonsPathsList();

  for (const QString &path : qAsConst(paths)) {
    QString themePath = path + "/themes";
    //        qDebug() << "themePath :";
    //        qDebug() << themePath;

    QDir dir(themePath);
    QStringList nameFilters;

    nameFilters << "*.json";
    QFileInfoList entries = dir.entryInfoList(nameFilters);

    for (const QFileInfo &entryInfo : qAsConst(entries)) {
      QFileInfo fileInfo(entryInfo);

      QFile file(fileInfo.canonicalFilePath());
      //            qDebug() << "theme :";
      //            qDebug() << fileInfo.canonicalFilePath();
      if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        continue;
      }

      QByteArray line = file.readAll();

      QJsonParseError jsonError;
      QJsonDocument jsonDoc = QJsonDocument::fromJson(line, &jsonError);

      if (jsonError.error != QJsonParseError::NoError) {
        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath()
                 << "result :" << jsonError.errorString();
      }

      if (jsonDoc.isNull()) {
        qDebug() << "jsonDoc.isNull";
        continue;
      }

      QJsonObject jsonObject = jsonDoc.object();


      bool isForDesktop =
          jsonObject.value("isForDesktop").toBool(false);

      if(!isForDesktop){
          continue;
      }


      QString themeName =
          jsonObject.value("themeName").toString("Missing theme name");

      QString themeType =
          jsonObject.value("themeType").toString("Missing theme type");

      if(themeType == "dark"){
          m_darkThemeWithLocationMap.insert(themeName, fileInfo.absoluteFilePath());
      }
      else {
          m_lightThemeWithLocationMap.insert(themeName, fileInfo.absoluteFilePath());
      }


      bool isThemeEditable = jsonObject.value("isEditable").toBool(false);

      if (isThemeEditable && !fileInfo.isWritable()) {
        isThemeEditable = false;
      }

      m_themeWithEditableHash.insert(themeName, isThemeEditable);
    }
  }
}

//----------------------------------------------

void ThemeManager::reapplyCurrentTheme()
{
    this->applyTheme(m_currentThemeName);
}

//----------------------------------------------

void ThemeManager::applyTheme(const QString &themeName)
{
        auto colorMap = this->getColorMap(themeName);
        if(!colorMap.isEmpty()){
            m_currentPalette = this->toPalette(colorMap);
            m_currentThemeName = themeName;
            qApp->setPalette(m_currentPalette);
            updateAllIconColors();

            for(QWidget *widget : qApp->allWidgets()){
                if(nullptr == widget){
                    continue;
                }

                if(widget->property("themeZone") == "middleZone"){
                    QPalette palette = createMiddlePalette(m_currentPalette);
                    widget->setPalette(palette);

                    QList<QComboBox *> comboBoxList = widget->findChildren<QComboBox *>();
                    for(QComboBox *comboBox :comboBoxList){
                        comboBox->setPalette(palette);
                    }

                }

                if(widget->property("themeZone") == "sideZone"){
                    QPalette palette = createSidePalette(m_currentPalette);
                    widget->setPalette(palette);

                    QList<QComboBox *> comboBoxList = widget->findChildren<QComboBox *>();
                    for(QComboBox *comboBox :comboBoxList){
                        comboBox->setPalette(palette);
                    }
                }
            }
            }



}


//----------------------------------------------

QPalette ThemeManager::createSidePalette(const QPalette &palette) const
{
    QPalette sidePalette;
    sidePalette.setBrush(QPalette::ColorRole::Base, palette.window());

    return sidePalette.resolve(palette);
}

//----------------------------------------------

QPalette ThemeManager::createMiddlePalette(const QPalette &palette) const
{
    QPalette middlePalette;
    middlePalette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::Button, palette.base());
    middlePalette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::Window, palette.base());
    middlePalette.setBrush(QPalette::ColorGroup::All, QPalette::ColorRole::AlternateBase, palette.base());

    return middlePalette.resolve(palette);
}


// ----------------------------------------------------------------------

//bool ThemeManager::duplicate(const QString& themeName, const QString& newThemeName) {
//    if (m_fileByThemeNameHash.contains(newThemeName)) {
//        return false;
//    }

//    if (PLMUtils::Dir::writableAddonsPathsList().isEmpty()) {
//        return false;
//    }


//    // read :


//    QFileInfo fileInfo(m_fileByThemeNameHash.value(themeName));
//    QFile     file(m_fileByThemeNameHash.value(themeName));

//    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
//        return false;
//    }


//    QByteArray lines = file.readAll();

//    QJsonParseError jsonError;
//    QJsonDocument   jsonDoc = QJsonDocument::fromJson(lines, &jsonError);

//    if (jsonError.error != QJsonParseError::NoError) {
//        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
//            jsonError.errorString();
//    }

//    if (jsonDoc.isNull()) {
//        qDebug() << "jsonDoc.isNull";
//        return false;
//    }

//    // modify name

//    QJsonObject jsonObject = jsonDoc.object();

//    jsonObject.insert("themeName",  QJsonValue::fromVariant(newThemeName));
//    jsonObject.insert("isEditable", QJsonValue::fromVariant(true));


//    // write
//    jsonDoc.setObject(jsonObject);

//    QString newPath = PLMUtils::Dir::writableAddonsPathsList().first();
//    QDir    dir(newPath + "/themes/");

//    dir.mkpath(newPath + "/themes/");

//    QFileInfo newFileInfo(newPath + "/themes/" + newThemeName + ".json");

//    if (newFileInfo.exists()) {
//        return false;
//    }


//    QFile newFile(newFileInfo.absoluteFilePath());

//    if (!newFile.open(QIODevice::WriteOnly | QIODevice::Text | QFile::Truncate)) {
//        return false;
//    }

//    if (newFile.write(jsonDoc.toJson()) == -1) {
//        return false;
//    }

//    // add
//    m_fileByThemeNameHash.insert(newThemeName, newFileInfo.absoluteFilePath());
//    m_isEditableByThemeNameHash.insert(newThemeName, true);

//    return true;
//}

// ----------------------------------------------------------------------

//bool SKRThemes::rename(const QString& themeName, const QString& newThemeName)
//{
//    if (m_fileByThemeNameHash.contains(newThemeName)) {
//        return false;
//    }


//    // read :


//    QFileInfo fileInfo(m_fileByThemeNameHash.value(themeName));
//    QFile     file(m_fileByThemeNameHash.value(themeName));

//    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
//        return false;
//    }

//    QByteArray lines = file.readAll();

//    file.close();

//    QJsonParseError jsonError;
//    QJsonDocument   jsonDoc = QJsonDocument::fromJson(lines, &jsonError);

//    if (jsonError.error != QJsonParseError::NoError) {
//        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
//            jsonError.errorString();
//    }

//    if (jsonDoc.isNull()) {
//        qDebug() << "jsonDoc.isNull";
//        return false;
//    }

//    // modify name

//    QJsonObject jsonObject = jsonDoc.object();

//    jsonObject.insert("themeName", QJsonValue::fromVariant(newThemeName));


//    // write
//    jsonDoc.setObject(jsonObject);


//    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QFile::Truncate)) {
//        return false;
//    }

//    if (file.write(jsonDoc.toJson()) == -1) {
//        return false;
//    }

//    file.close();
//    QString   newFilePath = fileInfo.canonicalPath() + "/" + newThemeName + ".json";
//    bool      success     = file.rename(newFilePath);
//    QFileInfo newFileInfo(newFilePath);

//    if (!success) {
//        return false;
//    }
//    m_fileByThemeNameHash.remove(themeName);
//    m_fileByThemeNameHash.insert(newThemeName, newFileInfo.absoluteFilePath());

//    bool isEditable = m_isEditableByThemeNameHash.value(themeName);
//    m_isEditableByThemeNameHash.remove(themeName);
//    m_isEditableByThemeNameHash.insert(newThemeName, isEditable);


//    return true;
//}

//void SKRThemes::saveTheme(const QString& themeName, const QVariantMap& colorMap) {
//    QString theme = themeName;

//    if (!m_fileByThemeNameHash.contains(theme)) {
//        theme = defaultLightTheme();
//    }


//    QFileInfo fileInfo(m_fileByThemeNameHash.value(theme));
//    QFile     file(m_fileByThemeNameHash.value(theme));

//    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
//        return;
//    }


//    QByteArray line = file.readAll();

//    file.close();

//    QJsonParseError jsonError;
//    QJsonDocument   jsonDoc = QJsonDocument::fromJson(line, &jsonError);

//    if (jsonError.error != QJsonParseError::NoError) {
//        qDebug() << "Error JSON in theme" << fileInfo.absoluteFilePath() << "result :" <<
//            jsonError.errorString();
//    }

//    if (jsonDoc.isNull()) {
//        qDebug() << "jsonDoc.isNull";
//        return;
//    }

//    // check editable

//    QJsonObject rootJsonObject = jsonDoc.object();
//    bool isThemeEditable       = rootJsonObject.value("isEditable").toBool(false);

//    if (isThemeEditable && !fileInfo.isWritable()) {
//        isThemeEditable = false;
//    }

//    if (!isThemeEditable) {
//        qWarning() << themeName << "isn't editable";
//    }


//    // write properties

//    QJsonObject normalObject = rootJsonObject.value("colors").toObject();


//    for (const QString& key : colorMap.keys()) {
//        normalObject.insert(key, colorMap.value(key).toString().toLower());
//    }


//    rootJsonObject.insert("colors", normalObject);


//    // write back to file
//    jsonDoc.setObject(rootJsonObject);

//    if (!file.open(QIODevice::WriteOnly | QIODevice::Text | QIODevice::Truncate)) {
//        return;
//    }


//    file.write(jsonDoc.toJson());
//    file.close();


//    // update if current

//    switch (m_currentColorMode) {
//    case Light:

//        if (getCurrentLightTheme() == themeName) {
//            applyTheme(themeName);
//        }
//        break;

//    case Dark:

//        if (getCurrentDarkTheme() == themeName) {
//            applyTheme(themeName);
//        }
//        break;

//    case DistractionFree:

//        if (getCurrentDistractionFreeTheme() == themeName) {
//            applyTheme(themeName);
//        }
//        break;
//    }
//}

// ----------------------------------------------------------------------

#ifndef THEMEMANAGER_H
#define THEMEMANAGER_H

#include "skribisto_desktop_common_global.h"
#include <QApplication>
#include <QObject>
#include <QPalette>
#include <QWidget>
#define themeManager ThemeManager::instance()

class SKRDESKTOPCOMMONEXPORT ThemeManager : public QObject {
  Q_OBJECT
public:
  enum ThemeType { Light, Dark };
  Q_ENUM(ThemeType);

  ~ThemeManager();
  static ThemeManager *instance() {
    if (!m_instance)
      m_instance = new ThemeManager(qApp);
    return m_instance;
  }

  const ThemeManager::ThemeType &currentThemeType() const;
  void setCurrentThemeType(const ThemeManager::ThemeType &newCurrentThemeType);

  const QString &lightTheme() const;
  void setLightTheme(const QString &newLightTheme);
  QString defaultLightTheme() const { return "Default Light"; }

  const QString &darkTheme() const;
  void setDarkTheme(const QString &newDarkTheme);
  QString defaultDarkTheme() const { return "Default Dark"; }

  QString themeInfo(const QString &themeName) const;

  const QMap<QString, QString> &lightThemeWithLocationMap() const;

  const QMap<QString, QString> &darkThemeWithLocationMap() const;

  void reloadThemes();

  const QHash<QString, bool> &themeWithEditableHash() const;

  void saveThemeState();
  void restoreThemeState();

  ThemeManager::ThemeType themeType(const QString &themeName) const;

  QPalette toPalette(const QMap<QString, QString> colorMap) const;
  QMap<QString, QString> getColorMap(const QString &themeName) const;
  void saveTheme(const QString &themeName, const QString &location,
                 const ThemeManager::ThemeType &themeType,
                 const QMap<QString, QString> colorMap) const;

  void addIconWidget(QWidget *widgetWithIcon);

  void scanChildrenAndAddWidgetsHoldingIcons(QWidget *widgetWithIcon);
  QIcon adaptIcon(const QIcon &icon) const;

  ThemeManager::ThemeType switchThemeType();
  void reapplyCurrentTheme();
  QString getWritablePathForTheme() const;
  QPalette createSidePalette(const QPalette &palette) const;
  QPalette createMiddlePalette(const QPalette &palette) const;
public slots:

  void updateAllIconColors();

signals:
  void currentThemeTypeChanged(ThemeManager::ThemeType currentTheme);
  void lightThemeChanged(QString name);
  void darkThemeChanged(QString name);

private:
  explicit ThemeManager(QObject *parent = nullptr);
  static ThemeManager *m_instance;

  ThemeManager::ThemeType m_currentThemeType;
  QString m_currentThemeName;
  QString m_lightTheme, m_darkTheme;
  QPalette m_currentPalette;
  QMap<QString, QString> m_lightThemeWithLocationMap;
  QMap<QString, QString> m_darkThemeWithLocationMap;
  QHash<QString, bool> m_themeWithEditableHash;
  QList<QPointer<QObject>> m_iconWidgetList;

  void applyTheme(const QString &themeName);

  QJsonDocument loadThemeJson(const QString &themeName) const;
};

#endif // THEMEMANAGER_H

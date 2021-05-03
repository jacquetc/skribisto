#ifndef SKRTHEMES_H
#define SKRTHEMES_H

#include <QHash>
#include <QObject>
#include <QString>
#include <QStringList>
#include <QVariantMap>

class SKRThemes : public QObject {
    Q_OBJECT


    Q_PROPERTY(
        QString currentLightTheme READ getCurrentLightTheme WRITE setCurrentLightTheme NOTIFY currentLightThemeChanged)
    Q_PROPERTY(
        QString currentDarkTheme READ getCurrentDarkTheme WRITE setCurrentDarkTheme NOTIFY currentDarkThemeChanged)
    Q_PROPERTY(
        QString currentDistractionFreeTheme READ getCurrentDistractionFreeTheme WRITE setCurrentDistractionFreeTheme NOTIFY currentDistractionFreeThemeChanged)
    Q_PROPERTY(
        ColorMode currentColorMode READ getCurrentColorMode WRITE setCurrentColorMode NOTIFY currentColorModeChanged)
    Q_PROPERTY(QString selectedTheme READ selectedTheme WRITE setSelectedTheme NOTIFY selectedThemeChanged)
    Q_PROPERTY(QString mainTextAreaBackground MEMBER m_mainTextAreaBackground NOTIFY colorsChanged)
    Q_PROPERTY(QString mainTextAreaForeground MEMBER m_mainTextAreaForeground NOTIFY colorsChanged)

    Q_PROPERTY(QString secondaryTextAreaBackground MEMBER m_secondaryTextAreaBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString secondaryTextAreaForeground MEMBER m_secondaryTextAreaForeground NOTIFY colorsChanged)

    Q_PROPERTY(QString pageBackground MEMBER m_pageBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString buttonBackground MEMBER m_buttonBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString buttonForeground MEMBER m_buttonForeground NOTIFY colorsChanged)

    Q_PROPERTY(QString buttonIcon MEMBER m_buttonIcon NOTIFY colorsChanged)

    Q_PROPERTY(QString buttonIconDisabled MEMBER m_buttonIconDisabled NOTIFY colorsChanged)

    Q_PROPERTY(QString accent MEMBER m_accent NOTIFY colorsChanged)

    Q_PROPERTY(QString spellcheck MEMBER m_spellcheck NOTIFY colorsChanged)

    Q_PROPERTY(QString toolBarBackground MEMBER m_toolBarBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString pageToolBarBackground MEMBER m_pageToolBarBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString divider MEMBER m_divider NOTIFY colorsChanged)

    Q_PROPERTY(QString menuBackground MEMBER m_menuBackground NOTIFY colorsChanged)

    Q_PROPERTY(QString listItemBackground MEMBER m_listItemBackground NOTIFY colorsChanged)
    Q_PROPERTY(QVariantMap selectedColorsMap MEMBER m_selectedColorsMap NOTIFY selectedColorsMapChanged)

public:

    enum ColorMode { Light,
                     Dark,
                     DistractionFree };

    Q_ENUM(ColorMode)


    explicit SKRThemes(QObject *parent = nullptr);
    void                    populate();

    Q_INVOKABLE QStringList getThemeList() const;

    Q_INVOKABLE QString     defaultLightTheme() const;
    Q_INVOKABLE QString     defaultDarkTheme() const;
    Q_INVOKABLE QString     defaultDistractionFreeTheme() const;
    Q_INVOKABLE bool        isThemeEditable(const QString& themeName) const;
    Q_INVOKABLE bool        doesThemeExist(const QString& themeName) const;

    Q_INVOKABLE void        saveTheme(const QString    & themeName,
                                      const QVariantMap& colorMap);
    Q_INVOKABLE bool        duplicate(const QString& themeName,
                                      const QString& newThemeName);
    Q_INVOKABLE bool        remove(const QString& themeName);
    Q_INVOKABLE bool        rename(const QString& themeName,
                                   const QString& newThemeName);

    Q_INVOKABLE void        resetSelectedThemeColors();

    SKRThemes::ColorMode    getCurrentColorMode() const;
    void                    setCurrentColorMode(SKRThemes::ColorMode colorMode);

    QString                 getCurrentDarkTheme() const;
    void                    setCurrentDarkTheme(const QString& currentDarkTheme);

    QString                 getCurrentLightTheme() const;
    void                    setCurrentLightTheme(const QString& currentLightTheme);

    QString                 getCurrentDistractionFreeTheme() const;
    void                    setCurrentDistractionFreeTheme(const QString& currentDistractionFreeTheme);

signals:

    void selectedThemeChanged(const QString& themeName);
    void colorsChanged();
    void currentLightThemeChanged(const QString& themeName);
    void currentDarkThemeChanged(const QString& themeName);
    void currentDistractionFreeThemeChanged(const QString& themeName);
    void currentColorModeChanged(const SKRThemes::ColorMode& colorMode);
    void selectedColorsMapChanged(const QVariantMap& map);

private:

    QString selectedTheme();
    void    setSelectedTheme(const QString& themeName);
    void    applyTheme(const QString& themeName,
                       bool           selected = false);

private:

    QHash<QString, QString>m_fileByThemeNameHash;
    QHash<QString, bool>m_isEditableByThemeNameHash;

    QString m_currentLightTheme;
    QString m_currentDarkTheme;
    QString m_currentDistractionFreeTheme;

    QString m_selectedTheme;
    SKRThemes::ColorMode m_currentColorMode;

    QString m_mainTextAreaBackground;
    QString m_mainTextAreaForeground;
    QString m_secondaryTextAreaBackground;
    QString m_secondaryTextAreaForeground;
    QString m_pageBackground;
    QString m_buttonBackground;
    QString m_buttonForeground;
    QString m_buttonIcon;
    QString m_buttonIconDisabled;
    QString m_accent;
    QString m_spellcheck;
    QString m_toolBarBackground;
    QString m_pageToolBarBackground;
    QString m_divider;
    QString m_menuBackground;
    QString m_listItemBackground;
    QVariantMap m_selectedColorsMap;
};

#endif // SKRTHEMES_H

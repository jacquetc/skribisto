#ifndef SKRTHEMES_H
#define SKRTHEMES_H

#include <QHash>
#include <QObject>
#include <QString>
#include <QStringList>

class SKRThemes : public QObject {
    Q_OBJECT
    Q_PROPERTY(
               QString currentTheme READ currentTheme WRITE setCurrentTheme NOTIFY currentThemeChanged)

    Q_PROPERTY(
               QString mainTextAreaBackground MEMBER m_mainTextAreaBackground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString distractionFree_mainTextAreaBackground MEMBER m_distractionFree_mainTextAreaBackground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString mainTextAreaForeground MEMBER m_mainTextAreaForeground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString distractionFree_mainTextAreaForeground MEMBER m_distractionFree_mainTextAreaForeground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString secondaryTextAreaBackground MEMBER m_secondaryTextAreaBackground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString distractionFree_secondaryTextAreaBackground MEMBER m_distractionFree_secondaryTextAreaBackground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString secondaryTextAreaForeground MEMBER m_secondaryTextAreaForeground NOTIFY colorsChanged)
    Q_PROPERTY(
               QString distractionFree_secondaryTextAreaForeground MEMBER m_distractionFree_secondaryTextAreaForeground NOTIFY colorsChanged)
    Q_PROPERTY(QString pageBackground MEMBER m_pageBackground NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_pageBackground MEMBER m_distractionFree_pageBackground NOTIFY colorsChanged)
    Q_PROPERTY(QString buttonBackground MEMBER m_buttonBackground NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_buttonBackground MEMBER m_distractionFree_buttonBackground NOTIFY colorsChanged)
    Q_PROPERTY(QString buttonForeground MEMBER m_buttonForeground NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_buttonForeground MEMBER m_distractionFree_buttonForeground NOTIFY colorsChanged)
    Q_PROPERTY(QString buttonIcon MEMBER m_buttonIcon NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_buttonIcon MEMBER m_distractionFree_buttonIcon NOTIFY colorsChanged)
    Q_PROPERTY(QString buttonIconDisabled MEMBER m_buttonIconDisabled NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_buttonIconDisabled MEMBER m_distractionFree_buttonIconDisabled NOTIFY colorsChanged)
    Q_PROPERTY(QString accent MEMBER m_accent NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_accent MEMBER m_distractionFree_accent NOTIFY colorsChanged)
    Q_PROPERTY(QString spellcheck MEMBER m_spellcheck NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_spellcheck MEMBER m_distractionFree_spellcheck NOTIFY colorsChanged)
    Q_PROPERTY(QString toolBarBackground MEMBER m_toolBarBackground NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_toolBarBackground MEMBER m_distractionFree_toolBarBackground NOTIFY colorsChanged)
    Q_PROPERTY(QString divider MEMBER m_divider NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_divider MEMBER m_distractionFree_divider NOTIFY colorsChanged)
    Q_PROPERTY(QString menuBackground MEMBER m_menuBackground NOTIFY colorsChanged)
    Q_PROPERTY(
        QString distractionFree_menuBackground MEMBER m_distractionFree_menuBackground NOTIFY colorsChanged)

public:

    explicit SKRThemes(QObject *parent = nullptr);
    void                    populate();

    Q_INVOKABLE QStringList getThemeList() const;

    Q_INVOKABLE QString     defaultTheme() const;
    Q_INVOKABLE bool        isThemeEditable(const QString& themeName) const;
    Q_INVOKABLE bool        doesThemeExist(const QString& themeName) const;

    Q_INVOKABLE void        saveTheme(const QString& themeName);
    Q_INVOKABLE bool        duplicate(const QString& themeName,
                                      const QString& newThemeName);
    Q_INVOKABLE bool        remove(const QString& themeName);

signals:

    void currentThemeChanged(const QString& themeName);
    void colorsChanged();

private:

    QString currentTheme();
    void    setCurrentTheme(const QString& themeName);
    void    applyTheme(const QString& themeName);

private:

    QHash<QString, QString>m_fileByThemeNameHash;
    QHash<QString, bool>m_isEditableByThemeNameHash;
    QString m_currentTheme;

    QString m_mainTextAreaBackground, m_distractionFree_mainTextAreaBackground;
    QString m_mainTextAreaForeground, m_distractionFree_mainTextAreaForeground;
    QString m_secondaryTextAreaBackground, m_distractionFree_secondaryTextAreaBackground;
    QString m_secondaryTextAreaForeground, m_distractionFree_secondaryTextAreaForeground;
    QString m_pageBackground, m_distractionFree_pageBackground;
    QString m_buttonBackground, m_distractionFree_buttonBackground;
    QString m_buttonForeground, m_distractionFree_buttonForeground;
    QString m_buttonIcon, m_distractionFree_buttonIcon;
    QString m_buttonIconDisabled, m_distractionFree_buttonIconDisabled;
    QString m_accent, m_distractionFree_accent;
    QString m_spellcheck, m_distractionFree_spellcheck;
    QString m_toolBarBackground, m_distractionFree_toolBarBackground;
    QString m_divider, m_distractionFree_divider;
    QString m_menuBackground, m_distractionFree_menuBackground;
};

#endif // SKRTHEMES_H

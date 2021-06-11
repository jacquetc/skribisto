#ifndef SKRSHORTCUTMANAGER_H
#define SKRSHORTCUTMANAGER_H

#include <QObject>
#include <QKeySequence>


struct SKRShortcut
{
    Q_GADGET

public:

    explicit SKRShortcut();
    SKRShortcut(const SKRShortcut& otherShortcut);
    SKRShortcut(const QString    & name,
                const QString    & description,
                const QStringList& groups,
                const QStringList& defaultShortcuts,
                const QStringList& userShortcuts);
    SKRShortcut      & operator=(const SKRShortcut& otherShortcut);
    bool               operator==(const SKRShortcut& otherShortcut) const;
    bool               operator!=(const SKRShortcut& otherShortcut) const;

    const QString    & name() const;
    void               setName(const QString& newName);

    const QStringList& groups() const;
    void               setGroups(const QStringList& newGroups);

    const QStringList& defaultShortcuts() const;
    void               setDefaultShortcuts(const QStringList& newDefaultShortcuts);

    const QStringList& userShortcuts() const;
    void               setUserShortcuts(const QStringList& newUserShortcuts);

    const QString    & description() const;
    void               setDescription(const QString& newDescription);

private:

    QString     m_name;
    QString     m_description;
    QStringList m_groups;
    QStringList m_defaultShortcuts;
    QStringList m_userShortcuts;
};

// -----------------------------------------------------------------
// -----------------------------------------------------------------
// -----------------------------------------------------------------

class SKRShortcutManager : public QObject {
    Q_OBJECT

public:

    explicit SKRShortcutManager(QObject *parent = nullptr);

    Q_INVOKABLE QList<SKRShortcut>shortcutList();

    Q_INVOKABLE QString           description(const QString& name) const;
    Q_INVOKABLE QStringList       shortcuts(const QString& name) const;
    Q_INVOKABLE void              setUserShortcuts(const QString    & name,
                                                   const QStringList& userShortcuts);

    Q_INVOKABLE QStringList       getNativeFormatOfDefaultShortcuts(const QString& name) const;

public slots:

    void populateShortcutList();

signals:

private:

    SKRShortcut createShortcut(const QString    & name,
                               const QString    & description,
                               const QStringList& groups,
                               const QStringList& defaultShortcuts) const;
    QList<SKRShortcut>m_shortcutList;
    QStringList convertStandardKeyToList(QKeySequence::StandardKey key);
};

#endif // SKRSHORTCUTMANAGER_H

#include "skrshortcutmanager.h"

#include <QSettings>

SKRShortcut::SKRShortcut()
{}

SKRShortcut::SKRShortcut(const SKRShortcut& otherShortcut)
{
    m_name             = otherShortcut.name();
    m_groups           = otherShortcut.groups();
    m_defaultShortcuts = otherShortcut.defaultShortcuts();
    m_userShortcuts    = otherShortcut.userShortcuts();
    m_description      = otherShortcut.description();
}

SKRShortcut::SKRShortcut(const QString    & name,
                         const QString    & description,
                         const QStringList& groups,
                         const QStringList& defaultShortcuts,
                         const QStringList& userShortcuts)
{
    m_name             = name;
    m_groups           = groups;
    m_defaultShortcuts = defaultShortcuts;
    m_userShortcuts    = userShortcuts;
    m_description      = description;
}

SKRShortcut& SKRShortcut::operator=(const SKRShortcut& otherShortcut)
{
    if (Q_LIKELY(&otherShortcut != this)) {
        m_name             = otherShortcut.name();
        m_groups           = otherShortcut.groups();
        m_defaultShortcuts = otherShortcut.defaultShortcuts();
        m_userShortcuts    = otherShortcut.userShortcuts();
        m_description      = otherShortcut.description();
    }

    return *this;
}

bool SKRShortcut::operator==(const SKRShortcut& otherShortcut) const
{
    return m_name == otherShortcut.name() &&
           m_groups == otherShortcut.groups() &&
           m_defaultShortcuts == otherShortcut.defaultShortcuts() &&
           m_userShortcuts == otherShortcut.userShortcuts() &&
           m_description == otherShortcut.description();
}

bool SKRShortcut::operator!=(const SKRShortcut& otherShortcut) const
{
    return m_name != otherShortcut.name() ||
           m_groups != otherShortcut.groups() ||
           m_defaultShortcuts != otherShortcut.defaultShortcuts() ||
           m_userShortcuts != otherShortcut.userShortcuts() ||
           m_description != otherShortcut.description();
}

const QString& SKRShortcut::name() const
{
    return m_name;
}

void SKRShortcut::setName(const QString& newName)
{
    m_name = newName;
}

const QStringList& SKRShortcut::groups() const
{
    return m_groups;
}

void SKRShortcut::setGroups(const QStringList& newGroups)
{
    m_groups = newGroups;
}

const QStringList& SKRShortcut::defaultShortcuts() const
{
    return m_defaultShortcuts;
}

void SKRShortcut::setDefaultShortcuts(const QStringList& newDefaultShortcuts)
{
    m_defaultShortcuts = newDefaultShortcuts;
}

const QStringList& SKRShortcut::userShortcuts() const
{
    return m_userShortcuts;
}

void SKRShortcut::setUserShortcuts(const QStringList& newUserShortcuts)
{
    m_userShortcuts = newUserShortcuts;
}

const QString& SKRShortcut::description() const
{
    return m_description;
}

void SKRShortcut::setDescription(const QString& newDescription)
{
    m_description = newDescription;
}

// -----------------------------------------------------------------
// -----------------------------------------------------------------
// -----------------------------------------------------------------

SKRShortcutManager::SKRShortcutManager(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRShortcut> >("QList<SKRShortcut>");

    populateShortcutList();
}

// -----------------------------------------------------------------

void SKRShortcutManager::populateShortcutList()
{
    m_shortcutList.clear();

    m_shortcutList <<
        createShortcut("cut",   tr("Cut"),   (QStringList() << "text" << "navigation"),
                       (QStringList() <<  convertStandardKeyToList(QKeySequence::Cut)));
    m_shortcutList <<
        createShortcut("copy",  tr("Copy"),  (QStringList() << "text" << "navigation"),
                       (QStringList() <<  convertStandardKeyToList(QKeySequence::Copy)));
    m_shortcutList <<
        createShortcut("paste", tr("Paste"), (QStringList() << "text" << "navigation"),
                       (QStringList() <<  convertStandardKeyToList(QKeySequence::Paste)));


    // add shortcuts from plugins
}

QStringList SKRShortcutManager::convertStandardKeyToList(QKeySequence::StandardKey key) {
    return QKeySequence(key).toString().split(",", Qt::SkipEmptyParts);
}

// -----------------------------------------------------------------

QList<SKRShortcut>SKRShortcutManager::shortcutList()
{
    return m_shortcutList;
}

// -----------------------------------------------------------------

QStringList SKRShortcutManager::getNativeFormatOfDefaultShortcuts(const QString& name) const
{
    QStringList shortcuts;

    for (const SKRShortcut& shortcut : m_shortcutList) {
        if (shortcut.name() == name) {
            shortcuts = shortcut.defaultShortcuts();
            break;
        }
    }
    QStringList nativeShortcuts;

    for (const QString& shortcut : qAsConst(shortcuts)) {
        nativeShortcuts << QKeySequence(shortcut).toString(QKeySequence::NativeText);
    }


    return nativeShortcuts;
}

// -----------------------------------------------------------------

QString SKRShortcutManager::description(const QString& name) const
{
    QString description;

    for (const SKRShortcut& shortcut : m_shortcutList) {
        if (shortcut.name() == name) {
            description = shortcut.description();
            break;
        }
    }

    return description;
}

// -----------------------------------------------------------------

QStringList SKRShortcutManager::shortcuts(const QString& name) const
{
    QStringList shortcuts;

    for (const SKRShortcut& shortcut : m_shortcutList) {
        if (shortcut.name() == name) {
            if (shortcut.userShortcuts().isEmpty()) {
                shortcuts = shortcut.defaultShortcuts();
            }
            else {
                shortcuts = shortcut.userShortcuts();
            }
            break;
        }
    }

    return shortcuts;
}

// -----------------------------------------------------------------

void SKRShortcutManager::setUserShortcuts(const QString& name, const QStringList& userShortcuts)
{
    QSettings settings;

    settings.beginGroup("shortcuts");
    settings.setValue(name, userShortcuts);

    populateShortcutList();
}

// -----------------------------------------------------------------

SKRShortcut SKRShortcutManager::createShortcut(const QString& name, const QString& description,
                                               const QStringList& groups,
                                               const QStringList& defaultShortcuts) const
{
    QSettings settings;

    settings.beginGroup("shortcuts");
    QStringList userShortcuts = settings.value(name, QStringList()).toStringList();

    return SKRShortcut(name, description, groups, defaultShortcuts, userShortcuts);
}

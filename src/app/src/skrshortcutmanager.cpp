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

    m_shortcutList <<
        createShortcut("settings", tr("&Settings"), (QStringList() << "global"),
                       (QStringList()));

    m_shortcutList <<
        createShortcut("print", tr("&Print"), (QStringList() << "global"),
                       (QStringList() << convertStandardKeyToList(QKeySequence::Print)));

    m_shortcutList <<
        createShortcut("fullscreen", tr("Fullscreen"), (QStringList() << "global"),
                       (QStringList() <<
                        tr("F11", "fullscreen") << convertStandardKeyToList(QKeySequence::FullScreen)));

    m_shortcutList <<
        createShortcut("center-vert-text-cursor", tr("Center vertically the text cursor"), (QStringList() << "global"),
                       (QStringList() << tr("Alt+C", "center-vert-text-cursor")));

    m_shortcutList <<
        createShortcut("show-minimap-scrollbar", tr("Show the minimap scrollbar"), (QStringList() << "global"),
                       (QStringList() << tr("Alt+M", "center-vert-text-cursor")));

    m_shortcutList <<
        createShortcut("user-manual", tr("&User manual"), (QStringList() << "global"),
                       (QStringList() << convertStandardKeyToList(QKeySequence::HelpContents)));

    m_shortcutList <<
        createShortcut("new-project", tr("&New Project"), (QStringList() << "global"),
                       (QStringList() << convertStandardKeyToList(QKeySequence::New)));

    m_shortcutList <<
        createShortcut("check-spelling", tr("&Check spelling"), (QStringList() << "global"),
                       (QStringList() << tr("Shift+F7", "check-spelling")));


    m_shortcutList <<
        createShortcut("open-project", tr("&Open"), (QStringList() << "global"),
                       (QStringList() << convertStandardKeyToList(QKeySequence::Open)));

    m_shortcutList <<
        createShortcut("save-project", tr("&Save"), (QStringList() << "global"),
                       (QStringList() << convertStandardKeyToList(QKeySequence::Save)));

    m_shortcutList <<
        createShortcut("save-all-project", tr("&Save All"), (QStringList() << "global"),
                       (QStringList() << tr("Ctrl+Shift+S", "save-all-project")));

    m_shortcutList <<
        createShortcut("save-as-project", tr("&Save As …"), (QStringList() << "global"),
                       (QStringList() <<  convertStandardKeyToList(QKeySequence::SaveAs)));

    m_shortcutList <<
        createShortcut("quit", tr("&Quit"), (QStringList() << "global"),
                       (QStringList() <<  convertStandardKeyToList(QKeySequence::Quit) << tr("Ctrl+Q")));

    m_shortcutList <<
        createShortcut("create-new-identical-page",
                       tr("Create a new page of the same type"),
                       (QStringList() << "global"),
                       (QStringList() << tr("Ctrl+Return", "create-new-identical-page")));

    m_shortcutList <<
        createShortcut("show-relationship-panel",
                       tr("Show relationships"),
                       (QStringList() << "page"),
                       (QStringList() << tr("Alt+R", "show-relationship-panel")));


    m_shortcutList <<
        createShortcut("add-quick-note",
                       tr("Add a quick note"),
                       (QStringList() << "page"),
                       (QStringList() << tr("Alt+N", "add-quick-note")));

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
    // menmonic:

    QStringList finalDefaultShortcuts = defaultShortcuts;

    if (description.contains("&")) {
        finalDefaultShortcuts << QKeySequence::mnemonic(tr("&Settings")).toString();
    }


    // use shortcuts:
    QSettings settings;

    settings.beginGroup("shortcuts");
    QStringList userShortcuts = settings.value(name, QStringList()).toStringList();

    return SKRShortcut(name, description, groups, finalDefaultShortcuts, userShortcuts);
}

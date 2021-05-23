#include "skrpluginloader.h"
#include "plmutils.h"
#include <QSettings>
#include <QDebug>

// #include "textconverter/converterinterface.h"

SKRPluginLoader::SKRPluginLoader(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRPlugin> >("QList<SKRPlugin>");

    for (const QString& path : PLMUtils::Dir::addonsPathsList()) {
        QCoreApplication::addLibraryPath(path);
    }


    // this->reload();
}

// ---------------------------------------------------------------------------

SKRPluginLoader::~SKRPluginLoader()
{}

// -----------------------------------------------------------------------------


// ---------------------------------------------------------------------------


// void SKRPluginLoader::reload()
// {
//    m_pluginsListHash.clear();

//    //    this->addPluginType<ConverterInterface>();
//    //       QList<ConverterInterface *> converterInterfaces;
//    //    foreach ( const QString  &path, Utils::Dir::addonsPathsList())
//    //
//    //
//    //
//    //
//    //
//     converterInterfaces.append(this->pluginByDir<ConverterInterface>(path
//    // + "/plugins/convert"));
//    // //    foreach (ConverterInterface *converterInterface,
//    // converterInterfaces)
//    // //        m_pluginsListHash.insert(converterInterface->name(),
//    // converterInterface);
//    QList<SKRSideMainBarIconInterface *> panelInterfaces;

//    foreach(const QString  &path, SKRUtils::Dir::addonsPathsList()) {
//        panelInterfaces.append(SKRPluginLoader::pluginByDir<
//                                   SKRSideMainBarIconInterface>(path +
// "/plugins"));
//    }

//    foreach(SKRSideMainBarIconInterface * interface, panelInterfaces) {
//        SKRPlugin plugin(
//            interface->name(), "",  dynamic_cast<QObject *>(interface));

//        m_pluginsListHash.insert(interface->name(), plugin);
//    }

//    QList<SKRSideMainBarIconInterface *> sideBarInterfaces;

//    foreach(const QString  &path, SKRUtils::Dir::addonsPathsList()) {
//        sideBarInterfaces.append(SKRPluginLoader::pluginByDir<
//                                     SKRSideMainBarIconInterface>(path +
//
//                                                                "/plugins"));
//    }

//    foreach(SKRSideMainBarIconInterface * interface, sideBarInterfaces) {
//        SKRPlugin plugin(
//            interface->name(), "",  dynamic_cast<QObject *>(interface));

//        m_pluginsListHash.insert(interface->name(), plugin);
//    }

//    foreach(SKRPlugin plugin, m_pluginsListHash.values()) {
//        QObject *object    = plugin.object;
//        QString  className = object->metaObject()->className();

//        qDebug() << "className : " + className;
//    }

// }

// ---------------------------------------------------------------------------


QStringList SKRPluginLoader::listAllByName()
{
    QStringList nameList;

    for (SKRPlugin plugin : m_pluginsListHash.values()) {
        nameList << plugin.name;
    }

    return nameList;
}

// ---------------------------------------------------------------------------

QStringList SKRPluginLoader::listActivatedByName()
{
    QStringList nameList;
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (dynamic_cast<SKRCoreInterface *>(plugin.object)->pluginEnabled()) {
            nameList << plugin.name;
        }
        ++i;
    }

    return nameList;
}

// ---------------------------------------------------------------------------

QString SKRPluginLoader::getDisplayedName(const QString& pluginName) const
{
    QString name;
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (plugin.name == pluginName) {
            if (dynamic_cast<SKRCoreInterface *>(plugin.object)) {
                SKRCoreInterface *coreInterface = dynamic_cast<SKRCoreInterface *>(plugin.object);
                name = coreInterface->displayedName();
                break;
            }
        }
        ++i;
    }
    return name;
}

// ---------------------------------------------------------------------------

QString SKRPluginLoader::getUse(const QString& pluginName) const
{
    QString name;
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (plugin.name == pluginName) {
            if (dynamic_cast<SKRCoreInterface *>(plugin.object)) {
                SKRCoreInterface *coreInterface = dynamic_cast<SKRCoreInterface *>(plugin.object);
                name = coreInterface->use();
                break;
            }
        }
        ++i;
    }
    return name;
}

// ---------------------------------------------------------------------------

bool SKRPluginLoader::getMandatory(const QString& pluginName) const
{
    bool mandatory                              = false;
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (plugin.name == pluginName) {
            if (plugin.object->property("mandatory").isValid()) {
                mandatory = plugin.object->property("mandatory").toBool();
            }
            break;
        }
        ++i;
    }
    return mandatory;
}

// ---------------------------------------------------------------------------

bool SKRPluginLoader::isThisPluginEnabled(const QString& pluginName)
{
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (plugin.name == pluginName) {
            SKRCoreInterface *corePlugin = dynamic_cast<SKRCoreInterface *>(plugin.object);
            if(!corePlugin){
                qDebug() << plugin.name << "is not SKRCoreInterface";
                return false;
            }
            else if (corePlugin->pluginEnabled()) {
                return true;
            }
        }
        ++i;
    }

    return false;
}

// ---------------------------------------------------------------------------
void SKRPluginLoader::setPluginEnabled(const QString& pluginName, bool enabled) {
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if (plugin.name == pluginName) {
            if (dynamic_cast<SKRCoreInterface *>(plugin.object)->pluginEnabled() != enabled) {
                dynamic_cast<SKRCoreInterface *>(plugin.object)->setPluginEnabled(enabled);
            }
            break;
        }
        ++i;
    }
}

// ---------------------------------------------------------------------------
void SKRPluginLoader::enablePlugin(const QString& pluginName)
{
    setPluginEnabled(pluginName, true);
}

// ---------------------------------------------------------------------------

void SKRPluginLoader::disablePlugin(const QString& pluginName)
{
    setPluginEnabled(pluginName, true);
}

// ---------------------------------------------------------------------------


QObject * SKRPluginLoader::pluginObjectByName(const QString& fileName)
{
    QPluginLoader loader(fileName);
    QObject *plugin = loader.instance();

    if (!plugin) {
        qDebug() << "loader.instance() FOR : " + fileName;
        qDebug() << "loader.instance() : " + loader.errorString();
    } else {
        plugin->setProperty("fileName", fileName);
        plugin->setProperty("activatedByDefault",
                            loader.metaData().value("MetaData").toObject().value(
                                "activatedByDefault").toBool());
        plugin->setProperty("version",
                            loader.metaData().value("MetaData").toObject().value(
                                "version").toString());
        plugin->setProperty("mandatory",
                            loader.metaData().value("MetaData").toObject().value(
                                "mandatory").toBool());
        plugin->setProperty("shortname",
                            loader.metaData().value("MetaData").toObject().value(
                                "shortname").toString());
        plugin->setProperty("longname",
                            loader.metaData().value("MetaData").toObject().value(
                                "longname").toString());
        plugin->setProperty("type",
                            loader.metaData().value("MetaData").toObject().value(
                                "type").toString());
    }

    return plugin;
}

// ---------------------------------------------------------------------------


void SKRPluginLoader::installPluginTranslations()
{
    // install translation of plugins:
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        QString className = i.value().object->metaObject()->className();

        // qDebug() << className;
        QTranslator *translator = new QTranslator(qApp);

        if (translator->load(QLocale(PLMUtils::Lang::getUserLang()), className.toLower(),
                             "_", ":/translations")) {
            qApp->installTranslator(translator);
        }

        ++i;
    }
}

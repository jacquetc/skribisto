#include "plmpluginloader.h"
#include "plmutils.h"
#include <QSettings>
#include <QDebug>

// #include "textconverter/converterinterface.h"

PLMPluginLoader::PLMPluginLoader(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<PLMPlugin> >("QList<PLMPlugin>");
    m_instance = this;

    foreach(const QString  &path, PLMUtils::Dir::addonsPathsList()) {
        QCoreApplication::addLibraryPath(path);
    }
    QCoreApplication::removeLibraryPath(qApp->applicationDirPath());

//    foreach(const QString  &path, QCoreApplication::libraryPaths() ) {
//    qDebug() << path;
//    }


    // this->reload();
}

// ---------------------------------------------------------------------------

PLMPluginLoader::~PLMPluginLoader()
{}

// -----------------------------------------------------------------------------


PLMPluginLoader *PLMPluginLoader::m_instance = nullptr;

// ---------------------------------------------------------------------------


//void PLMPluginLoader::reload()
//{
//    m_pluginsListHash.clear();

//    //    this->addPluginType<ConverterInterface>();
//    //       QList<ConverterInterface *> converterInterfaces;
//    //    foreach ( const QString  &path, Utils::Dir::addonsPathsList())
//    //
//    //
//    //
//    //
//    //     converterInterfaces.append(this->pluginByDir<ConverterInterface>(path
//    // + "/plugins/convert"));
//    // //    foreach (ConverterInterface *converterInterface,
//    // converterInterfaces)
//    // //        m_pluginsListHash.insert(converterInterface->name(),
//    // converterInterface);
//    QList<PLMSideMainBarIconInterface *> panelInterfaces;

//    foreach(const QString  &path, PLMUtils::Dir::addonsPathsList()) {
//        panelInterfaces.append(PLMPluginLoader::pluginByDir<
//                                   PLMSideMainBarIconInterface>(path + "/plugins"));
//    }

//    foreach(PLMSideMainBarIconInterface * interface, panelInterfaces) {
//        PLMPlugin plugin(
//            interface->name(), "",  dynamic_cast<QObject *>(interface));

//        m_pluginsListHash.insert(interface->name(), plugin);
//    }

//    QList<PLMSideMainBarIconInterface *> sideBarInterfaces;

//    foreach(const QString  &path, PLMUtils::Dir::addonsPathsList()) {
//        sideBarInterfaces.append(PLMPluginLoader::pluginByDir<
//                                     PLMSideMainBarIconInterface>(path +
//                                                                  "/plugins"));
//    }

//    foreach(PLMSideMainBarIconInterface * interface, sideBarInterfaces) {
//        PLMPlugin plugin(
//            interface->name(), "",  dynamic_cast<QObject *>(interface));

//        m_pluginsListHash.insert(interface->name(), plugin);
//    }

//    foreach(PLMPlugin plugin, m_pluginsListHash.values()) {
//        QObject *object    = plugin.object;
//        QString  className = object->metaObject()->className();

//        qDebug() << "className : " + className;
//    }

//}

// ---------------------------------------------------------------------------


QList<PLMPlugin>PLMPluginLoader::listAll()
{
    return m_pluginsListHash.values();
}

// ---------------------------------------------------------------------------

QList<PLMPlugin>PLMPluginLoader::listActivated()
{
    QList<PLMPlugin> list;
    QHash<QString, PLMPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        PLMPlugin plugin = i.value();

        // TODO: link to a list of not activated given in settings
        // if (!QSettings().value("Plugins/deactivatedPlugins",
        //
        // 0).toStringList().contains(plugin.object->metaObject()->className()));
        list << plugin;
        ++i;
    }

    return list;
}

// ---------------------------------------------------------------------------


QObject * PLMPluginLoader::pluginObjectByName(const QString& fileName)
{
    QPluginLoader loader(fileName);
    QObject *plugin = loader.instance();

    if (!plugin) {
        qDebug() << "loader.instance() FOR : " + fileName;
        qDebug() << "loader.instance() : " + loader.errorString();

    } else {
        plugin->setProperty("fileName", fileName);
        plugin->setProperty("activatedbydefault", loader.metaData().value("MetaData").toObject().value("activatedbydefault").toBool());
        plugin->setProperty("version", loader.metaData().value("MetaData").toObject().value("version").toString());
        plugin->setProperty("mandatory", loader.metaData().value("MetaData").toObject().value("mandatory").toString());
        plugin->setProperty("shortname", loader.metaData().value("MetaData").toObject().value("shortname").toString());
        plugin->setProperty("longname", loader.metaData().value("MetaData").toObject().value("longname").toString());

    }

    return plugin;
}

// ---------------------------------------------------------------------------


void PLMPluginLoader::installPluginTranslations()
{
    // install translation of plugins:
    QHash<QString, PLMPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        QString className       = i.value().object->metaObject()->className();
        //qDebug() << className;
        QTranslator *translator = new QTranslator(qApp);

        if (translator->load(QLocale(PLMUtils::Lang::getUserLang()), className.toLower(), "_", ":/translations")) {
            qApp->installTranslator(translator);
        }

        ++i;
    }
}

#include "skrpluginloader.h"
#include "plmutils.h"
#include <QSettings>
#include <QDebug>

// #include "textconverter/converterinterface.h"

SKRPluginLoader::SKRPluginLoader(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRPlugin> >("QList<SKRPlugin>");

    for(const QString& path : PLMUtils::Dir::addonsPathsList()) {
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


QList<SKRPlugin>SKRPluginLoader::listAll()
{
    return m_pluginsListHash.values();
}

// ---------------------------------------------------------------------------

QList<SKRPlugin>SKRPluginLoader::listActivated()
{
    QList<SKRPlugin> list;
    QHash<QString, SKRPlugin>::const_iterator i = m_pluginsListHash.constBegin();

    while (i != m_pluginsListHash.constEnd()) {
        SKRPlugin plugin = i.value();

        if(qobject_cast<SKRCoreInterface *>(plugin.object)->pluginEnabled()){
            list << plugin;
        }
        ++i;
    }

    return list;
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
        plugin->setProperty("activatedbydefault",
                            loader.metaData().value("MetaData").toObject().value(
                                "activatedbydefault").toBool());
        plugin->setProperty("version",
                            loader.metaData().value("MetaData").toObject().value(
                                "version").toString());
        plugin->setProperty("mandatory",
                            loader.metaData().value("MetaData").toObject().value(
                                "mandatory").toString());
        plugin->setProperty("shortname",
                            loader.metaData().value("MetaData").toObject().value(
                                "shortname").toString());
        plugin->setProperty("longname",
                            loader.metaData().value("MetaData").toObject().value(
                                "longname").toString());
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

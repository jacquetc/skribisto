#ifndef SKRPLUGINLOADER_H
#define SKRPLUGINLOADER_H


#include <QList>
#include <QPluginLoader>
#include <QObject>
#include <QString>
#include <QDir>
#include <QObject>
#include <QDebug>
#include <QCoreApplication>
#include "skribisto_data_global.h"
#include "skrcoreinterface.h"
#include "skr.h"

// #include <string>
// #include <cstdlib>
// #include <cxxabi.h>

struct EXPORT SKRPlugin {
    SKRPlugin() {}

    SKRPlugin(const QString& pluginName, const QString& file, QObject *pluginObject)
    {
        name      = pluginName;
        fileName  = file;
        object    = pluginObject;
        className = pluginObject->metaObject()->className();
    }

    SKRPlugin(const SKRPlugin& other)
    {
        name      = other.name;
        fileName  = other.fileName;
        object    = other.object;
        className = other.object->metaObject()->className();
    }

    ~SKRPlugin() {}


    QString  name, fileName, className;
    QObject *object;
};
Q_DECLARE_METATYPE(SKRPlugin)


class EXPORT SKRPluginLoader : public QObject {
    Q_OBJECT

public:

    explicit SKRPluginLoader(QObject *parent = 0);
    ~SKRPluginLoader();


    // void reload();


    Q_INVOKABLE QStringList  listAllByName(bool sorted = false);
    Q_INVOKABLE QString      getDisplayedName(const QString& pluginName) const;
    Q_INVOKABLE QString      getPluginGroup(const QString& pluginName) const;
    Q_INVOKABLE QString      getPluginSelectionGroup(const QString& pluginName) const;
    Q_INVOKABLE QString      getUse(const QString& pluginName) const;
    Q_INVOKABLE bool         getMandatory(const QString& pluginName) const;

    Q_INVOKABLE QStringList  listActivatedByName();
    Q_INVOKABLE void         enablePlugin(const QString& pluginName);
    Q_INVOKABLE void         disablePlugin(const QString& pluginName);
    Q_INVOKABLE void         setPluginEnabled(const QString& pluginName,
                                              bool           enabled);
    Q_INVOKABLE bool         isThisPluginEnabled(const QString& pluginName);


    template<typename T>void addPluginType()
    {
        for (QObject *obj : QPluginLoader::staticInstances()) {
            T *instance = dynamic_cast<T *>(obj);

            if (instance) {
                SKRPlugin plugin(obj->property("shortname").toString(), obj->property(
                                     "fileName").toString(), obj);

                if (!m_pluginsListHash.contains(plugin.name)) {
                    m_pluginsListHash.insert(plugin.name, plugin);
                }
            }
        }

        for (const QString& path : QCoreApplication::libraryPaths()) {
            QList<QObject *> objects = this->pluginObjectsByDir<T>(path);

            for (QObject *obj : objects) {
                T *instance = dynamic_cast<T *>(obj);

                if (instance) {
                    SKRPlugin plugin(obj->property("shortname").toString(), obj->property(
                                         "fileName").toString(), obj);

                    if (!m_pluginsListHash.contains(plugin.name)) {
                        qDebug() << "plugin found : " << plugin.name << "at" << path;
                        m_pluginsListHash.insert(plugin.name, plugin);
                    }
                }
            }
        }
    }

    template<typename T>QList<T *>pluginsByType(bool includeDisabled = false)
    {
        QList<T *> list;

        for (SKRPlugin p : m_pluginsListHash.values()) {
            QObject *object = p.object;


            SKRCoreInterface *corePlugin = dynamic_cast<SKRCoreInterface *>(object);

            if (!corePlugin) {
                qWarning() << "Plugin" << object->property("shortname").toString() <<
                    "is not SKRCoreInterface, ignoring it";
                continue;
            }

            if (!corePlugin->isPluginEnabledSettingExisting()) {
                if (!object->property("activatedByDefault").toBool()) {
                    continue;
                }
            }

            bool pluginEnabled = corePlugin->pluginEnabled();

            if (!pluginEnabled && !includeDisabled) {
                continue;
            }


            if (T *plugin = dynamic_cast<T *>(object)) {
                list.push_back(plugin);
            }
        }

        return list;
    }

    void installPluginTranslations();

private:

    template<typename T>T* pluginByName(const QString& fileName)
    {
        QPluginLoader loader(fileName);
        QObject *plugin = loader.instance();

        if (!plugin) {
            //            qDebug() << "loader.instance() AT : " + fileName;
            //            qDebug() << "loader.instance() : " +
            // loader.errorString();

            // to clean up if wrong libs loaded :
            plugin->deleteLater();
        }
        else {
            this->installPluginTranslations();
        }


        return dynamic_cast<T *>(plugin);
    }

    template<typename T>QList<T *>pluginByDir(const QString& dir)
    {
        QList<T *> ls;
        QDir plugDir = QDir(dir);

        for (const QString& file : plugDir.entryList(QDir::Files)) {
            if (T *plugin =
                    SKRPluginLoader::pluginByName<T>(plugDir.absoluteFilePath(file))) {
                ls.push_back(plugin);
            }
        }

        return ls;
    }

    QObject                           * pluginObjectByName(const QString& fileName);

    template<typename T>QList<QObject *>pluginObjectsByDir(const QString& dir)
    {
        QList<QObject *> ls;
        QDir plugDir = QDir(dir);
        QStringList filter;

        filter << "*.so" << "*.dll" << "*.dylib";
        QDir::Filters filterFlags(QDir::Files& ~QDir::Executable);
        QStringList   list = plugDir.entryList(filter, filterFlags, QDir::NoSort);

        for (const QString& file : plugDir.entryList(filter, filterFlags, QDir::NoSort)) {
            if (T *plugin =
                    SKRPluginLoader::pluginByName<T>(plugDir.absoluteFilePath(file)))
                if (plugin) {
                    ls.push_back(pluginObjectByName(plugDir.absoluteFilePath(file)));
                }
        }

        return ls;
    }

private:

    QHash<QString, SKRPlugin>m_pluginsListHash;
    QStringList m_pluginTypesList;
};


#endif // PLUGINLOADER_H

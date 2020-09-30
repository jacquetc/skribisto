#ifndef PLMPLUGINLOADER_H
#define PLMPLUGINLOADER_H


#include <QList>
#include <QPluginLoader>
#include <QObject>
#include <QString>
#include <QDir>
#include <QObject>
#include <QDebug>
#include <QCoreApplication>
#include "skribisto_data_global.h"

// #include <string>
// #include <cstdlib>
// #include <cxxabi.h>

struct EXPORT PLMPlugin {
    PLMPlugin() {}

    PLMPlugin(const QString& pluginName, const QString& file, QObject *pluginObject)
    {
        name      = pluginName;
        fileName  = file;
        object    = pluginObject;
        className = pluginObject->metaObject()->className();
    }

    PLMPlugin(const PLMPlugin& other)
    {
        name      = other.name;
        fileName  = other.fileName;
        object    = other.object;
        className = other.object->metaObject()->className();
    }

    ~PLMPlugin() {}


    QString  name, fileName, className;
    QObject *object;
};
Q_DECLARE_METATYPE(PLMPlugin)


class EXPORT PLMPluginLoader : public QObject {
    Q_OBJECT

public:

    explicit PLMPluginLoader(QObject *parent = 0);
    ~PLMPluginLoader();

    static PLMPluginLoader* instance()
    {
        return m_instance;
    }

    // void reload();


    QList<PLMPlugin>         listAll();
    QList<PLMPlugin>         listActivated();


    template<typename T>void addPluginType()
    {
        for(QObject * obj : QPluginLoader::staticInstances()) {
            T *instance = qobject_cast<T *>(obj);

            if (instance) {
                PLMPlugin plugin(obj->property("name").toString(), obj->property(
                                     "fileName").toString(), obj);

                if (!m_pluginsListHash.contains(plugin.name)) {
                    m_pluginsListHash.insert(plugin.name, plugin);
                }
            }
        }

        for(const QString& path : QCoreApplication::libraryPaths()) {
            QList<QObject *> objects = this->pluginObjectsByDir<T>(path);

            for(QObject * obj : objects) {
                T *instance = qobject_cast<T *>(obj);

                if (instance) {
                    PLMPlugin plugin(obj->property("name").toString(), obj->property(
                                         "fileName").toString(), obj);

                    if (!m_pluginsListHash.contains(plugin.name)) {
                        m_pluginsListHash.insert(plugin.name, plugin);
                    }
                }
            }
        }
    }

    template<typename T>QList<T *>pluginsByType()
    {
        QList<T *> list;

        for(PLMPlugin p : m_pluginsListHash.values()) {
            QObject *object = p.object;

            if (!object->property("activatedbydefault").toBool()) {
                continue;
            }

            if (T *plugin = qobject_cast<T *>(object)) {
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
            qDebug() << "loader.instance() AT : " + fileName;
            qDebug() << "loader.instance() : " + loader.errorString();

            // to clean up if wrong libs loaded :
            plugin->deleteLater();
        }
        else {
            this->installPluginTranslations();
        }


        return qobject_cast<T *>(plugin);
    }

    template<typename T>QList<T *>pluginByDir(const QString& dir)
    {
        QList<T *> ls;
        QDir plugDir = QDir(dir);

        for(const QString& file : plugDir.entryList(QDir::Files)) {
            if (T *plugin =
                    PLMPluginLoader::pluginByName<T>(plugDir.absoluteFilePath(file))) {
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

        filter << "*.so" << ".dll";
        QDir::Filters filterFlags(QDir::Files& ~QDir::Executable);

        for(const QString& file : plugDir.entryList(filter, filterFlags, QDir::NoSort)) {
            if (T *plugin =
                    PLMPluginLoader::pluginByName<T>(plugDir.absoluteFilePath(file)))
                if (plugin) {
                    ls.push_back(pluginObjectByName(plugDir.absoluteFilePath(file)));
                }
        }

        return ls;
    }

private:

    static PLMPluginLoader *m_instance;
    QHash<QString, PLMPlugin>m_pluginsListHash;
    QStringList m_pluginTypesList;
};


#endif // PLUGINLOADER_H

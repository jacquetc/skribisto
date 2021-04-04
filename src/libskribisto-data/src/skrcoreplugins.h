#ifndef PLMCOREPLUGINS_H
#define PLMCOREPLUGINS_H


#include "skrpluginloader.h"
#include "skrcoreinterface.h"

namespace SKRCorePlugins {
void addCorePlugins()
{
    SKRPluginLoader *loader = SKRPluginLoader::instance();

    loader->addPluginType<SKRCoreInterface>();

    // loader->addPluginType<PLMSideMainBarIconInterface>();
}
}


#endif // PLMGUIPLUGINS_H

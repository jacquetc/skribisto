#ifndef PLMCOREPLUGINS_H
#define PLMCOREPLUGINS_H


#include "plmpluginloader.h"
#include "plmcoreinterface.h"

namespace PLMCorePlugins {

void addCorePlugins()
{
    PLMPluginLoader *loader = PLMPluginLoader::instance();

    loader->addPluginType<PLMBaseInterface>();

    // loader->addPluginType<PLMSideMainBarIconInterface>();
}
}


#endif // PLMGUIPLUGINS_H

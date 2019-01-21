#ifndef PLMGUIPLUGINS_H
#define PLMGUIPLUGINS_H


#include "plmpluginloader.h"
#include "plmguiinterface.h"
#include "plmdockwidgetinterface.h"

namespace PLMGuiPlugins {
void addGuiPlugins()
{
    PLMPluginLoader *loader = PLMPluginLoader::instance();

    loader->addPluginType<PLMWindowInterface>();
    loader->addPluginType<PLMSideMainBarIconInterface>();
    loader->addPluginType<PLMDockWidgetInterface>();

    // loader->reload();
}
}


#endif // PLMGUIPLUGINS_H

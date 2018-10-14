#ifndef PLMGUIPLUGINS_H
#define PLMGUIPLUGINS_H


#include "plmpluginloader.h"
#include "plmguiinterface.h"

namespace PLMGuiPlugins
{

void addGuiPlugins()
{
    PLMPluginLoader *loader = PLMPluginLoader::instance();
    loader->addPluginType<PLMWindowInterface>();
    loader->addPluginType<PLMSideMainBarIconInterface>();
    //loader->reload();
}

}



#endif // PLMGUIPLUGINS_H

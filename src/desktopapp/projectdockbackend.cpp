#include "projectdockbackend.h"
#include "skrdata.h"

#include <interfaces/projecttoolboxinterface.h>

ProjectDockBackend::ProjectDockBackend(QObject *parent, Dock* dock)
    : QObject{parent}
{


    QList<ProjectToolboxInterface *> pluginList =
            skrpluginhub->pluginsByType<ProjectToolboxInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](ProjectToolboxInterface *plugin1, ProjectToolboxInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    QList<Toolbox*> toolboxes;
    for(auto *plugin : pluginList){
        toolboxes.append(plugin->getToolbox());
    }
    dock->setToolboxes(toolboxes);

    dock->stack()->setCurrentIndex(0);

}

#include "importer.h"
#include "interfaces/importerinterface.h"
#include "skrdata.h"
#include "project/plmproject.h"

Importer::Importer(QObject *parent)
    : QObject{parent}
{

}

void Importer::init()
{
    skrpluginhub->addPluginType<ImporterInterface>();

}

int Importer::importProject(const QUrl &url, const QString &extension, const QVariantMap &parameters, SKRResult &result)
{
int newProjectId = -1;


    QList<ImporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ImporterInterface>();

    for(auto *plugin : pluginList){
        if(plugin->extension() == extension){



             QString sqlDatabaseName = plugin->importProject(url, parameters, result);

             IFOK(result){
                 PLMProject *project = new PLMProject(plmProjectManager, &result, sqlDatabaseName, url);

                 result = plmProjectManager->loadProject(project);

                 newProjectId = project->id();
             }



        }
    }

    return newProjectId;
}

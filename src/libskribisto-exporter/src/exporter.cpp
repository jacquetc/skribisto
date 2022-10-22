#include "exporter.h"
#include "interfaces/exporterinterface.h"
#include "skrdata.h"

Exporter::Exporter(QObject *parent)
    : QObject{parent}
{

}

void Exporter::init()
{
    skrpluginhub->addPluginType<ExporterInterface>();

}

SKRResult Exporter::exportProject(int projectId, const QUrl &url, const QString &extension, const QVariantMap &parameters, QList<int> treeItemIds)
{
    SKRResult result("Exporter");

    PLMProject *project = plmProjectManager->project(projectId);

    if (!project) {
        result = SKRResult(SKRResult::Critical, "Exporter", "project_missing");
        result.addData("projectId", projectId);
        return result;
    }

    if (url.isEmpty()) {
        result = SKRResult(SKRResult::Critical, "Exporter" , "no_path");
        result.addData("projectId", projectId);
        return result;
    }


    QList<ExporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ExporterInterface>();

    for(auto *plugin : pluginList){
        if(plugin->extension() == extension){
            result = plugin->run(projectId, url, parameters, treeItemIds);
        }
    }

    return result;
}

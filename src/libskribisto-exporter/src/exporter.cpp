#include "exporter.h"
#include "interfaces/exporterinterface.h"
#include "skrdata.h"

#include <QTemporaryFile>

Exporter::Exporter(QObject *parent)
    : QObject{parent}
{

}

void Exporter::init()
{
    skrpluginhub->addPluginType<ExporterInterface>();

}

SKRResult Exporter::exportProject(int projectId, const QUrl &url, const QString &extension, const QVariantMap &parameters, QList<TreeItemAddress> treeItemAddresses)
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

    QList<TreeItemAddress> treeItemAddressesToExport;
    if(treeItemAddresses.isEmpty()){ // export all ids
        treeItemAddressesToExport = skrdata->treeHub()->getAllIds(projectId);

        QMutableListIterator<TreeItemAddress> i(treeItemAddressesToExport);
        while (i.hasNext()) { // remove trashed and non-printable
            TreeItemAddress treeItemAddress = i.next();
            if(skrdata->treeHub()->getTrashed(treeItemAddress) || skrdata->treePropertyHub()->getProperty(treeItemAddress, "printable", "true") != "true"){
                i.remove();
            }
        }
    }
    else {
        treeItemAddressesToExport = treeItemAddresses;
    }


    QList<ExporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ExporterInterface>();

    bool havePlugin = false;
    for(auto *plugin : pluginList){
        if(plugin->extensions().contains(extension)){
            result = plugin->run(treeItemAddressesToExport, url, extension, parameters );
            havePlugin = true;
            break;
        }
    }

    if(!havePlugin){
        result = SKRResult(SKRResult::Critical, "Exporter" , "no_plugin_found");
        result.addData("projectId", projectId);
        result.addData("extension", extension);
        return result;
    }

    return result;
}

//-----------------------------------------------------------------------------

QTextDocument *Exporter::getPrintTextDocument(QList<TreeItemAddress> treeItemAddresses, const QVariantMap &parameters, SKRResult *result)
{
    QTextDocument *textDocument = new QTextDocument();

    PLMProject *project = plmProjectManager->project(treeItemAddresses.first().projectId);

    if (!project) {
        *result = SKRResult(SKRResult::Critical, "Exporter", "project_missing");
        result->addData("projectId", treeItemAddresses.first().projectId);
        return textDocument;
    }

    QString extension = "html";
    QUrl url;
    QString fileName;
    QTemporaryFile tempFile;
    tempFile.setAutoRemove(false);
    if (tempFile.open()) {
        fileName = tempFile.fileName();
        url = QUrl::fromLocalFile(fileName);
        tempFile.close();
        }


        // write HTML using plugin

        QList<ExporterInterface *> pluginList =
                skrpluginhub->pluginsByType<ExporterInterface>();

        bool havePlugin = false;
        for(auto *plugin : pluginList){
            if(plugin->extensions().contains(extension)){
                *result = plugin->run(treeItemAddresses, url, extension, parameters);
                havePlugin = true;
                break;
            }
        }

        if(!havePlugin){
            *result = SKRResult(SKRResult::Critical, "Exporter" , "no_plugin_found");
            result->addData("projectId", treeItemAddresses.first().projectId);
            result->addData("extension", extension);
            return new QTextDocument();
        }


        QFile tempFileBis;
        tempFileBis.setFileName(fileName);
        tempFileBis.open(QIODevice::ReadOnly);
        // retrieve HTML
        QByteArray array(tempFileBis.readAll());

        textDocument->setHtml(array);

        tempFileBis.remove();


    return textDocument;

}

//-----------------------------------------------------------------------------

QString Exporter::getSaveFilter()
{

    QStringList filters;


    QList<ExporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ExporterInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](ExporterInterface *plugin1, ExporterInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){
        if(plugin->canSave()){
            for(int i = 0; i <  plugin->extensions().count() ; i++){

                const QString &extension = plugin->extensions().at(i);
                filters << QString("%1 (*.%2)").arg(plugin->extensionHumanNames().at(i), extension);
            }
        }
    }


    return filters.join(";;");
}

QStringList Exporter::getExportExtensionHumanNames()
{


    QStringList humanNames;


    QList<ExporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ExporterInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](ExporterInterface *plugin1, ExporterInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){
        if(!plugin->canSave()){
            for(int i = 0; i <  plugin->extensions().count() ; i++){

                const QString &extension = plugin->extensions().at(i);
                humanNames << plugin->extensionHumanNames().at(i);
            }
        }
    }


    return humanNames;

}
//-----------------------------------------------------------------------------


QList<QPair<QString, QString> > Exporter::getExportExtensions()
{

    QList<QPair<QString, QString> > exportExtensions;


    QList<ExporterInterface *> pluginList =
            skrpluginhub->pluginsByType<ExporterInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](ExporterInterface *plugin1, ExporterInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){
        if(!plugin->canSave()){
            for(int i = 0; i <  plugin->extensions().count() ; i++){

                const QString &extension = plugin->extensions().at(i);
                exportExtensions << QPair<QString, QString>(plugin->extensionShortNames().at(i), extension);
            }
        }
    }


    return exportExtensions;
}
//-----------------------------------------------------------------------------


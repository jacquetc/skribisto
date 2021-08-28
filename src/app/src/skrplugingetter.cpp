#include "skrplugingetter.h"
#include "skrdata.h"
#include "skrimporterinterface.h"
#include "skrsettingspanelinterface.h"

SKRPluginGetter::SKRPluginGetter(QObject *parent) : QObject(parent)
{
    skrdata->pluginHub()->addPluginType<SKRImporterInterface>();
    skrdata->pluginHub()->addPluginType<SKRSettingsPanelInterface>();
}

// --------------------------------------------------------------------------------------
// ------------Importer
// --------------------------------------------------------------------
// --------------------------------------------------------------------------------------

QStringList SKRPluginGetter::findImporterNames() const
{
    QList<SKRImporterInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRImporterInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](SKRImporterInterface *plugin1, SKRImporterInterface
                 *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
              );

    QStringList list;

    for (SKRImporterInterface *plugin: qAsConst(pluginList)) {
        list << plugin->name();
    }


    return list;
}

// ---------------------------------------------------------------------------------

QString SKRPluginGetter::findImporterIconSource(const QString& name) const
{
    QList<SKRImporterInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRImporterInterface>();


    QString icon;

    for (SKRImporterInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            icon = plugin->iconSource();
            break;
        }
    }


    return icon;
}

// ---------------------------------------------------------------------------

QString SKRPluginGetter::findImporterButtonText(const QString& name) const
{
    QList<SKRImporterInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRImporterInterface>();


    QString text;

    for (SKRImporterInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            text = plugin->buttonText();
            break;
        }
    }


    return text;
}

// ---------------------------------------------------------------------------------


QString SKRPluginGetter::findImporterUrl(const QString& name) const
{
    QList<SKRImporterInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRImporterInterface>();


    QString url;

    for (SKRImporterInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            url = plugin->qmlUrl();
        }
    }

    return url;
}

// --------------------------------------------------------------------------------------
// ------------SettingsPanel
// --------------------------------------------------------------------
// --------------------------------------------------------------------------------------

QStringList SKRPluginGetter::findSettingsPanelNames() const
{
    QList<SKRSettingsPanelInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRSettingsPanelInterface>();

    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](SKRSettingsPanelInterface *plugin1, SKRSettingsPanelInterface
                 *plugin2) -> bool {
        return plugin1->settingsPanelWeight() < plugin2->settingsPanelWeight();
    }
              );

    QStringList list;

    for (SKRSettingsPanelInterface *plugin: qAsConst(pluginList)) {
        list << plugin->name();
    }


    return list;
}

// ---------------------------------------------------------------------------------

QString SKRPluginGetter::findSettingsPanelIconSource(const QString& name) const
{
    QList<SKRSettingsPanelInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRSettingsPanelInterface>();


    QString icon;

    for (SKRSettingsPanelInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            icon = plugin->settingsPanelIconSource();
            break;
        }
    }


    return icon;
}

// ---------------------------------------------------------------------------------

QString SKRPluginGetter::findSettingsPanelButtonText(const QString& name) const
{
    QList<SKRSettingsPanelInterface *> pluginList =
        skrdata->pluginHub()->pluginsByType<SKRSettingsPanelInterface>();


    QString text;

    for (SKRSettingsPanelInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            text = plugin->settingsPanelButtonText();
            break;
        }
    }


    return text;
}

// ---------------------------------------------------------------------------------

QString SKRPluginGetter::findSettingsPanelUrl(const QString& name) const
{
    QList<SKRSettingsPanelInterface *> pluginList = skrdata->pluginHub()->pluginsByType<SKRSettingsPanelInterface>();


    QString url;

    for (SKRSettingsPanelInterface *plugin: qAsConst(pluginList)) {
        if (plugin->name() == name) {
            url = plugin->settingsPanelQmlUrl();
        }
    }

    return url;
}

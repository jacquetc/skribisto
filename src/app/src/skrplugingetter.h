#ifndef SKRPLUGINGETTER_H
#define SKRPLUGINGETTER_H

#include <QObject>

class SKRPluginGetter : public QObject {
    Q_OBJECT

public:

    explicit SKRPluginGetter(QObject *parent = nullptr);


    Q_INVOKABLE QStringList findImporterNames() const;
    Q_INVOKABLE QString     findImporterIconSource(const QString& name) const;
    Q_INVOKABLE QString     findImporterButtonText(const QString& name) const;
    Q_INVOKABLE QString     findImporterUrl(const QString& name) const;

    Q_INVOKABLE QStringList findSettingsPanelNames() const;
    Q_INVOKABLE QString     findSettingsPanelIconSource(const QString& name) const;
    Q_INVOKABLE QString     findSettingsPanelButtonText(const QString& name) const;
    Q_INVOKABLE QString     findSettingsPanelUrl(const QString& name) const;

signals:
};

#endif // SKRPLUGINGETTER_H

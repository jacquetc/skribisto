#include "skrusersettings.h"

#include <QDataStream>
#include <QIODevice>

SKRUserSettings::SKRUserSettings(QObject *parent) : QObject(parent)
{}

void SKRUserSettings::setProjectSetting(int projectId, const QString& key, QVariant value)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);

    setSetting("project_" + unique_identifier, key, value);

}

QVariant SKRUserSettings::getProjectSetting(int projectId, const QString& key,
                                       QVariant defaultValue)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);

    return getSetting("project_" + unique_identifier, key, defaultValue);

}

void SKRUserSettings::removeProjectSetting(int projectId, const QString& key)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);

    removeSetting("project_" + unique_identifier, key);
}

void SKRUserSettings::setSetting(const QString &group, const QString& key, QVariant value)
{

    QSettings settings;

    settings.beginGroup(group);
    settings.setValue(key, value);
    settings.endGroup();
}

QVariant SKRUserSettings::getSetting(const QString &group, const QString& key,
                                       QVariant defaultValue)
{
    QSettings settings;

    settings.beginGroup(group);
    QVariant value = settings.value(key, defaultValue);

    settings.endGroup();

    return value;
}

void SKRUserSettings::removeSetting(const QString &group, const QString& key)
{
    QSettings settings;

    settings.beginGroup(group);
    settings.remove(key);
    settings.endGroup();
}
QByteArray SKRUserSettings::serializingHash(const QHash<QString, QVariant>& hash) const
{
    QByteArray array;

    // Serializing
    QDataStream out(&array, QIODevice::WriteOnly); // write the data

    out << hash;
    return array;
}

QHash<QString, QVariant>SKRUserSettings::deserializingHash(QByteArray hashArray)
{
    // Deserializing
    // read the data serialized from the file
    QHash<QString, QVariant> hash;
    QDataStream in(&hashArray, QIODevice::ReadOnly);

    in >> hash;

    return hash;
}

void SKRUserSettings::insertInProjectSettingHash(int             projectId,
                                                 const QString & key,
                                                 const QString & hashKey,
                                                 const QVariant& value)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
insertInSomeGroupSettingHash("project_" + unique_identifier, key, hashKey, value);
}

QVariant SKRUserSettings::getFromProjectSettingHash(int             projectId,
                                                    const QString & key,
                                                    const QString & hashKey,
                                                    const QVariant& defaultValue)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    return getFromSomeGroupSettingHash("project_" + unique_identifier, key, hashKey, defaultValue);

}

void SKRUserSettings::removeFromProjectSettingHash(int            projectId,
                                                   const QString& key,
                                                   const QString& hashKey)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    removeFromSomeGroupSettingHash("project_" + unique_identifier, key, hashKey);

}

// --------------------------------------------------------------------

void SKRUserSettings::insertInSomeGroupSettingHash(const QString &settingGroup, const QString &key, const QString &hashKey, const QVariant &value)
{
    QSettings settings;

    settings.beginGroup(settingGroup);

    // Serializing default value
    QHash<QString, QVariant> newHash;
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    // Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    hash.insert(hashKey, value);

    // write back
    QByteArray newArray = serializingHash(hash);

    settings.setValue(key, newArray);

    settings.endGroup();
}

// --------------------------------------------------------------------

QVariant SKRUserSettings::getFromSomeGroupSettingHash(const QString &settingGroup, const QString &key, const QString &hashKey, const QVariant &defaultValue)
{
    QSettings settings;

    settings.beginGroup(settingGroup);

    // Serializing default value
    QHash<QString, QVariant> newHash;
    newHash.insert(hashKey, defaultValue);
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    // Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    settings.endGroup();

    return hash.value(hashKey, defaultValue);
}

// --------------------------------------------------------------------

void SKRUserSettings::removeFromSomeGroupSettingHash(const QString &settingGroup, const QString &key, const QString &hashKey)
{
    QSettings settings;

    settings.beginGroup(settingGroup);

    // Serializing default value
    QHash<QString, QVariant> newHash;
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    // Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    hash.remove(hashKey);

    // write back
    QByteArray newArray = serializingHash(hash);

    settings.setValue(key, newArray);

    settings.endGroup();
}

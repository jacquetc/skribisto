#include "skrusersettings.h"

#include <QDataStream>
#include <QIODevice>

SkrUserSettings::SkrUserSettings(QObject *parent) : QObject(parent)
{


}

void SkrUserSettings::setProjectSetting(int projectId, const QString &key, int value)
{
    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);
    settings.setValue(key, value);
    settings.endGroup();
}

int SkrUserSettings::getProjectSetting(int projectId, const QString &key, int defaultValue)
{
    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);
    int value = settings.value(key, defaultValue).toInt();
    settings.endGroup();

    return value;
}

void SkrUserSettings::removeProjectSetting(int projectId, const QString &key)
{

    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);
    settings.remove(key);
    settings.endGroup();
}

QByteArray SkrUserSettings::serializingHash(const QHash<QString, QVariant> &hash) const
{
    QByteArray array;

    //Serializing
    QDataStream out(&array, QIODevice::WriteOnly);   // write the data
    out << hash;
    return array;
}

QHash<QString, QVariant> SkrUserSettings::deserializingHash(QByteArray hashArray)
{
    //Deserializing
    // read the data serialized from the file
    QHash<QString, QVariant> hash;
    QDataStream in(&hashArray, QIODevice::ReadOnly);
    in >> hash;

    return hash;
}
void SkrUserSettings::insertInProjectSettingHash(int projectId, const QString &key, const QString &hashKey, const QVariant &value)
{


    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);

    //Serializing default value
    QHash<QString, QVariant> newHash;
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    //Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    hash.insert(hashKey, value);

    // write back
    QByteArray newArray = serializingHash(hash);
    settings.setValue(key, newArray);

    settings.endGroup();
}

QVariant SkrUserSettings::getFromProjectSettingHash(int projectId, const QString &key, const QString &hashKey, const QVariant &defaultValue)
{
    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);

    //Serializing default value
    QHash<QString, QVariant> newHash;
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    //Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    return hash.value(hashKey, defaultValue);

    settings.endGroup();
}

void SkrUserSettings::removeFromProjectSettingHash(int projectId, const QString &key, const QString &hashKey)
{


    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);

    //Serializing default value
    QHash<QString, QVariant> newHash;
    QByteArray array = serializingHash(newHash);

    QByteArray hashArray = settings.value(key, array).toByteArray();

    //Deserializing
    QHash<QString, QVariant> hash = deserializingHash(hashArray);

    hash.remove(hashKey);

    // write back
    QByteArray newArray = serializingHash(newHash);
    settings.setValue(key, newArray);

    settings.endGroup();
}

//--------------------------------------------------------------------

#include "result.h"
#include <QMetaProperty>
#include <QSqlQuery>
#include <QStringList>

#pragma once

namespace Database
{

template <class T> class Tools
{
  public:
    /**
     * @brief getEntityClassName returns the name of the class associated with this database.
     * @return The class name as a QString.
     */
    static QString getEntityClassName();
    static QString getEntityTableName();
    static QString getTableNameFromClassName(const QString &className);

    /**
     * @brief getEntityProperties returns the list of properties associated with the class
     * associated with this database.
     * @return A QStringList containing the property names.
     */
    static QStringList getEntityProperties();
    static QVariant getEntityPropertyValue(const T &entity, const QString &propertyName);
    static void setEntityPropertyValue(T &entity, const QString &propertyName, const QVariant &propertyValue);

    /**
     * @brief Maps a hash of field names to their corresponding values to an entity of type T.
     * @param fieldWithValue The hash of field names to their corresponding values to be mapped.
     * @return Result<T> The result of the mapping operation, containing either the mapped entity of type T or an error.
     */
    static Result<T> mapToEntity(const QHash<QString, QVariant> &valuesHash);
    static void readEntityFromQuery(T &entity, const QSqlQuery &query);
    static void bindValueFromEntityToQuery(const T &entity, QSqlQuery &query);
    static QString fromPascalToSnakeCase(const QString &string);
    static QString fromSnakeCaseToPascalCase(const QString &string);
    static QString fromSnakeCaseToCamelCase(const QString &string);
};

//--------------------------------------------

template <class T> QString Tools<T>::getEntityClassName()
{

    const QMetaObject &sourceMetaObject = T::staticMetaObject;
    return QString(sourceMetaObject.className()).split("::").last();
}

//--------------------------------------------

template <class T> QString Tools<T>::getEntityTableName()
{
    QString className = Tools<T>::getEntityClassName();
    return getTableNameFromClassName(className);
}

//--------------------------------------------

template <class T> QString Tools<T>::getTableNameFromClassName(const QString &className)
{
    return Tools<T>::fromPascalToSnakeCase(className.split("::").last());
}

//--------------------------------------------

template <class T> QStringList Tools<T>::getEntityProperties()
{
    QStringList propertyList;

    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            if (property.name() == QString("objectName"))
            {
                continue;
            }
            propertyList.append(property.name());
        }
    }

    return propertyList;
}

//--------------------------------------------

template <class T> QVariant Tools<T>::getEntityPropertyValue(const T &entity, const QString &propertyName)
{
    QVariant propertyValue;

    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            if (property.name() == QString("objectName"))
            {
                continue;
            }
            if (property.name() == propertyName)
            {
                propertyValue = property.read(&entity);
                break;
            }
        }
    }

    return propertyValue;
}

template <class T>
void Tools<T>::setEntityPropertyValue(T &entity, const QString &propertyName, const QVariant &propertyValue)
{
    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isWritable())
        {
            if (property.name() == QString("objectName"))
            {
                continue;
            }
            if (property.name() == propertyName)
            {
                property.write(&entity, propertyValue);
                break;
            }
        }
    }
}

//--------------------------------------------

template <class T> Result<T> Tools<T>::mapToEntity(const QHash<QString, QVariant> &valuesHash)
{
    T entity;
    const QMetaObject *metaObject = entity.metaObject();

    QHash<QString, QVariant>::const_iterator i = valuesHash.constBegin();
    while (i != valuesHash.constEnd())
    {

        QString columnName = i.key();
        QString propertyName = Tools<T>::fromSnakeCaseToCamelCase(columnName);

        int destinationPropertyIndex = metaObject->indexOfProperty(propertyName.toLatin1());
        if (destinationPropertyIndex >= 0)
        {
            QVariant value = i.value();
            QMetaProperty destinationProperty = metaObject->property(destinationPropertyIndex);

            if (destinationProperty.isWritable() &&
                QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
            {
                bool success = destinationProperty.write(&entity, value);
                if (!success)
                {
                    Result<T>(Error(Q_FUNC_INFO, Error::Fatal, "map_write_failed",
                                    "Failed to write value to destination property", propertyName));
                }
            }
        }
        else
        {
            Result<T>(Error(Q_FUNC_INFO, Error::Fatal, "map_missing_property", "Missing property in destination object",
                            propertyName));
        }
        ++i;
    }
    return Result<T>(entity);
}

//--------------------------------------------

template <class T> void Tools<T>::readEntityFromQuery(T &entity, const QSqlQuery &query)
{
    const QStringList &properties = getEntityProperties();
    for (int i = 0; i < properties.count(); i++)
    {
        const QString &property = properties.at(i);
        QVariant value = query.value(i);
        QByteArray truePropertyName = property.toLatin1();
        if (!entity.setProperty(truePropertyName, value))
        {

            qCritical() << "setting property " << truePropertyName << "failed on" << getEntityClassName();
        }
    }
}

//--------------------------------------------

template <class T> QString Tools<T>::fromPascalToSnakeCase(const QString &string)
{
    QString finalString;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character.isUpper())
        {
            if (i != 0)
            {
                finalString.append("_");
            }
            finalString.append(character.toLower());
        }
        else
        {
            finalString.append(character);
        }
    }
    return finalString;
}

//--------------------------------------------

template <class T> QString Tools<T>::fromSnakeCaseToPascalCase(const QString &string)
{
    QString finalString;
    bool next_letter_must_be_upper = false;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character == '_')
        {
            next_letter_must_be_upper = true;
            continue;
        }
        else if (next_letter_must_be_upper || i == 0)
        {
            finalString.append(character.toUpper());
            next_letter_must_be_upper = false;
        }
        else
        {
            finalString.append(character.toLower());
        }
    }
    return finalString;
}
//--------------------------------------------

template <class T> QString Tools<T>::fromSnakeCaseToCamelCase(const QString &string)
{
    QString finalString;
    bool next_letter_must_be_upper = false;
    for (int i = 0; i < string.size(); i++)
    {
        const QChar &character = string.at(i);
        if (character == '_')
        {
            next_letter_must_be_upper = true;
            continue;
        }
        else if (next_letter_must_be_upper)
        {
            finalString.append(character.toUpper());
            next_letter_must_be_upper = false;
        }
        else
        {
            finalString.append(character.toLower());
        }
    }
    return finalString;
}
} // namespace Database

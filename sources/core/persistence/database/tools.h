#include "result.h"
#include <QMetaProperty>
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

    /**
     * @brief getEntityProperties returns the list of properties associated with the class
     * associated with this database.
     * @return A QStringList containing the property names.
     */
    static QStringList getEntityProperties();
    /**
     * @brief Maps a hash of field names to their corresponding values to an entity of type T.
     * @param fieldWithValue The hash of field names to their corresponding values to be mapped.
     * @return Result<T> The result of the mapping operation, containing either the mapped entity of type T or an error.
     */
    static Result<T> mapToEntity(const QHash<QString, QVariant> &fieldWithValue);
};

//--------------------------------------------

template <class T> QString Tools<T>::getEntityClassName()
{

    const QMetaObject &sourceMetaObject = T::staticMetaObject;

    return QString(sourceMetaObject.className()).split("::").last();
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

template <class T> Result<T> Tools<T>::mapToEntity(const QHash<QString, QVariant> &fieldWithValue)
{
    T entity;
    const QMetaObject *metaObject = entity.metaObject();

    QHash<QString, QVariant>::const_iterator i = fieldWithValue.constBegin();
    while (i != fieldWithValue.constEnd())
    {

        QString propertyName = i.key();

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
                    Result<T>(Error("SkribFile", Error::Fatal, "map_write_failed",
                                    "Failed to write value to destination property", propertyName));
                }
            }
        }
        else
        {
            Result<T>(Error("SkribFile", Error::Fatal, "map_missing_property", "Missing property in destination object",
                            propertyName));
        }
        ++i;
    }
    return Result<T>(entity);
}

} // namespace Database

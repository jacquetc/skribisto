#pragma once

#include "contracts_global.h"
#include <QDateTime>
#include <QVariant>
#include <QtCore/QMetaObject>
#include <QtCore/QMetaProperty>

namespace AutoMapper
{
class SKR_CONTRACTS_EXPORT AutoMapper
{
  public:
    template <class Destination> static Destination map(const QObject &sourceObject)
    {
        Destination destinationObject;
        mapImpl<Destination>(sourceObject, destinationObject, std::is_pointer<QObject>{},
                             std::is_pointer<Destination>{});
        return destinationObject;
    }

    template <class Destination> static Destination map(QObject &sourceObject)
    {

        Destination destinationObject;
        mapImpl<Destination>(sourceObject, destinationObject, std::is_pointer<QObject>{},
                             std::is_pointer<Destination>{});
        return destinationObject;
    }

  private:
    template <class Destination>
    static typename std::enable_if<!std::is_pointer<QObject>::value && !std::is_pointer<Destination>::value>::type
    mapImpl(const QObject &sourceObject, Destination &destinationObject, std::false_type, std::false_type)
    {

        const QMetaObject *sourceMetaObject = sourceObject.metaObject();
        const QMetaObject *destinationMetaObject = destinationObject.metaObject();

        int propertyCount = sourceMetaObject->propertyCount();

        for (int i = 0; i < propertyCount; ++i)
        {
            QMetaProperty sourceProperty = sourceMetaObject->property(i);
            if (sourceProperty.isReadable())
            {
                if (sourceProperty.name() == QString("objectName"))
                {
                    continue;
                }
                int destinationPropertyIndex = destinationMetaObject->indexOfProperty(sourceProperty.name());
                if (destinationPropertyIndex >= 0)
                {
                    QVariant value = sourceProperty.read(&sourceObject);
                    QMetaProperty destinationProperty = destinationMetaObject->property(destinationPropertyIndex);

                    if (destinationProperty.isWritable() &&
                        QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
                    {
                        bool success = destinationProperty.write(&destinationObject, value);
                        if (!success)
                        {
                            qWarning() << "Failed to write value" << value << "to destination property"
                                       << destinationProperty.name();
                        }
                    }
                    else
                    {

                        qFatal("AutoMapper error: Property types do not match. Check the types of the source and "
                               "destination properties.");
                    }
                }
                //                else
                //                {
                //                    qFatal("AutoMapper error: Property names do not match. Check the names of the
                //                    source and "
                //                           "destination properties.");
                //                    // qWarning() << "Missing property" << sourceProperty.name() << "in destination
                //                    object";
                //                }
            }
        }
    }

    template <typename DestinationType>
    typename std::enable_if<std::is_pointer<QObject>::value ||
                            std::is_pointer<DestinationType>::value>::type static mapImpl(const QObject *source,
                                                                                          DestinationType &destination,
                                                                                          std::true_type,
                                                                                          std::true_type)
    {
        if (source == nullptr || &destination == nullptr)
        {
            return;
        }
        mapImpl<DestinationType>(*source, *destination, std::false_type{}, std::false_type{});
    }
};

} // namespace AutoMapper


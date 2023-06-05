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
    template <class DestinationType, class SourceType> static DestinationType map(const SourceType &sourceObject)
    {
        DestinationType destinationObject;
        mapImpl<DestinationType>(sourceObject, destinationObject, std::is_pointer<SourceType>{},
                                 std::is_pointer<DestinationType>{});
        return destinationObject;
    }

    template <class DestinationType, class SourceType> static DestinationType map(SourceType &sourceObject)
    {

        DestinationType destinationObject;
        mapImpl<DestinationType>(sourceObject, destinationObject, std::is_pointer<SourceType>{},
                                 std::is_pointer<DestinationType>{});
        return destinationObject;
    }

  private:
    template <class DestinationType, class SourceType>
    static typename std::enable_if<!std::is_pointer<QObject>::value && !std::is_pointer<DestinationType>::value>::type
    mapImpl(const SourceType &sourceObject, DestinationType &destinationObject, std::false_type, std::false_type)
    {

        const QMetaObject &sourceMetaObject = SourceType::staticMetaObject;
        const QMetaObject &destinationMetaObject = DestinationType::staticMetaObject;

        int propertyCount = sourceMetaObject.propertyCount();

        for (int i = 0; i < propertyCount; ++i)
        {
            QMetaProperty sourceProperty = sourceMetaObject.property(i);
            if (sourceProperty.isReadable())
            {
                int destinationPropertyIndex = destinationMetaObject.indexOfProperty(sourceProperty.name());
                if (destinationPropertyIndex >= 0)
                {
                    QVariant value = sourceProperty.readOnGadget(&sourceObject);
                    QMetaProperty destinationProperty = destinationMetaObject.property(destinationPropertyIndex);

                    if (destinationProperty.isWritable() &&
                        QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
                    {
                        bool success = destinationProperty.writeOnGadget(&destinationObject, value);
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

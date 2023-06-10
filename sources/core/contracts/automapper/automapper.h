#pragma once

#include "contracts_global.h"
#include <QDateTime>
#include <QHash>
#include <QMutexLocker>
#include <QVariant>
#include <QtCore/QMetaObject>
#include <QtCore/QMetaProperty>


namespace AutoMapper {
class SKR_CONTRACTS_EXPORT AutoMapper {
public:

    template<class SourceType, class DestinationType>
    static void registerMapping(bool reverseConversion = false) {
        QMutexLocker locker(&mutex);

        qRegisterMetaType<SourceType>();
        qRegisterMetaType<DestinationType>();


        getSiblingFunctions[QMetaType::fromType<SourceType>()] =
            []() {
                return QVariant::fromValue(DestinationType());
            };

        getSiblingFunctions[QMetaType::fromType<QList<SourceType> >()] =
            []() {
                return QVariant::fromValue(DestinationType());
            };


        siblingMetaTypeHash[QMetaType::fromType<SourceType>()]         = QMetaType::fromType<DestinationType>();
        siblingMetaTypeHash[QMetaType::fromType<QList<SourceType> >()] = QMetaType::fromType<QList<DestinationType> >();


        writerHash[QMetaType::fromType<SourceType>()] =
            [&](const QMetaProperty& destinationProperty, void *gadgetPointer, const QVariant& value) {
                return destinationProperty.writeOnGadget(gadgetPointer,
                                                         QVariant::fromValue(value.value<DestinationType>()));
            };

        writerForListHash[QMetaType::fromType<QList<SourceType> >()] =
            [](const QMetaProperty& destinationProperty, void *gadgetPointer, const QList<QVariant>& sourceList) {
                QList<DestinationType> destinationList;

                for (const QVariant& value : sourceList) {
                    const QVariant& customDestinationObject = AutoMapper::implMap(value);
                    destinationList.append(customDestinationObject.value<DestinationType>());
                }

                auto variant = QVariant::fromValue(destinationList);
                return destinationProperty.writeOnGadget(gadgetPointer, variant);
            };

        if (reverseConversion) {
            getSiblingFunctions[QMetaType::fromType<DestinationType>()] =
                []() {
                    return QVariant::fromValue(SourceType());
                };

            getSiblingFunctions[QMetaType::fromType<QList<DestinationType> >()] =
                []() {
                    return QVariant::fromValue(SourceType());
                };


            siblingMetaTypeHash[QMetaType::fromType<DestinationType>()]         = QMetaType::fromType<SourceType>();
            siblingMetaTypeHash[QMetaType::fromType<QList<DestinationType> >()] =
                QMetaType::fromType<QList<SourceType> >();


            writerHash[QMetaType::fromType<DestinationType>()] =
                [](const QMetaProperty& destinationProperty, void *gadgetPointer, const QVariant& value) {
                    return destinationProperty.writeOnGadget(gadgetPointer,
                                                             QVariant::fromValue(value.value<SourceType>()));
                };

            writerForListHash[QMetaType::fromType<QList<DestinationType> >()] =
                [](const QMetaProperty& destinationProperty, void *gadgetPointer, const QList<QVariant>& sourceList) {
                    QList<SourceType> destinationList;

                    for (const QVariant& value : sourceList) {
                        const QVariant& customDestinationObject = AutoMapper::implMap(value);
                        destinationList.append(customDestinationObject.value<SourceType>());
                    }
                    auto variant = QVariant::fromValue(destinationList);
                    return destinationProperty.writeOnGadget(gadgetPointer, variant);
                };
        }
    }

    template<class SourceType, class DestinationType>
    static DestinationType map(const SourceType& sourceObject)
    {
        const QVariant& destinationVariant = implMap(QVariant::fromValue(sourceObject));

        return destinationVariant.value<DestinationType>();
    }

    template<class SourceType, class DestinationType>
    static DestinationType map(SourceType& sourceObject)
    {
        const QVariant& destinationVariant = implMap(QVariant::fromValue(sourceObject));

        return destinationVariant.value<DestinationType>();
    }

private:

    static QVariant implMap(const QVariant& source) {
        auto getSiblingFunction = getSiblingFunctions.find(source.metaType());

        if (getSiblingFunction == getSiblingFunctions.end()) {
            qWarning() << "No mapping found for type" << source.typeName();
        }

        QVariant destination = getSiblingFunction.value()();


        const QMetaObject *sourceMetaObject      = source.metaType().metaObject();
        const QMetaObject *destinationMetaObject = destination.metaType().metaObject();

        // convert to actual type
        const void *sourcePointer = source.data();
        void *destinationPointer  = destination.data();


        int propertyCount = sourceMetaObject->propertyCount();

        for (int i = 0; i < propertyCount; ++i)
        {
            QMetaProperty sourceProperty = sourceMetaObject->property(i);

            if (sourceProperty.isReadable())
            {
                int destinationPropertyIndex = destinationMetaObject->indexOfProperty(sourceProperty.name());

                if (destinationPropertyIndex >= 0)
                {
                    const QVariant& value                    = sourceProperty.readOnGadget(sourcePointer);
                    const QMetaProperty& destinationProperty =
                        destinationMetaObject->property(destinationPropertyIndex);

                    QVariant destinationValue;

                    if (destinationProperty.isWritable() &&
                        QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
                    {
                        bool success = destinationProperty.writeOnGadget(destinationPointer, value);

                        if (!success)
                        {
                            qWarning() << "Failed to write value" << value << "to destination property"
                                       << destinationProperty.name();
                        }
                    }

                    else if (destinationProperty.isWritable()) {
                        // Check if a conversion function exists for this
                        // property type
                        auto getSiblingFunction = getSiblingFunctions.find(value.metaType());


                        if (getSiblingFunction != getSiblingFunctions.end()) {
                            // We have a conversion for this type.
                            if (QString(value.metaType().name()).startsWith("QList<")) {
                                // If it's a QList<QVariant>, process each
                                // QVariant.
                                QList<QVariant> sourceList = value.toList();

                                if (sourceList.isEmpty()) {
                                    // If the list is empty, we can't get the
                                    // type of the custom type.
                                    // So we can't instantiate a new object of
                                    // the custom type.
                                    // So we can't call mapImpl recursively.
                                    // So we can't convert the list.
                                    // So we can't do anything.
                                    // So we just return an empty list.
                                    continue;
                                }


                                // destinationValue = foreignDestinationList;

                                auto writeForListIt = writerForListHash.find(value.metaType());

                                if (writeForListIt != writerForListHash.end()) {
                                    bool success = writeForListIt.value()(destinationProperty,
                                                                          destinationPointer, sourceList);

                                    if (!success)
                                    {
                                        qWarning() << "Failed to write value" << destinationValue <<
                                            "to destination property"
                                                   << destinationProperty.name();
                                    }
                                }
                            } else {
                                // It's a single QVariant.
                                auto customIt          = getSiblingFunctions.find(value.metaType());
                                auto siblingMetaTypeIt = siblingMetaTypeHash.find(value.metaType());

                                if (customIt != getSiblingFunctions.end()) {
                                    auto customDestinationObject = AutoMapper::implMap(value);
                                    destinationValue = customDestinationObject;

                                    auto writeFunction = writerHash.find(destinationProperty.metaType());

                                    if (writeFunction != writerHash.end()) {
                                        bool success = writeFunction.value()(destinationProperty,
                                                                             destinationPointer,
                                                                             destinationValue);

                                        if (!success)
                                        {
                                            qWarning() << "Failed to write value" << destinationValue <<
                                                "to destination property"
                                                       << destinationProperty.name();
                                        }
                                    }
                                }
                            }
                        }
                    }

                    else
                    {
                        qCritical("AutoMapper error: Property types do not match. Check the types of the source and "
                                  "destination properties.");
                    }
                }
            }
        }
        return destination;
    }

private:

    static QHash<QMetaType, std::function<QVariant()> >getSiblingFunctions;
    static QHash<QMetaType, QMetaType>siblingMetaTypeHash;
    static QHash<QMetaType, std::function<bool(const QMetaProperty&  destinationProperty, void *gadgetPointer,
                                               const QVariant& value)> >writerHash;
    static QHash<QMetaType, std::function<bool(const QMetaProperty&  destinationProperty, void *gadgetPointer,
                                               const QList<QVariant>& sourceList)> >writerForListHash;


    static QMutex mutex;
};
} // namespace AutoMapper

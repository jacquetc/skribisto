#include <QDateTime>
#include <QJSValue>
#include <QMetaProperty>
#include <QObject>
#include <QUuid>
#include <QUrl>

#pragma once



template<typename T>
T mapToDto(const QJSValue &jsValue) {
    T dto;
    QObject *jsObject = jsValue.toQObject();
    const QMetaObject *metaobject = jsObject->metaObject();
    int count = metaobject->propertyCount();

    // Define QMap with conversion functions
    QMap<QString, std::function<void(const char*, QVariant)>> conversionMap;

    conversionMap["bool"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toBool());
    };

    conversionMap["QUuid"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toUuid());
    };

    conversionMap["QString"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toString());
    };

    conversionMap["int"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toInt());
    };

    conversionMap["double"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toDouble());
    };

    conversionMap["QUrl"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toUrl());
    };

    conversionMap["QDateTime"] = [&](const char* name, QVariant value) {
        int propertyIndex = T::staticMetaObject.indexOfProperty(name);
        T::staticMetaObject.property(propertyIndex).writeOnGadget(&dto, value.toDateTime());
    };

    // add more conversion functions for other property types...

    for (int i=0; i<count; ++i) {
        QMetaProperty metaproperty = metaobject->property(i);
        const char *name = metaproperty.name();
        QVariant value = jsObject->property(name);

        QString typeName = QString::fromUtf8(metaproperty.typeName());

        // Check if we have a conversion function for this type
        if (conversionMap.contains(typeName)) {
            // Convert and set property
            conversionMap[typeName](name, value);
        }
    }

    return dto;
}

#include <QDateTime>
#include <QJSValue>
#include <QMetaProperty>
#include <QObject>

#pragma once



template<typename T>
T mapToDto(const QJSValue &jsValue) {
    T dto;
    QObject *jsObject = jsValue.toQObject();
    const QMetaObject *metaobject = jsObject->metaObject();
    int count = metaobject->propertyCount();

    // Define QMap with conversion functions
    QMap<QString, std::function<void(const char*, QVariant)>> conversionMap;

    conversionMap["QString"] = [&](const char* name, QVariant value) {
        dto.setProperty(name, value.toString());
    };

    conversionMap["int"] = [&](const char* name, QVariant value) {
        dto.setProperty(name, value.toInt());
    };

    conversionMap["QDateTime"] = [&](const char* name, QVariant value) {
        dto.setProperty(name, value.toDateTime());
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

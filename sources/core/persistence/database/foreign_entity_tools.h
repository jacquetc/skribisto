#include "tools.h"
#include <QString>

#pragma once

namespace Database
{

template <class T> class ForeignEntityTools
{
  public:
    static QString getRelationshipTableName(const QString &propertyName);
    static QString getRelationshipEntityIdColumnName();
    static QString getRelationshipOtherEntityIdColumnName(const QString &otherEntityClassName);
    static QStringList getRelationshipTableColumns(const QString &otherEntityClassName);

    static QString getOtherEntityTableName(const QString &propertyType);
    static QString getOtherEntityClassName(const QString &propertyType);

    static bool isList(const QStringList &entityClassNames, const QString &propertyType);
    static bool isUnique(const QStringList &entityClassNames, const QString &propertyType);
    static bool isForeign(const QStringList &entityClassNames, const QString &propertyType);
    static bool isEntity(const QStringList &entityClassNames, const QString &propertyType);

    static bool isEntityPropertyLoaded(const T &entity, const QString &propertyName);
    static void resetEntityProperty(T &entity, const QString &propertyName);
};

//--------------------------------------------

//--------------------------------------------

template <class T> QString ForeignEntityTools<T>::getRelationshipTableName(const QString &propertyName)
{
    QString entityClassName = Tools<T>::getEntityTableName();
    QString relationshipTableName = entityClassName + "_" + propertyName + "_relationship";
    return Tools<T>::fromPascalToSnakeCase(relationshipTableName);
}

//--------------------------------------------

template <class T> QString ForeignEntityTools<T>::getRelationshipEntityIdColumnName()
{
    return Tools<T>::fromPascalToSnakeCase(Tools<T>::getEntityClassName().left(1).toLower() +
                                           Tools<T>::getEntityClassName().mid(1) + "Id");
}

//--------------------------------------------

template <class T>
QString ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(const QString &otherEntityClassName)
{
    return Tools<T>::fromPascalToSnakeCase(otherEntityClassName.left(1).toLower() + otherEntityClassName.mid(1) + "Id");
}

//--------------------------------------------

template <class T> QStringList ForeignEntityTools<T>::getRelationshipTableColumns(const QString &otherEntityClassName)
{
    QStringList columns;
    columns << "id" << getRelationshipEntityIdColumnName()
            << getRelationshipOtherEntityIdColumnName(otherEntityClassName) << "previous"
            << "next";
}

//--------------------------------------------

template <class T> QString ForeignEntityTools<T>::getOtherEntityTableName(const QString &propertyType)
{

    QString tableName;

    if (propertyType.contains("<"))
    { // QList or Qset
        QString className =
            propertyType.mid(propertyType.indexOf('<') + 1, propertyType.indexOf('>') - propertyType.indexOf('<') - 1);
        className = className.split("::").last();
        tableName = Tools<T>::getTableNameFromClassName(className);
    }
    else
    { // Unique
        tableName = Tools<T>::getTableNameFromClassName(propertyType);
        tableName = tableName.split("::").last();
    }

    return tableName;
}

//--------------------------------------------

///
/// \brief ForeignEntityTools::getOtherEntityClassName
/// \param propertyType
/// \return
/// The namespace will be stripped.
template <class T> QString ForeignEntityTools<T>::getOtherEntityClassName(const QString &propertyType)
{
    QString className;

    if (propertyType.contains("<"))
    { // QList or Qset
        className =
            propertyType.mid(propertyType.indexOf('<') + 1, propertyType.indexOf('>') - propertyType.indexOf('<') - 1);
        className = className.split("::").last();
    }
    else
    { // Unique
        className = propertyType;
        className = className.split("::").last();
    }

    return className;
}

//--------------------------------------------

template <class T> bool ForeignEntityTools<T>::isList(const QStringList &entityClassNames, const QString &propertyType)
{
    QString className =
        propertyType.mid(propertyType.indexOf('<') + 1, propertyType.indexOf('>') - propertyType.indexOf('<') - 1)
            .split("::")
            .last();
    return propertyType.startsWith("QList<") && isEntity(entityClassNames, className);
}

//--------------------------------------------

template <class T>
bool ForeignEntityTools<T>::isUnique(const QStringList &entityClassNames, const QString &propertyType)
{
    return isEntity(entityClassNames, propertyType.split("::").last());
}

//--------------------------------------------

template <class T>
bool ForeignEntityTools<T>::isEntity(const QStringList &entityClassNames, const QString &propertyType)
{
    return entityClassNames.contains(propertyType);
}

//--------------------------------------------

template <class T>
bool ForeignEntityTools<T>::isForeign(const QStringList &entityClassNames, const QString &propertyType)
{
    return isList(entityClassNames, propertyType) || isUnique(entityClassNames, propertyType);
}

//--------------------------------------------

template <class T> bool ForeignEntityTools<T>::isEntityPropertyLoaded(const T &entity, const QString &propertyName)
{
    return Tools<T>::getEntityPropertyValue(entity, propertyName + "Loaded").toBool();
}

//--------------------------------------------

template <class T> void ForeignEntityTools<T>::resetEntityProperty(T &entity, const QString &propertyName)
{

    Tools<T>::setEntityPropertyValue(entity, propertyName + "Loaded", false);
    Tools<T>::setEntityPropertyValue(entity, propertyName, QVariant());
}
//--------------------------------------------

} // namespace Database

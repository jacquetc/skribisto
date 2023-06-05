#include "entity.h"
#include "foreign_entity.h"
#include "foreign_entity_tools.h"
#include "tools.h"
#include <QMetaObject>
#include <QMetaProperty>
#include <QString>

#pragma once

namespace Database
{

class EntityTableSqlGenerator
{
  public:
    EntityTableSqlGenerator(const QStringList &entityClassNames);

    template <class T> QStringList generateEntitySql();

  private:
    template <class T> QString generateMainTableSql();
    template <class T> QStringList generateRelationshipTablesSql();
    template <class T> QStringList generateIndexes();
    static const char *qtMetaTypeToSqlType(int qtMetaType);

    QStringList m_entityClassNames;
};

template <class T> QStringList EntityTableSqlGenerator::generateEntitySql()
{

    static_assert(std::is_base_of<Domain::Entity, T>::value, "T must inherit from Domain::Entity");

    QStringList finalSqlList;
    finalSqlList.append(EntityTableSqlGenerator::generateMainTableSql<T>());
    finalSqlList.append(EntityTableSqlGenerator::generateRelationshipTablesSql<T>());

    finalSqlList.append(EntityTableSqlGenerator::generateIndexes<T>());

    return finalSqlList;
}

template <class T> QString EntityTableSqlGenerator::generateMainTableSql()
{

    static_assert(std::is_base_of<Domain::Entity, T>::value, "T must inherit from Domain::Entity");

    QString tableName = Tools<T>::getEntityTableName();

    QString createTableSql = QString("CREATE TABLE %1 (").arg(tableName);

    QStringList relationshipPropertyNameListToIgnore;

    int propertyCount = T::staticMetaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {

        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        // ignore QList and QSet properties

        if (property.isReadable())
        {
            QString typeName = property.typeName();

            if (ForeignEntityTools<T>::isForeign(m_entityClassNames, typeName))
            {

                relationshipPropertyNameListToIgnore.append(propertyName);
                relationshipPropertyNameListToIgnore.append(QString(propertyName) + "Loaded");
            }
        }
    }

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        // Ignore the "objectName" property
        if (strcmp(propertyName, "objectName") == 0)
        {
            continue;
        }

        if (relationshipPropertyNameListToIgnore.contains(property.name()))
        {
            continue;
        }

        int propertyMetaType = property.userType();
        const char *propertySqlType = EntityTableSqlGenerator::qtMetaTypeToSqlType(propertyMetaType);

        if (propertySqlType)
        {
            createTableSql.append(QString("%1 %2").arg(Tools<T>::fromPascalToSnakeCase(propertyName), propertySqlType));

            // Set uuid property as primary key, not null, and unique
            if (strcmp(propertyName, "id") == 0)
            {
                createTableSql.append(" PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT"
                                      " UNIQUE ON CONFLICT ROLLBACK"
                                      " NOT NULL ON CONFLICT ROLLBACK");
            }

            createTableSql.append(", ");
        }
        else
        {
            // Handle the case when an unsupported type is encountered
            QMetaType metaType(static_cast<QMetaType::Type>(propertyMetaType));
            qWarning("Unsupported property type for '%s': %s", propertyName, metaType.name());
        }
    }
    // remove last comma
    createTableSql.chop(2);

    // Add foreign key to ordering table if T is an OrderedEntity
    //    if (std::is_base_of<Domain::OrderedEntity, T>::value)
    //    {
    //        createTableSql.append(
    //            QString(", ordering_id INTEGER, FOREIGN KEY (ordering_id) REFERENCES %1 (ordering_id) ON DELETE
    //            CASCADE")
    //                .arg(Tools<T>::getEntityOrderingTableName()));
    //    }

    createTableSql.append(");");

    return createTableSql;
}

template <class T> QStringList EntityTableSqlGenerator::generateRelationshipTablesSql()
{
    QStringList tableList;
    // table with id, previous, next, foreign key to T and foreign key to the entity in the relationship

    int propertyCount = T::staticMetaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {

        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        if (!property.isReadable())
        {
            continue;
        }
        QString typeName = property.typeName();

        if (!ForeignEntityTools<T>::isForeign(m_entityClassNames, typeName))
        {
            continue;
        }

        QString relationShipTableName = ForeignEntityTools<T>::getRelationshipTableName(propertyName);
        QString otherEntityClassName = ForeignEntityTools<T>::getOtherEntityClassName(typeName);
        QString otherEntityTableName = ForeignEntityTools<T>::getOtherEntityTableName(typeName);
        QString createTableSql;

        QString relationshipEntityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString relationshipOtherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(otherEntityClassName);

        if (ForeignEntityTools<T>::isList(m_entityClassNames, typeName))

        {
            createTableSql.append(QString("CREATE TABLE %1 (").arg(relationShipTableName));
            createTableSql.append("id INTEGER PRIMARY KEY AUTOINCREMENT,");
            createTableSql.append("previous INTEGER,");
            createTableSql.append("next INTEGER,");
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipEntityIdColumnName));
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipOtherEntityIdColumnName));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipEntityIdColumnName, Tools<T>::getEntityTableName()));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipOtherEntityIdColumnName, otherEntityTableName));
            createTableSql.append(QString("UNIQUE (%1, %2) ON CONFLICT ROLLBACK);")
                                      .arg(relationshipEntityIdColumnName, relationshipOtherEntityIdColumnName));

            tableList.append(createTableSql);
        }
        if (ForeignEntityTools<T>::isSet(m_entityClassNames, typeName))
        {

            createTableSql.append(QString("CREATE TABLE %1 (").arg(relationShipTableName));
            createTableSql.append("id INTEGER PRIMARY KEY AUTOINCREMENT,");
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipEntityIdColumnName));
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipOtherEntityIdColumnName));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipEntityIdColumnName, Tools<T>::getEntityTableName()));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipOtherEntityIdColumnName, otherEntityTableName));
            createTableSql.append(QString("UNIQUE (%1, %2) ON CONFLICT ROLLBACK);")
                                      .arg(relationshipEntityIdColumnName, relationshipOtherEntityIdColumnName));

            tableList.append(createTableSql);
        }
        else if (ForeignEntityTools<T>::isUnique(m_entityClassNames, typeName))
        {

            createTableSql.append(QString("CREATE TABLE %1 (").arg(relationShipTableName));
            createTableSql.append("id INTEGER PRIMARY KEY AUTOINCREMENT,");
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipEntityIdColumnName));
            createTableSql.append(QString("%1 INTEGER,").arg(relationshipOtherEntityIdColumnName));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipEntityIdColumnName, Tools<T>::getEntityTableName()));
            createTableSql.append(QString("FOREIGN KEY (%1) REFERENCES %2 (id) ON DELETE CASCADE,")
                                      .arg(relationshipOtherEntityIdColumnName, otherEntityTableName));
            createTableSql.append(QString("UNIQUE (%1) ON CONFLICT ROLLBACK);").arg(relationshipEntityIdColumnName));

            tableList.append(createTableSql);
        }
        else
            continue;
    }

    return tableList;
}

template <class T> QStringList EntityTableSqlGenerator::generateIndexes()
{
    QStringList indexList;

    //    if (std::is_base_of<Domain::OrderedEntity, T>::value)
    //    {
    //        indexList.append(QString("CREATE INDEX %1_id_index ON %2 (ordering_id);")
    //                             .arg(Tools<T>::getEntityOrderingTableName(), Tools<T>::getEntityTableName()));
    //    }

    return indexList;
}

} // namespace Database

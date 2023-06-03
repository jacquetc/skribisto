#include "entity.h"
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
    EntityTableSqlGenerator();

    template <class T> static QStringList generateEntitySql();

  private:
    template <class T> static QString generateMainTableSql();
    template <class T> static QString generateOrderingTable();
    template <class T> static QStringList generateIndexes();
    static const char *qtMetaTypeToSqlType(int qtMetaType);
};

template <class T> QStringList EntityTableSqlGenerator::generateEntitySql()
{

    static_assert(std::is_base_of<Domain::Entity, T>::value, "T must inherit from Domain::Entity");

    QStringList finalSqlList;
    finalSqlList.append(EntityTableSqlGenerator::generateMainTableSql<T>());

    finalSqlList.append(EntityTableSqlGenerator::generateIndexes<T>());

    return finalSqlList;
}

template <class T> QString EntityTableSqlGenerator::generateMainTableSql()
{

    static_assert(std::is_base_of<Domain::Entity, T>::value, "T must inherit from Domain::Entity");

    QString tableName = Tools<T>::getEntityTableName();

    QString createTableSql = QString("CREATE TABLE %1 (").arg(tableName);

    int propertyCount = T::staticMetaObject.propertyCount();
    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = T::staticMetaObject.property(i);
        const char *propertyName = property.name();

        // Ignore the "objectName" property
        if (strcmp(propertyName, "objectName") == 0)
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

template <class T> QString EntityTableSqlGenerator::generateOrderingTable()
{

    // Create the ordering table
    QString createOrderingTableSql = QString("CREATE TABLE %1 (");
    createOrderingTableSql.append("ordering_id INTEGER PRIMARY KEY AUTOINCREMENT, ");
    createOrderingTableSql.append("previous INTEGER, "); // This column can be NULL
    createOrderingTableSql.append("next INTEGER);");     // This column can also be NULL
    createOrderingTableSql = createOrderingTableSql.arg(Tools<T>::getEntityOrderingTableName());

    return createOrderingTableSql;
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

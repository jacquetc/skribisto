#pragma once
#include "QtSql/qsqlerror.h"
#include "database/interface_database_context.h"
#include "result.h"
#include "tools.h"
#include <QHash>
#include <QMetaProperty>
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QString>

using namespace Contracts::Database;

namespace Database
{

struct PropertyWithForeignKey
{
    Q_GADGET
  public:
    enum class RelationshipType
    {
        Unique,
        Set,
        List
    };
    Q_ENUM(RelationshipType)

    QString foreignTableName;
    QString relationshipTableName;
    RelationshipType relationshipType;
};

template <class T> class ForeignEntity
{
  public:
    ForeignEntity(InterfaceDatabaseContext *context);

    static QString getRelationshipTableName(const QString &otherEntityClassName);
    Result<void> addEntityRelationship(const QUuid &entityUuid, const QUuid &otherEntityUuid,
                                       const QString &otherEntityName, int position = -1);
    Result<void> removeEntityRelationship(const QUuid &entityUuid, const QUuid &otherEntityUuid,
                                          const QString &otherEntityName);
    Result<QList<QUuid>> getRelatedEntityUuids(const QUuid &entityUuid, const QString &otherEntityName);

  private:
    QHash<QString, PropertyWithForeignKey> getEntityPropertiesWithForeignKey();

  private:
    InterfaceDatabaseContext *
        m_databaseContext; /**< A QScopedPointer that holds the InterfaceDatabaseContext associated with this SkribFile.
                            */
    const QHash<QString, PropertyWithForeignKey> m_foreignKeyProperties = this->getEntityPropertiesWithForeignKey();
};

//--------------------------------------------

template <class T> ForeignEntity<T>::ForeignEntity(InterfaceDatabaseContext *context) : m_databaseContext(context)
{
}

//--------------------------------------------

template <class T> QString ForeignEntity<T>::getRelationshipTableName(const QString &otherEntityClassName)
{
    QString entityClassName = Tools<T>::getEntityTableName();
    QString relationshipTableName = entityClassName + otherEntityClassName + "Relationship";
    return relationshipTableName;
}

template <class T>
Result<void> ForeignEntity<T>::addEntityRelationship(const QUuid &entityUuid, const QUuid &otherEntityUuid,
                                                     const QString &otherEntityName, int position)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(otherEntityName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {
        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {

            QString propertyName = foreignKeyPropertyIter.key();
            QString queryStr = QString("UPDATE %1 SET %2 = :otherEntityUuid WHERE uuid = :entityUuid")
                                   .arg(Tools<T>::getEntityClassName(), propertyName);
            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
            query.bindValue(":otherEntityUuid", otherEntityUuid.toString());

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {
            QString relationshipTableName = getRelationshipTableName(otherEntityName);
            QString entityUuidColumnName =
                Tools<T>::getEntityClassName().left(1).toLower() + Tools<T>::getEntityClassName().mid(1) + "Uuid";
            QString otherEntityUuidColumnName = otherEntityName.left(1).toLower() + otherEntityName.mid(1) + "Uuid";

            QString queryStr = QString("INSERT INTO %1 (%2, %3) VALUES (:entityUuid, :otherEntityUuid)")
                                   .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
            query.bindValue(":otherEntityUuid", otherEntityUuid.toString());

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            QString relationshipTableName = getRelationshipTableName(otherEntityName);
            QString entityUuidColumnName =
                Tools<T>::getEntityClassName().left(1).toLower() + Tools<T>::getEntityClassName().mid(1) + "Uuid";
            QString otherEntityUuidColumnName = otherEntityName.left(1).toLower() + otherEntityName.mid(1) + "Uuid";

            // This query is designed to insert an element in the relationship table for a
            // PropertyWithForeignKey::RelationshipType::List. The query is composed of multiple parts using common
            // table expressions (CTEs).
            //
            // The recursive CTE `find_insert_point` traverses the linked list of elements to find the insert point
            // (between prev and next). The `prev_element` and `next_element` CTEs find the previous and next elements
            // based on the row number provided as :row. The `update_prev` CTE updates the 'next' value of the previous
            // element to point to the new element being inserted. The `update_next` CTE updates the 'prev' value of the
            // next element to point to the new element being inserted. Finally, the query inserts the new element into
            // the relationship table with the right 'prev' and 'next' values.

            QString queryStr = QString(R"(
                WITH RECURSIVE
                    find_insert_point (prev, next) AS (
                        SELECT prev, next FROM %1 WHERE %2 = :entityUuid
                        UNION ALL
                        SELECT t.prev, t.next FROM %1 t, find_insert_point fip WHERE t.%2 = :entityUuid AND t.prev = fip.next
                    ),
                    prev_element AS (SELECT prev FROM find_insert_point WHERE prev < :row AND next >= :row LIMIT 1),
                    next_element AS (SELECT next FROM find_insert_point WHERE prev < :row AND next >= :row LIMIT 1),
                    update_prev AS (
                        UPDATE %1 SET next = :otherEntityUuid WHERE %2 = :entityUuid AND %3 = (SELECT prev FROM prev_element)
                    ),
                    update_next AS (
                        UPDATE %1 SET prev = :otherEntityUuid WHERE %2 = :entityUuid AND %3 = (SELECT next FROM next_element)
                    )
                INSERT INTO %1 (prev, %2, %3, next)
                VALUES ((SELECT prev FROM prev_element), :entityUuid, :otherEntityUuid, (SELECT next FROM next_element))
            )")
                                   .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);

            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
            query.bindValue(":otherEntityUuid", otherEntityUuid.toString());
            query.bindValue(":row", position);
        }
    }
    else
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "add_relationship_failed",
                                  "Unknown relationship property", otherEntityName));
    }
}

//--------------------------------------------

template <class T>
Result<void> ForeignEntity<T>::removeEntityRelationship(const QUuid &entityUuid, const QUuid &otherEntityUuid,
                                                        const QString &otherEntityName)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(otherEntityName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {
        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            QString propertyName = foreignKeyPropertyIter.key();
            QString queryStr =
                QString("UPDATE %1 SET %2 = NULL WHERE uuid = :uuid").arg(Tools<T>::getEntityClassName(), propertyName);
            query.prepare(queryStr);
            query.bindValue(":uuid", entityUuid.toString());

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            QString relationshipTableName = getRelationshipTableName(otherEntityName);
            QString entityUuidColumnName =
                Tools<T>::getEntityClassName().left(1).toLower() + Tools<T>::getEntityClassName().mid(1) + "Uuid";
            QString otherEntityUuidColumnName = otherEntityName.left(1).toLower() + otherEntityName.mid(1) + "Uuid";

            // Prepare the query to find the element to remove and its neighbors
            QString findElementQueryStr =
                QString(R"(
                WITH element AS (
                    SELECT * FROM %1 WHERE %2 = :entityUuid AND %3 = :otherEntityUuid
                ),
                prev_element AS (
                    SELECT * FROM %1 WHERE %3 = (SELECT prev FROM element)
                ),
                next_element AS (
                    SELECT * FROM %1 WHERE %2 = (SELECT next FROM element)
                )
            )")
                    .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);

            // Update the 'next' value of the previous element and the 'prev' value of the next element
            QString updateNeighborsQueryStr =
                QString(R"(
                UPDATE %1
                SET next = (SELECT next FROM element) WHERE %3 = (SELECT prev FROM element);
                UPDATE %1
                SET prev = (SELECT prev FROM element) WHERE %2 = (SELECT next FROM element);
            )")
                    .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);

            // Remove the element from the relationship table
            QString removeElementQueryStr =
                QString("DELETE FROM %1 WHERE %2 = :entityUuid AND %3 = :otherEntityUuid")
                    .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);

            QSqlQuery query(database);
            query.prepare(findElementQueryStr + updateNeighborsQueryStr + removeElementQueryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
            query.bindValue(":otherEntityUuid", otherEntityUuid.toString());

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), query.lastQuery()));
            }
            return Result<void>();
        }

        else // Set case
        {
            QString relationshipTableName = getRelationshipTableName(otherEntityName);
            QString entityUuidColumnName =
                Tools<T>::getEntityClassName().left(1).toLower() + Tools<T>::getEntityClassName().mid(1) + "Uuid";
            QString otherEntityUuidColumnName = otherEntityName.left(1).toLower() + otherEntityName.mid(1) + "Uuid";

            QString queryStr = QString("DELETE FROM %1 WHERE %2 = :entityUuid AND %3 = :otherEntityUuid")
                                   .arg(relationshipTableName, entityUuidColumnName, otherEntityUuidColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
            query.bindValue(":otherEntityUuid", otherEntityUuid.toString());

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
    }
    else
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "remove_relationship_failed",
                                  "Unknown relationship property", otherEntityName));
    }
}

//--------------------------------------------

template <class T>
Result<QList<QUuid>> ForeignEntity<T>::getRelatedEntityUuids(const QUuid &entityUuid, const QString &otherEntityName)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(otherEntityName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {
        QString entityUuidColumnName =
            Tools<T>::getEntityClassName().left(1).toLower() + Tools<T>::getEntityClassName().mid(1) + "Uuid";
        QString otherEntityUuidColumnName = otherEntityName.left(1).toLower() + otherEntityName.mid(1) + "Uuid";

        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            QString propertyName = foreignKeyPropertyIter.key();
            QString queryStr =
                QString("SELECT %1 FROM %2 WHERE uuid = :entityUuid").arg(propertyName, Tools<T>::getEntityClassName());
            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
        }
        else
        {
            QString relationshipTableName = getRelationshipTableName(otherEntityName);
            QString queryStr;

            if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
            {
                queryStr = QString("WITH RECURSIVE ordered_relationships AS "
                                   "(SELECT %1, next FROM %2 WHERE %3 = :entityUuid AND prev IS NULL "
                                   "UNION ALL "
                                   "SELECT r.%1, r.next FROM %2 r "
                                   "JOIN ordered_relationships ON r.prev = ordered_relationships.%1) "
                                   "SELECT %1 FROM ordered_relationships")
                               .arg(otherEntityUuidColumnName, relationshipTableName, entityUuidColumnName);
            }
            else // Set case
            {
                queryStr = QString("SELECT %1 FROM %2 WHERE %3 = :entityUuid")
                               .arg(otherEntityUuidColumnName, relationshipTableName, entityUuidColumnName);
            }

            query.prepare(queryStr);
            query.bindValue(":entityUuid", entityUuid.toString());
        }

        if (!query.exec())
        {
            return Result<QList<QUuid>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text()));
        }

        QList<QUuid> uuids;
        while (query.next())
        {
            uuids.append(QUuid(query.value(0).toString()));
        }
        return Result<QList<QUuid>>(uuids);
    }
    else
    {
        return Result<QList<QUuid>>(Error(Q_FUNC_INFO, Error::Critical, "get_related_uuids_failed",
                                          "Unknown relationship property", otherEntityName));
    }
}

//--------------------------------------------

template <class T> QHash<QString, PropertyWithForeignKey> ForeignEntity<T>::getEntityPropertiesWithForeignKey()
{
    QHash<QString, PropertyWithForeignKey> foreignKeyProperties;
    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            QString typeName = property.typeName();
            QString otherEntityClassName;
            PropertyWithForeignKey::RelationshipType relationshipType;

            if (typeName.startsWith("QList<"))
            {
                otherEntityClassName =
                    typeName.mid(typeName.indexOf('<') + 1, typeName.indexOf('>') - typeName.indexOf('<') - 1);
                const QMetaObject *otherEntityMetaObject =
                    QMetaType::fromName(otherEntityClassName.toLatin1()).metaObject();
                if (!otherEntityMetaObject)
                {
                    continue;
                }
                relationshipType = PropertyWithForeignKey::RelationshipType::List;
            }
            else if (typeName.startsWith("QSet<"))
            {
                otherEntityClassName =
                    typeName.mid(typeName.indexOf('<') + 1, typeName.indexOf('>') - typeName.indexOf('<') - 1);
                const QMetaObject *otherEntityMetaObject =
                    QMetaType::fromName(otherEntityClassName.toLatin1()).metaObject();
                if (!otherEntityMetaObject)
                {
                    continue;
                }
                relationshipType = PropertyWithForeignKey::RelationshipType::Set;
            }
            else if (QMetaType::fromName(typeName.toLatin1()).flags() & QMetaType::PointerToQObject)
            {
                otherEntityClassName = typeName;
                relationshipType = PropertyWithForeignKey::RelationshipType::Unique;
            }
            else
            {
                continue;
            }

            if (!otherEntityClassName.isEmpty())
            {
                QString relationshipTableName = getRelationshipTableName(otherEntityClassName);

                // Check if the relationship table exists
                QSqlDatabase database = m_databaseContext->getConnection();
                if (database.tables().contains(relationshipTableName))
                {
                    PropertyWithForeignKey propWithForeignKey{otherEntityClassName, relationshipTableName,
                                                              relationshipType};
                    foreignKeyProperties.insert(property.name(), propWithForeignKey);
                }
            }
        }
    }
    return foreignKeyProperties;
}
} // namespace Database

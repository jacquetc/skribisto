#pragma once
#include "QtSql/qsqlerror.h"
#include "database/interface_database_context.h"
#include "entity.h"
#include "foreign_entity_tools.h"
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
template <class T> class ForeignEntity;

namespace Private
{

template <class T> class ForeignEntityFriend : public ForeignEntity<T>
{
  public:
    using ForeignEntity<T>::ForeignEntity; // Inherit constructors from ForeignEntity<T>

    Result<void> addEntityRelationship(int entityId, int otherEntityId, const QString &propertyName, int position = -1)
    {
        return ForeignEntity<T>::addEntityRelationship(entityId, otherEntityId, propertyName, position);
    }

    Result<void> removeEntityRelationship(int entityId, int otherEntityId, const QString &propertyName)
    {
        return ForeignEntity<T>::removeEntityRelationship(entityId, otherEntityId, propertyName);
    }
    Result<QList<int>> getRelatedEntityIds(int entityId, const QString &propertyName)
    {
        return ForeignEntity<T>::getRelatedEntityIds(entityId, propertyName);
    }
    Result<void> moveEntityRelationship(int entityId, int otherEntityId, const QString &propertyName, int newPosition)
    {
        return ForeignEntity<T>::moveEntityRelationship(entityId, otherEntityId, propertyName, newPosition);
    }
};

} // namespace Private

//--------------------------------------

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

    QString propertyName;
    QString foreignTableName;
    QString relationshipTableName;
    RelationshipType relationshipType;
};

//--------------------------------------

template <class T> class ForeignEntity
{
  public:
    ForeignEntity(InterfaceDatabaseContext *context);
    Result<QList<int>> getRelatedForeignIds(const T &entity, const QString &propertyName);

  protected:
    QStringList getPropertyNamesWithForeignKeydAndLoaderProperty() const;
    Result<void> manageAfterEntityAddition(T &entity);
    Result<void> manageAfterEntityUpdate(T &entity);
    Result<void> manageAfterTableClearing();

  private:
    Result<void> addEntityRelationship(int entityId, int otherEntityId, const QString &propertyName, int position = -1);
    Result<void> removeEntityRelationship(int entityId, int otherEntityId, const QString &propertyName);
    Result<QList<int>> getRelatedEntityIds(int entityId, const QString &propertyName);
    Result<void> moveEntityRelationship(int entityId, int otherEntityId, const QString &propertyName, int newPosition);

    QHash<QString, PropertyWithForeignKey> getEntityPropertiesWithForeignKey() const;
    Result<QPair<int, int>> getFuturePreviousAndNextRelationshipTableIds(int entityId, const QString &propertyName,
                                                                         int position);
    Result<QPair<int, int>> getPreviousAndNextRelationshipTableIds(int entityId, const QString &propertyName);

  private:
    InterfaceDatabaseContext *m_databaseContext;
    using ForeignKeyProperties = QHash<QString, PropertyWithForeignKey>;
    const ForeignKeyProperties m_foreignKeyProperties = this->getEntityPropertiesWithForeignKey();

  private:
    friend class Private::ForeignEntityFriend<T>;
};

//--------------------------------------------

template <class T> ForeignEntity<T>::ForeignEntity(InterfaceDatabaseContext *context) : m_databaseContext(context)
{
}

//--------------------------------------------

template <class T> Result<void> ForeignEntity<T>::manageAfterEntityAddition(T &entity)
{
    // we don't care if the property is loaded or not since there is no sense to add a lazy loader when creating an
    // entity

    for (const auto &foreignKeyProperty : m_foreignKeyProperties.values())
    {
        // verify if the property have a non empty value faor the QSet and the QList
        QVariant propertyValue = Tools<T>::getEntityPropertyValue(entity, foreignKeyProperty.propertyName);
        if (propertyValue.isNull())
        {
            continue;
        }

        if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            Domain::Entity otherEntity = propertyValue.value<Domain::Entity>();

            if (otherEntity.id() == 0) // 0 means no real entity
            {
                continue;
            }
            Result<void> result =
                this->addEntityRelationship(entity.id(), otherEntity.id(), foreignKeyProperty.propertyName);
            if (!result)
            {
                qWarning() << "Error while adding relationship between " << Tools<T>::getEntityClassName() << " and "
                           << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                return result;
            }
        }
        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {
            QSet<Domain::Entity> otherEntities = propertyValue.value<QSet<Domain::Entity>>();
            for (const Domain::Entity &otherEntity : otherEntities)
            {
                Result<void> result =
                    this->addEntityRelationship(entity.id(), otherEntity.id(), foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while adding relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return result;
                }
            }
        }
        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {

            QList<QVariant> variantList = propertyValue.toList();
            for (const QVariant &variantEntity : variantList)
            {
                bool canConvert = variantEntity.canConvert<Domain::Entity>();
                Domain::Entity otherEntity = variantEntity.value<Domain::Entity>();

                Result<void> result =
                    this->addEntityRelationship(entity.id(), otherEntity.id(), foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while adding relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return result;
                }
            }
        }
    }

    // reset lazy-loaded properties

    for (const auto &foreignKeyProperty : m_foreignKeyProperties.values())
    {
        if (ForeignEntityTools<T>::isEntityPropertyLoaded(entity, foreignKeyProperty.propertyName))
        {
            ForeignEntityTools<T>::resetEntityProperty(entity, foreignKeyProperty.propertyName);
        }
    }

    return Result<void>();
}

//--------------------------------------------

template <class T> Result<void> ForeignEntity<T>::manageAfterEntityUpdate(T &entity)
{
    // update only if the property is loaded ( {propertyName}Loaded() == true ) since we don't want to update with the
    // default empty value. Add if not already in the relationship table and remove if not in the property value.
    // Reaorganise the QList if the order is not the same as the property value

    for (const auto &foreignKeyProperty : m_foreignKeyProperties.values())
    {

        if (!ForeignEntityTools<T>::isEntityPropertyLoaded(entity, foreignKeyProperty.propertyName))
        {
            continue;
        }

        // verify if the property have a non empty value faor the QSet and the QList
        QVariant propertyValue = Tools<T>::getEntityPropertyValue(entity, foreignKeyProperty.propertyName);
        if (propertyValue.isNull())
        {
            continue;
        }

        if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            Domain::Entity otherEntity = propertyValue.value<Domain::Entity>();

            if (otherEntity.id() == 0) // 0 means no real entity, so remove the relationship if exists
            {
                Result<void> result =
                    this->removeEntityRelationship(entity.id(), otherEntity.id(), foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return result;
                }
                continue;
            }

            // get the current relationship:

            Result<QList<int>> result = this->getRelatedEntityIds(entity.id(), foreignKeyProperty.propertyName);
            if (!result)
            {
                qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName() << " and "
                           << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                return Result<void>(Error(result.error()));
            }

            QList<int> foreignIds = result.value();

            // if the relationship exists, do nothing
            if (foreignIds.contains(otherEntity.id()))
            {
                continue;
            }

            // if the relationship doesn't exists, remove the old one and add the new one
            if (foreignIds.size() > 0)
            {
                Result<void> result =
                    this->removeEntityRelationship(entity.id(), foreignIds.first(), foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return Result<void>(Error(result.error()));
                }
            }
            Result<void> addResult =
                this->addEntityRelationship(entity.id(), otherEntity.id(), foreignKeyProperty.propertyName);
            if (!result)
            {
                qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName() << " and "
                           << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                return addResult;
            }
        }
        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {
            QSet<Domain::Entity> otherEntitySet = propertyValue.value<QSet<Domain::Entity>>();

            // get the current relationship:
            Result<QList<int>> result = this->getRelatedEntityIds(entity.id(), foreignKeyProperty.propertyName);
            if (!result)
            {
                qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName() << " and "
                           << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                return Result<void>(Error(result.error()));
            }

            QList<int> resultList = result.value();
            QSet<int> foreignIds(resultList.begin(), resultList.end());

            // convert otherEntitySet in QSet<int>
            QSet<int> otherEntitySetIds;
            for (const Domain::Entity &otherEntity : otherEntitySet)
            {
                otherEntitySetIds.insert(otherEntity.id());
            }

            // Compare the two sets and remove the relationships that are not in the property value
            QSet<int> foreignIdsToRemove = foreignIds - otherEntitySetIds;
            for (const int &foreignId : foreignIdsToRemove)
            {
                Result<void> result =
                    this->removeEntityRelationship(entity.id(), foreignId, foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return result;
                }
            }

            // Compare the two sets and add the relationships that are not in the relationship table
            QSet<int> otherEntitiesToAdd = otherEntitySetIds - foreignIds;
            for (const int &otherEntityId : otherEntitiesToAdd)
            {
                Result<void> result =
                    this->addEntityRelationship(entity.id(), otherEntityId, foreignKeyProperty.propertyName);
                if (!result)
                {
                    qWarning() << "Error while adding relationship between " << Tools<T>::getEntityClassName()
                               << " and " << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                    return result;
                }
            }
        }

        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            QList<Domain::Entity> otherEntityList = propertyValue.value<QList<Domain::Entity>>();

            // get the current relationship:
            Result<QList<int>> result = this->getRelatedEntityIds(entity.id(), foreignKeyProperty.propertyName);
            if (!result)
            {
                qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName() << " and "
                           << foreignKeyProperty.foreignTableName << " : " << result.error().message();
                return Result<void>(Error(result.error()));
            }
            QList<int> foreignIds = result.value();

            // convert otherEntityList in QList<int>
            QList<int> otherEntityListIds;
            for (const Domain::Entity &otherEntity : otherEntityList)
            {
                otherEntityListIds.append(otherEntity.id());
            }

            // leave if lists are the same
            if (foreignIds == otherEntityListIds)
            {
                continue;
            }

            // Compare the two lists element by element and remove the relationships that are not in the property value
            for (const int &foreignId : foreignIds)
            {
                if (!otherEntityListIds.contains(foreignId))
                {
                    Result<void> result =
                        this->removeEntityRelationship(entity.id(), foreignId, foreignKeyProperty.propertyName);
                    if (!result)
                    {
                        qWarning() << "Error while removing relationship between " << Tools<T>::getEntityClassName()
                                   << " and " << foreignKeyProperty.foreignTableName << " : "
                                   << result.error().message();
                        return result;
                    }
                }
            }

            // Compare the two lists element by element and add the relationships that are not in the relationship table
            for (const int &otherEntityId : otherEntityListIds)
            {
                if (!foreignIds.contains(otherEntityId))
                {
                    Result<void> result =
                        this->addEntityRelationship(entity.id(), otherEntityId, foreignKeyProperty.propertyName);
                    if (!result)
                    {
                        qWarning() << "Error while adding relationship between " << Tools<T>::getEntityClassName()
                                   << " and " << foreignKeyProperty.foreignTableName << " : "
                                   << result.error().message();
                        return result;
                    }
                }
            }

            // reorder the relationships so that they are in the same order as in the property value
            for (int i = 0; i < otherEntityList.size(); ++i)
            {
                if (foreignIds[i] != otherEntityList[i].id())
                {
                    Result<void> result =
                        this->moveEntityRelationship(entity.id(), foreignIds[i], foreignKeyProperty.propertyName, i);
                    if (!result)
                    {
                        qWarning() << "Error while moving relationship between " << Tools<T>::getEntityClassName()
                                   << " and " << foreignKeyProperty.foreignTableName << " : "
                                   << result.error().message();
                        return result;
                    }
                }
            }
        }
    }
    return Result<void>();
}
//--------------------------------------------

template <class T>
Result<void> ForeignEntity<T>::moveEntityRelationship(int entityId, int otherEntityId, const QString &propertyName,
                                                      int newPosition)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {
        PropertyWithForeignKey foreignKeyProperty = foreignKeyPropertyIter.value();

        QString relationshipTableName = foreignKeyProperty.relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyProperty.foreignTableName);

        // This query first removes the existing entity relationship by updating the 'prev' and 'next' values of
        // the neighboring elements, and then inserts the entity relationship at the new position.
        QString queryStr = QString(R"(
            WITH
                current_position AS (
                    SELECT row_number() OVER (ORDER BY previous) - 1 AS position
                    FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId
                ),
                prev_element AS (
                    SELECT next AS id FROM %1 WHERE previous = (SELECT position FROM current_position)
                ),
                next_element AS (
                    SELECT previous AS id FROM %1 WHERE next = (SELECT position FROM current_position)
                ),
                remove AS (
                    UPDATE %1 SET previous = (SELECT id FROM next_element) WHERE previous = (SELECT position FROM current_position);
                    UPDATE %1 SET next = (SELECT id FROM prev_element) WHERE next = (SELECT position FROM current_position);
                    DELETE FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId
                ),
                find_insert_point (previous, next) AS (
                    SELECT previous, next FROM %1 WHERE %2 = :entityId
                    UNION ALL
                    SELECT t.previous, t.next FROM %1 t, find_insert_point fip WHERE t.%2 = :entityId AND t.previous = fip.next
                ),
                prev_element_insert AS (SELECT previous FROM find_insert_point WHERE previous < :newPosition AND next >= :newPosition LIMIT 1),
                next_element_insert AS (SELECT next FROM find_insert_point WHERE previous < :newPosition AND next >= :newPosition LIMIT 1),
                update_prev_insert AS (
                    UPDATE %1 SET next = :otherEntityId WHERE %2 = :entityId AND %3 = (SELECT previous FROM prev_element_insert)
                ),
                update_next_insert AS (
                    UPDATE %1 SET previous = :otherEntityId WHERE %2 = :entityId AND %3 = (SELECT next FROM next_element_insert)
                )
            INSERT INTO %1 (previous, %2, %3, next)
            VALUES ((SELECT previous FROM prev_element_insert), :entityId, :otherEntityId, (SELECT next FROM next_element_insert))
        )")
                               .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);

        query.prepare(queryStr);
        query.bindValue(":entityId", entityId);
        query.bindValue(":otherEntityId", otherEntityId);
        query.bindValue(":newPosition", newPosition);

        if (!query.exec())
        {
            qWarning() << "Error while moving relationship between " << Tools<T>::getEntityClassName() << " and "
                       << " : " << query.lastError().text();
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(),
                                      "Error while moving relationship"));
        }
    }
    else
    {
        qWarning() << "Error while moving relationship between " << Tools<T>::getEntityClassName() << " and "
                   << " : no property named " << propertyName;
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", "No property named " + propertyName,
                                  "Error while moving relationship"));
    }

    return Result<void>();
}

//--------------------------------------------

template <class T> Result<void> ForeignEntity<T>::manageAfterTableClearing()
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    for (const auto &foreignKeyProperty : m_foreignKeyProperties.values())
    {
        QString relationshipTableName = foreignKeyProperty.relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyProperty.foreignTableName);

        QString queryStr = QString("DELETE FROM %1").arg(relationshipTableName);
        query.prepare(queryStr);
        if (!query.exec())
        {
            qWarning() << "Error while clearing relationship table " << relationshipTableName << " : "
                       << query.lastError().text();
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(),
                                      "Error while clearing relationship table"));
        }
    }

    return Result<void>();
}

//--------------------------------------------

template <class T>
Result<QList<int>> ForeignEntity<T>::getRelatedForeignIds(const T &entity, const QString &propertyName)
{

    return getRelatedEntityIds(entity.id(), propertyName);
}

//--------------------------------------------

template <class T>
Result<void> ForeignEntity<T>::addEntityRelationship(int entityId, int otherEntityId, const QString &propertyName,
                                                     int position)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {

        QString relationshipTableName = foreignKeyPropertyIter->relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyPropertyIter->foreignTableName);

        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            QString queryStr = QString("INSERT INTO %1 (%2, %3) VALUES (:entityId, :otherEntityId)")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {

            QString queryStr = QString("INSERT INTO %1 (%2, %3) VALUES (:entityId, :otherEntityId)")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {

            int listSize = this->getRelatedEntityIds(entityId, propertyName).value().size();
            if (position == -1 || position > listSize)
            {
                position = listSize;
            }

            Result<QPair<int, int>> futurePreviousAndNextIdsResult =
                this->getFuturePreviousAndNextRelationshipTableIds(entityId, propertyName, position);
            if (futurePreviousAndNextIdsResult.hasError())
            {
                return Result<void>(futurePreviousAndNextIdsResult.error());
            }

            QPair<int, int> futurePreviousAndNextIds = futurePreviousAndNextIdsResult.value();

            int previousId = futurePreviousAndNextIds.first;
            int nextId = futurePreviousAndNextIds.second;

            // Now perform the updates separately
            QString updatePrevQueryStr =
                QString("UPDATE %1 SET next = :otherEntityId WHERE %2 = :entityId AND %3 = :prevId")
                    .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(updatePrevQueryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);
            query.bindValue(":prevId", previousId == 0 ? QVariant(QMetaType::fromType<QString>()) : previousId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updatePrevQueryStr));
            }

            QString updateNextQueryStr =
                QString("UPDATE %1 SET previous = :otherEntityId WHERE %2 = :entityId AND %3 = :nextId")
                    .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(updateNextQueryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);
            query.bindValue(":nextId", nextId == 0 ? QVariant(QMetaType::fromType<QString>()) : nextId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), updateNextQueryStr));
            }

            // Finally, insert the new row
            QString insertQueryStr =
                QString("INSERT INTO %1 (previous, next, %2, %3) VALUES (:prevId, :nextId, :entityId, :otherEntityId)")
                    .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(insertQueryStr);
            query.bindValue(":prevId", previousId == 0 ? QVariant(QMetaType::fromType<QString>()) : previousId);
            query.bindValue(":nextId", nextId == 0 ? QVariant(QMetaType::fromType<QString>()) : nextId);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), insertQueryStr));
            }

            return Result<void>();
        }
    }
    else
    {
        return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "add_relationship_failed",
                                  "Unknown relationship property", propertyName));
    }
    return Result<void>();
}

//--------------------------------------------

template <class T>
Result<void> ForeignEntity<T>::removeEntityRelationship(int entityId, int otherEntityId, const QString &propertyName)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {

        QString relationshipTableName = foreignKeyPropertyIter->relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyPropertyIter->foreignTableName);

        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {

            QString queryStr = QString("DELETE FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {

            QString queryStr = QString("DELETE FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            return Result<void>();
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            // remove the element from the list and update the prev and next values of the elements before and
            // after the element being removed
            QString queryStr = QString(R"(
                    WITH RECURSIVE
                        find_element (prev, next) AS (
                            SELECT prev, next FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId
                            UNION ALL
                            SELECT t.prev, t.next FROM %1 t, find_element fe WHERE t.%2 = :entityId AND t.prev = fe.next
                        ),
                        update_prev AS (
                            UPDATE %1 SET next = (SELECT next FROM find_element) WHERE %2 = :entityId AND %3 = (SELECT prev FROM find_element)
                        ),
                        update_next AS (
                            UPDATE %1 SET prev = (SELECT prev FROM find_element) WHERE %2 = :entityId AND %3 = (SELECT next FROM find_element)
                        )
                    DELETE FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId
                )")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);

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
                                  "Unknown relationship property", propertyName));
    }
}

//--------------------------------------------

template <class T> Result<QList<int>> ForeignEntity<T>::getRelatedEntityIds(int entityId, const QString &propertyName)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {

        QString relationshipTableName = foreignKeyPropertyIter->relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyPropertyIter->foreignTableName);
        QString queryStr;

        if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            QString propertyName = foreignKeyPropertyIter.key();
            queryStr = QString("SELECT %1 FROM %2 WHERE %3 = :entityId")
                           .arg(otherEntityIdColumnName, relationshipTableName, entityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
        }
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            queryStr = QString("WITH RECURSIVE ordered_relationships(id, %3, row_number) AS ("
                               "  SELECT id, %3, 1"
                               "  FROM %1"
                               "  WHERE previous IS NULL AND %2 = :entityId"
                               "  UNION ALL"
                               "  SELECT deo.id, deo.%3, o_r.row_number + 1"
                               "  FROM %1 deo"
                               "  JOIN ordered_relationships o_r ON deo.previous = o_r.id"
                               ")"
                               "SELECT %3 FROM ordered_relationships ORDER BY row_number")
                           .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
        }

        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::Set)
        {

            queryStr = QString("SELECT %1 FROM %2 WHERE %3 = :entityId")
                           .arg(otherEntityIdColumnName, relationshipTableName, entityIdColumnName);
        }

        if (!query.prepare(queryStr))
        {
            return Result<QList<int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error_prepare", query.lastError().text()));
        }
        query.bindValue(":entityId", entityId);

        if (!query.exec())
        {
            return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text()));
        }

        QList<int> ids;
        while (query.next())
        {
            ids.append(query.value(0).toInt());
        }
        return Result<QList<int>>(ids);
    }
    else
    {
        return Result<QList<int>>(Error(Q_FUNC_INFO, Error::Critical, "get_related_ids_failed",
                                        "Unknown relationship property", propertyName));
    }

    // unreachable
    return Result<QList<int>>();
}

template <class T> QStringList ForeignEntity<T>::getPropertyNamesWithForeignKeydAndLoaderProperty() const
{
    QStringList foreignKeyProperties = m_foreignKeyProperties.keys();

    // add "Loader" to each property name and append it to the list
    QStringList propertyNamesWithForeignKeydAndLoaderProperty;
    propertyNamesWithForeignKeydAndLoaderProperty.append(foreignKeyProperties);
    for (const QString &propertyName : foreignKeyProperties)
    {
        propertyNamesWithForeignKeydAndLoaderProperty.append(propertyName + "Loaded");
    }

    return propertyNamesWithForeignKeydAndLoaderProperty;
}

//--------------------------------------------

template <class T> QHash<QString, PropertyWithForeignKey> ForeignEntity<T>::getEntityPropertiesWithForeignKey() const
{
    QHash<QString, PropertyWithForeignKey> foreignKeyProperties;
    const QMetaObject &metaObject = T::staticMetaObject;
    int propertyCount = metaObject.propertyCount();

    QList<QMetaProperty> foreignProperties;

    // filters foreign properties
    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty property = metaObject.property(i);
        if (property.isReadable())
        {
            QString typeName = property.typeName();
            if (ForeignEntityTools<T>::isForeign(m_databaseContext->entityClassNames(), typeName))
            {
                foreignProperties.append(property);
            }
        }
    }

    // fill foreignKeyProperties

    for (const QMetaProperty &property : foreignProperties)
    {
        if (property.isReadable())
        {
            QString typeName = property.typeName();
            QString otherEntityTableName = ForeignEntityTools<T>::getOtherEntityTableName(typeName);
            PropertyWithForeignKey::RelationshipType relationshipType;

            if (ForeignEntityTools<T>::isList(m_databaseContext->entityClassNames(), typeName))
            {
                relationshipType = PropertyWithForeignKey::RelationshipType::List;
            }
            else if (ForeignEntityTools<T>::isSet(m_databaseContext->entityClassNames(), typeName))
            {
                relationshipType = PropertyWithForeignKey::RelationshipType::Set;
            }
            else if (ForeignEntityTools<T>::isUnique(m_databaseContext->entityClassNames(), typeName))
            {

                relationshipType = PropertyWithForeignKey::RelationshipType::Unique;
            }
            else
            {
                Q_UNREACHABLE();
            }

            QString relationshipTableName = ForeignEntityTools<T>::getRelationshipTableName(property.name());

            // Check if the relationship table exists
            QSqlDatabase database = m_databaseContext->getConnection();
            if (database.tables().contains(relationshipTableName))
            {
                PropertyWithForeignKey propWithForeignKey{property.name(), otherEntityTableName, relationshipTableName,
                                                          relationshipType};
                foreignKeyProperties.insert(property.name(), propWithForeignKey);
            }
            else
            {
                qFatal("relationship table doesn't exist");
            }
        }
    }
    return foreignKeyProperties;
}

//--------------------------------------------------------------

template <class T>
Result<QPair<int, int>> ForeignEntity<T>::getFuturePreviousAndNextRelationshipTableIds(int entityId,
                                                                                       const QString &propertyName,
                                                                                       int position)
{

    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    int previousRowId = 0;
    int nextRowId = 0;

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {

        QString relationshipTableName = foreignKeyPropertyIter->relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyPropertyIter->foreignTableName);

        // dertermine previous row id and next row id:

        // empty ?
        QString checkQueryStr =
            QString("SELECT COUNT(*) FROM " + relationshipTableName + " WHERE %1 = :entityId").arg(entityIdColumnName);

        if (!query.prepare(checkQueryStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), checkQueryStr));
        }

        query.bindValue(":entityId", entityId);

        if (!query.exec())
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), checkQueryStr));
        }
        if (!query.next())
        {
            return Result<QPair<int, int>>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text()));
        }

        int rowCount = query.value(0).toInt();

        if (position == -1 || position > rowCount)
        {
            position = rowCount;
        }

        if (rowCount == 0)
        {
            previousRowId = 0;
            nextRowId = 0;
        }
        else if (rowCount > 0 && rowCount <= position)
        {

            QString selectNextStr = QString("SELECT id FROM %1 WHERE next IS NULL AND %2 = :entityId")
                                        .arg(relationshipTableName, entityIdColumnName);
            query.prepare(selectNextStr);
            query.bindValue(":entityId", entityId);

            if (!query.exec())
            {
                return Result<QPair<int, int>>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectNextStr));
            }

            if (query.next())
            {
                previousRowId = query.value(0).toInt();
            }
            nextRowId = 0;
        }
        else if (position == 0)
        {
            previousRowId = 0;

            QString selectPreviousStr = QString("SELECT id FROM %1 WHERE previous IS NULL AND %2 = :entityId")
                                            .arg(relationshipTableName, entityIdColumnName);
            query.prepare(selectPreviousStr);
            query.bindValue(":entityId", entityId);

            if (!query.exec())
            {
                return Result<QPair<int, int>>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousStr));
            }

            if (query.next())
            {
                nextRowId = query.value(0).toInt();
            }
        }
        else
        {
            QString selectPreviousStr = QString("WITH RECURSIVE ordered_relationships(id, next, row_number) AS ("
                                                "  SELECT id, next, 1"
                                                "  FROM %1"
                                                "  WHERE previous IS NULL AND %2 = :entityId"
                                                "  UNION ALL"
                                                "  SELECT deo.id, deo.next, o_r.row_number + 1"
                                                "  FROM %1 deo"
                                                "  JOIN ordered_relationships o_r ON deo.previous = o_r.id"
                                                ")"
                                                "SELECT id FROM ordered_relationships WHERE row_number = %3")
                                            .arg(relationshipTableName, entityIdColumnName)
                                            .arg(position);

            query.prepare(selectPreviousStr);
            query.bindValue(":entityId", entityId);

            if (!query.exec())
            {
                return Result<QPair<int, int>>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousStr));
            }
            if (query.next())
            {
                previousRowId = query.value(0).toInt();
            }

            QString selectNextStr = QString("WITH RECURSIVE ordered_relationships(id, next, row_number) AS ("
                                            "  SELECT id, next, 1"
                                            "  FROM %1"
                                            "  WHERE previous IS NULL AND %2 = :entityId"
                                            "  UNION ALL"
                                            "  SELECT deo.id, deo.next, o_r.row_number + 1"
                                            "  FROM %1 deo"
                                            "  JOIN ordered_relationships o_r ON deo.previous = o_r.id"
                                            ")"
                                            "SELECT id FROM ordered_relationships WHERE row_number = %3")
                                        .arg(relationshipTableName, entityIdColumnName)
                                        .arg(position + 1);

            query.prepare(selectNextStr);
            query.bindValue(":entityId", entityId);

            if (!query.exec())
            {
                return Result<QPair<int, int>>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectNextStr));
            }
            if (query.next())
            {
                nextRowId = query.value(0).toInt();
            }
        }
    }
    else
    {
        return Result<QPair<int, int>>(Error(Q_FUNC_INFO, Error::Critical,
                                             "get_future_previous_and_next_relationship_ids_failed",
                                             "Unknown relationship property", propertyName));
    }

    return Result<QPair<int, int>>(QPair<int, int>(previousRowId, nextRowId));
}

template <class T>
Result<QPair<int, int>> ForeignEntity<T>::getPreviousAndNextRelationshipTableIds(int entityId,
                                                                                 const QString &propertyName)
{
    QSqlDatabase database = m_databaseContext->getConnection();
    QSqlQuery query(database);

    int previousRowId = 0;
    int nextRowId = 0;

    auto foreignKeyPropertyIter = m_foreignKeyProperties.find(propertyName);
    if (foreignKeyPropertyIter != m_foreignKeyProperties.end())
    {

        QString relationshipTableName = foreignKeyPropertyIter->relationshipTableName;
        QString entityIdColumnName = ForeignEntityTools<T>::getRelationshipEntityIdColumnName();
        QString otherEntityIdColumnName =
            ForeignEntityTools<T>::getRelationshipOtherEntityIdColumnName(foreignKeyPropertyIter->foreignTableName);

        // dertermine previous row id and next row id:

        // Get the ordering_id for the given entityId
        QString getEntityOrderingIdStr =
            QString("SELECT id FROM %1 WHERE %2 = %3").arg(relationshipTableName, entityIdColumnName).arg(entityId);
        if (!query.exec(getEntityOrderingIdStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), getEntityOrderingIdStr));
        }

        int relationshipId = 0;
        if (query.next())
        {
            relationshipId = query.value(0).toInt();
        }
        else
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Entity not found for given entityId."));
        }

        // Get previous and next ordering ids using the entity's ordering_id
        QString selectPreviousAndNextStr =
            QString("SELECT previous, next FROM %1 WHERE id = %2").arg(relationshipTableName).arg(relationshipId);
        if (!query.exec(selectPreviousAndNextStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousAndNextStr));
        }

        if (query.next())
        {
            QVariant previousValue = query.value(0);
            QVariant nextValue = query.value(1);

            previousRowId = previousValue.isNull() ? 0 : previousValue.toInt();
            nextRowId = nextValue.isNull() ? 0 : nextValue.toInt();
        }
        else
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Ordering not found for given relationshipId."));
        }
    }
    else
    {
        return Result<QPair<int, int>>(Error(Q_FUNC_INFO, Error::Critical,
                                             "get_previous_and_next_relationship_ids_failed",
                                             "Unknown relationship property", propertyName));
    }

    return Result<QPair<int, int>>(QPair<int, int>(previousRowId, nextRowId));
}

} // namespace Database

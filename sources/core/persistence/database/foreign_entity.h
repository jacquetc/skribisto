#pragma once
#include "QtSql/qsqlerror.h"
#include "database/interface_database_context.h"
#include "entity_base.h"
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
    Result<QPair<int, int>> getPreviousAndNextRelationshipTableIds(int entityId, int otherEntityId,
                                                                   const QString &propertyName);

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
        // verify if the property have a non empty value for the the QList
        QVariant propertyValue = Tools<T>::getEntityPropertyValue(entity, foreignKeyProperty.propertyName);
        if (propertyValue.isNull())
        {
            continue;
        }

        if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            Domain::EntityBase otherEntity = propertyValue.value<Domain::EntityBase>();

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
        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {

            QList<QVariant> variantList = propertyValue.toList();
            for (const QVariant &variantEntity : variantList)
            {
                bool canConvert = variantEntity.canConvert<Domain::EntityBase>();
                Domain::EntityBase otherEntity = variantEntity.value<Domain::EntityBase>();

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

        // verify if the property have a non empty value for the the QList
        QVariant propertyValue = Tools<T>::getEntityPropertyValue(entity, foreignKeyProperty.propertyName);
        if (propertyValue.isNull())
        {
            continue;
        }

        if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::Unique)
        {
            Domain::EntityBase otherEntity = propertyValue.value<Domain::EntityBase>();

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

        else if (foreignKeyProperty.relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            QList<Domain::EntityBase> otherEntityList = propertyValue.value<QList<Domain::EntityBase>>();

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
            for (const Domain::EntityBase &otherEntity : otherEntityList)
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
        // Get current and future position IDs
        Result<QPair<int, int>> currentPostionIdsResult =
            getPreviousAndNextRelationshipTableIds(entityId, otherEntityId, propertyName);
        if (currentPostionIdsResult.hasError())
        {
            qWarning() << "Error while moving relationship: Could not get previous and next relationship table ids for "
                          "property named "
                       << propertyName;
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Could not get current position ids",
                                      "Error while moving relationship"));
        }

        Result<QPair<int, int>> futurePositionIdsResult =
            getFuturePreviousAndNextRelationshipTableIds(entityId, propertyName, newPosition);
        if (futurePositionIdsResult.hasError())
        {
            qWarning() << "Error while moving relationship: Could not get future previous and next relationship table "
                          "ids for property named "
                       << propertyName;
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error", "Could not get future position ids",
                                      "Error while moving relationship"));
        }

        // Extract ids
        int currentPrevId = currentPostionIdsResult.value().first;
        int currentNextId = currentPostionIdsResult.value().second;
        int futurePrevId = futurePositionIdsResult.value().first;
        int futureNextId = futurePositionIdsResult.value().second;

        // Prepare the SQL queries
        QString queryText;

        int rowId = 0;

        // 0 . Get the relationship table id of the moved row:
        queryText = QString("SELECT id FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId")
                        .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
        query.prepare(queryText);
        query.bindValue(":entityId", entityId);
        query.bindValue(":otherEntityId", otherEntityId);
        if (!query.exec())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                      "Database error while getting the id of the current row",
                                      "Error while moving relationship"));
        }
        if (query.next())
        {
            rowId = query.value(0).toInt();
        }
        else
        {

            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                      "Database error while getting the id of the current row",
                                      "Error while moving relationship"));
        }

        // 1. Modify the 'next' of the future previous row, setting the target id, only if futurePrevId is not NULL
        // (meaning the future empplacement of  the moved id will be at the beginning of the list)
        if (futurePrevId > 0)
        {
            queryText = QString("UPDATE %1 SET next = :rowId WHERE id = :futurePrevId").arg(relationshipTableName);
            query.prepare(queryText);
            query.bindValue(":rowId", rowId);
            query.bindValue(":futurePrevId", futurePrevId);
            if (!query.exec())
            {
                return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                          "Database error while updating future previous relationship",
                                          "Error while moving relationship"));
            }
        }

        // 2. Modify the 'previous' of the future next row, setting the target id, only if futureNextId is not NULL
        // (meaning the future empplacement of  the moved id will be at the end of the list)
        if (futureNextId > 0)
        {
            queryText = QString("UPDATE %1 SET previous = :rowId WHERE id = :futureNextId").arg(relationshipTableName);
            query.prepare(queryText);
            query.bindValue(":rowId", rowId);
            query.bindValue(":futureNextId", futureNextId);
            if (!query.exec())
            {
                return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                          "Database error while updating future next relationship",
                                          "Error while moving relationship"));
            }
        }

        // 3. Modify the 'next' of the current previous row, setting the current next row, only if currentPrevId is not
        // NULL (meaning the moved id was at the begining of the list)
        if (currentPrevId > 0)
        {
            queryText =
                QString("UPDATE %1 SET next = :currentNextId WHERE id = :currentPrevId").arg(relationshipTableName);
            query.prepare(queryText);
            query.bindValue(":currentNextId",
                            currentNextId == 0 ? QVariant(QMetaType::fromType<int>()) : currentNextId);
            query.bindValue(":currentPrevId", currentPrevId);
            if (!query.exec())
            {
                return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                          "Database error while updating current previous relationship",
                                          "Error while moving relationship"));
            }
        }

        // 4. Modify the 'previous' of the current next row, setting the current previous row, only if currentNextId is
        // not NULL (meaning the moved id was at the end of the list)
        queryText =
            QString("UPDATE %1 SET previous = :currentPrevId WHERE id = :currentNextId").arg(relationshipTableName);
        query.prepare(queryText);
        query.bindValue(":currentPrevId", currentPrevId == 0 ? QVariant(QMetaType::fromType<int>()) : currentPrevId);
        query.bindValue(":currentNextId", currentNextId);
        if (!query.exec())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                      "Database error while updating current next relationship",
                                      "Error while moving relationship"));
        }

        // 5. Modify the 'previous' and 'next' of the current row
        queryText = QString("UPDATE %1 SET previous = :futurePrevId, next = :futureNextId WHERE id = :rowId")
                        .arg(relationshipTableName);
        query.prepare(queryText);
        query.bindValue(":futurePrevId", futurePrevId == 0 ? QVariant(QMetaType::fromType<int>()) : futurePrevId);
        query.bindValue(":futureNextId", futureNextId == 0 ? QVariant(QMetaType::fromType<int>()) : futureNextId);
        query.bindValue(":rowId", rowId);
        if (!query.exec())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "sql_error",
                                      "Database error while updating the previous and next of the current row",
                                      "Error while moving relationship"));
        }

        return Result<void>();
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
        else if (foreignKeyPropertyIter->relationshipType == PropertyWithForeignKey::RelationshipType::List)
        {
            // Step 1: Fetch the relevant 'previous' and 'next' values from the table
            QString queryStr = QString(R"(
        SELECT previous, next FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId
    )")
                                   .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":otherEntityId", otherEntityId);
            if (!query.exec() || !query.next())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }
            auto previous = query.value(0);
            auto next = query.value(1);

            // Step 2: Update the 'next' value for the previous element
            queryStr = QString(R"(
        UPDATE %1 SET next = :next WHERE %2 = :entityId AND %3 = :previous
    )")
                           .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":next", next);
            query.bindValue(":previous", previous);
            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }

            // Step 3: Update the 'previous' value for the next element
            queryStr = QString(R"(
        UPDATE %1 SET previous = :previous WHERE %2 = :entityId AND %3 = :next
    )")
                           .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
            query.prepare(queryStr);
            query.bindValue(":entityId", entityId);
            query.bindValue(":previous", previous);
            query.bindValue(":next", next);
            if (!query.exec())
            {
                return Result<void>(
                    Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), queryStr));
            }

            // Step 4: Delete the original element
            queryStr = QString(R"(
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
                               "  JOIN ordered_relationships o_r ON deo.previous = o_r.id "
                               "  AND %2 = :entityId"
                               ")"
                               "SELECT %3 FROM ordered_relationships ORDER BY row_number")
                           .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);
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

        // rectify position
        if (position == -1 || position > rowCount)
        {
            position = rowCount;
        }

        // edge case : empty
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
Result<QPair<int, int>> ForeignEntity<T>::getPreviousAndNextRelationshipTableIds(int entityId, int otherEntityId,
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

        // Get previous and next ordering ids using the entity's ordering_id
        QString selectPreviousAndNextStr =
            QString("SELECT previous, next FROM %1 WHERE %2 = :entityId AND %3 = :otherEntityId")
                .arg(relationshipTableName, entityIdColumnName, otherEntityIdColumnName);

        if (!query.prepare(selectPreviousAndNextStr))
        {
            return Result<QPair<int, int>>(
                Error(Q_FUNC_INFO, Error::Critical, "sql_error", query.lastError().text(), selectPreviousAndNextStr));
        }

        query.bindValue(":entityId", entityId);
        query.bindValue(":otherEntityId", otherEntityId);

        if (!query.exec())
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

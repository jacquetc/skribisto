#pragma once

#include "database/foreign_entity.h"

//---------------------------

template <class T> class ForeignEntityWrapper : public Database::Private::ForeignEntityFriend<T>
{
  public:
    ForeignEntityWrapper(InterfaceDatabaseContext *context) : Database::Private::ForeignEntityFriend<T>(context)
    {
    }

    // Wrap protected methods
    QStringList testGetPropertyNamesWithForeignKeydAndLoaderProperty() const
    {
        return this->getPropertyNamesWithForeignKeydAndLoaderProperty();
    }

    Result<void> testManageAfterEntityAddition(T &entity)
    {
        return this->manageAfterEntityAddition(entity);
    }

    Result<void> testManageAfterEntityUpdate(T &entity)
    {
        return this->manageAfterEntityUpdate(entity);
    }

    Result<void> testManageAfterTableClearing()
    {
        return this->manageAfterTableClearing();
    }

    // Wrap private methods
    Result<void> testAddEntityRelationship(int entityId, int otherEntityId, const QString &propertyName,
                                           int position = -1)
    {
        return this->addEntityRelationship(entityId, otherEntityId, propertyName, position);
    }

    Result<void> testRemoveEntityRelationship(int entityId, int otherEntityId, const QString &propertyName)
    {
        return this->removeEntityRelationship(entityId, otherEntityId, propertyName);
    }

    Result<QList<int>> testGetRelatedEntityIds(int entityId, const QString &propertyName)
    {
        return this->getRelatedEntityIds(entityId, propertyName);
    }

    Result<void> testMoveEntityRelationship(int entityId, int otherEntityId, const QString &propertyName,
                                            int newPosition)
    {
        return this->moveEntityRelationship(entityId, otherEntityId, propertyName, newPosition);
    }

    QHash<QString, Database::PropertyWithForeignKey> testGetEntityPropertiesWithForeignKey() const
    {
        return this->getEntityPropertiesWithForeignKey();
    }
};

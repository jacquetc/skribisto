// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "chapter_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

ChapterRepository::ChapterRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Chapter> *chapterDatabase,
                                     InterfaceSceneRepository *sceneRepository)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::Chapter>(chapterDatabase),
      m_sceneRepository(sceneRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *ChapterRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Skribisto::Domain::Chapter> ChapterRepository::update(Domain::Chapter &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().scenesSet)
    {

        Result<QList<Domain::Scene>> scenesResult = m_sceneRepository->updateEntitiesInRelationOf(
            Domain::Chapter::schema, entity.id(), "scenes", entity.scenes());

#ifdef QT_DEBUG
        if (scenesResult.isError())
        {
            qCritical() << scenesResult.error().code() << scenesResult.error().message() << scenesResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Chapter, scenesResult)
    }

    return Qleany::Repository::GenericRepository<Domain::Chapter>::update(std::move(entity));
}

Result<Skribisto::Domain::Chapter> ChapterRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Domain::Chapter>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Chapter entity = getResult.value();

    Result<QList<Domain::Scene>> scenesResult =
        m_sceneRepository->getEntitiesInRelationOf(Domain::Chapter::schema, entity.id(), "scenes");

#ifdef QT_DEBUG
    if (scenesResult.isError())
    {
        qCritical() << scenesResult.error().code() << scenesResult.error().message() << scenesResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Chapter, scenesResult)

    entity.setScenes(scenesResult.value());

    return Result<Domain::Chapter>(entity);
}

Skribisto::Domain::Chapter::ScenesLoader ChapterRepository::fetchScenesLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "scenes" property in the entity Chapter using staticMetaObject
    int propertyIndex = Skribisto::Domain::Chapter::staticMetaObject.indexOfProperty("scenes");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Chapter doesn't have a property named scenes";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult =
            m_sceneRepository->getEntitiesInRelationOf(Skribisto::Domain::Chapter::schema, entityId, "scenes");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Skribisto::Domain::Scene>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> ChapterRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the scenes in cascade

    Qleany::Domain::RelationshipInfo sceneScenesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Chapter::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Scene && relationship.fieldName == "scenes")
        {
            sceneScenesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (sceneScenesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Scene> foreignScenes = this->fetchScenesLoader().operator()(entityId);

            if (foreignScenes.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &scene : foreignScenes)
            {
                foreignIds.append(scene.id());
            }

            auto removalResult = m_sceneRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::Chapter, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> ChapterRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the scenes in cascade

    Qleany::Domain::RelationshipInfo sceneScenesRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Chapter::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::Scene && relationship.fieldName == "scenes")
        {
            sceneScenesRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (sceneScenesRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::Scene> foreignScenes = this->fetchScenesLoader().operator()(entityId);

            if (foreignScenes.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &scene : foreignScenes)
            {
                foreignIds.append(scene.id());
            }

            auto changeResult = m_sceneRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::Chapter, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
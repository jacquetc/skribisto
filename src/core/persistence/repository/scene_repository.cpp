// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "scene_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Persistence::Repository;
using namespace Skribisto::Contracts::Repository;

SceneRepository::SceneRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Scene> *sceneDatabase,
                                 InterfaceSceneParagraphRepository *sceneParagraphRepository)
    : Qleany::Repository::GenericRepository<Skribisto::Domain::Scene>(sceneDatabase),
      m_sceneParagraphRepository(sceneParagraphRepository)
{
    m_signalHolder.reset(new SignalHolder(nullptr));
}

SignalHolder *SceneRepository::signalHolder()
{
    QReadLocker locker(&m_lock);
    return m_signalHolder.data();
}

Result<Skribisto::Domain::Scene> SceneRepository::update(Domain::Scene &&entity)
{
    QWriteLocker locker(&m_lock);

    if (entity.metaData().paragraphsSet)
    {

        Result<QList<Domain::SceneParagraph>> paragraphsResult = m_sceneParagraphRepository->updateEntitiesInRelationOf(
            Domain::Scene::schema, entity.id(), "paragraphs", entity.paragraphs());

#ifdef QT_DEBUG
        if (paragraphsResult.isError())
        {
            qCritical() << paragraphsResult.error().code() << paragraphsResult.error().message()
                        << paragraphsResult.error().data();
            qFatal("Error found. The application will now exit");
        }
#endif
        QLN_RETURN_IF_ERROR(Domain::Scene, paragraphsResult)
    }

    return Qleany::Repository::GenericRepository<Domain::Scene>::update(std::move(entity));
}

Result<Skribisto::Domain::Scene> SceneRepository::getWithDetails(int entityId)
{
    QWriteLocker locker(&m_lock);
    auto getResult = Qleany::Repository::GenericRepository<Domain::Scene>::get(entityId);

    if (getResult.isError())
    {
        return getResult;
    }

    Domain::Scene entity = getResult.value();

    Result<QList<Domain::SceneParagraph>> paragraphsResult =
        m_sceneParagraphRepository->getEntitiesInRelationOf(Domain::Scene::schema, entity.id(), "paragraphs");

#ifdef QT_DEBUG
    if (paragraphsResult.isError())
    {
        qCritical() << paragraphsResult.error().code() << paragraphsResult.error().message()
                    << paragraphsResult.error().data();
        qFatal("Error found. The application will now exit");
    }
#endif
    QLN_RETURN_IF_ERROR(Domain::Scene, paragraphsResult)

    entity.setParagraphs(paragraphsResult.value());

    return Result<Domain::Scene>(entity);
}

Skribisto::Domain::Scene::ParagraphsLoader SceneRepository::fetchParagraphsLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "paragraphs" property in the entity Scene using staticMetaObject
    int propertyIndex = Skribisto::Domain::Scene::staticMetaObject.indexOfProperty("paragraphs");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Scene doesn't have a property named paragraphs";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto foreignEntitiesResult = m_sceneParagraphRepository->getEntitiesInRelationOf(
            Skribisto::Domain::Scene::schema, entityId, "paragraphs");

        if (foreignEntitiesResult.isError())
        {
            qCritical() << foreignEntitiesResult.error().code() << foreignEntitiesResult.error().message()
                        << foreignEntitiesResult.error().data();
            return QList<Skribisto::Domain::SceneParagraph>();
        }

        return foreignEntitiesResult.value();
    };
}

Result<QHash<int, QList<int>>> SceneRepository::removeInCascade(QList<int> ids)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the paragraphs in cascade

    Qleany::Domain::RelationshipInfo sceneParagraphParagraphsRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Scene::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::SceneParagraph &&
            relationship.fieldName == "paragraphs")
        {
            sceneParagraphParagraphsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (sceneParagraphParagraphsRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::SceneParagraph> foreignParagraphs =
                this->fetchParagraphsLoader().operator()(entityId);

            if (foreignParagraphs.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &sceneParagraph : foreignParagraphs)
            {
                foreignIds.append(sceneParagraph.id());
            }

            auto removalResult = m_sceneParagraphRepository->removeInCascade(foreignIds);
            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removalResult)

            returnedHashOfEntityWithRemovedIds.insert(removalResult.value());
        }
    }

    // finally remove the entites of this repository

    Result<void> associationRemovalResult = this->databaseTable()->removeAssociationsWith(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, associationRemovalResult)
    Result<QList<int>> removedIdsResult = this->databaseTable()->remove(ids);
    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, removedIdsResult)

    returnedHashOfEntityWithRemovedIds.insert(Skribisto::Domain::Entities::Scene, removedIdsResult.value());

    emit m_signalHolder->removed(removedIdsResult.value());

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithRemovedIds);
}

Result<QHash<int, QList<int>>> SceneRepository::changeActiveStatusInCascade(QList<int> ids, bool active)
{
    QWriteLocker locker(&m_lock);
    QHash<int, QList<int>> returnedHashOfEntityWithActiveChangedIds;

    // cahnge active status of the paragraphs in cascade

    Qleany::Domain::RelationshipInfo sceneParagraphParagraphsRelationship;
    for (const Qleany::Domain::RelationshipInfo &relationship : Skribisto::Domain::Scene::schema.relationships)
    {
        if (relationship.rightEntityId == Skribisto::Domain::Entities::SceneParagraph &&
            relationship.fieldName == "paragraphs")
        {
            sceneParagraphParagraphsRelationship = relationship;
            break;
        }
    }

    for (int entityId : ids)
    {
        if (sceneParagraphParagraphsRelationship.strength == Qleany::Domain::RelationshipStrength::Strong)
        {
            // get foreign entities

            QList<Skribisto::Domain::SceneParagraph> foreignParagraphs =
                this->fetchParagraphsLoader().operator()(entityId);

            if (foreignParagraphs.isEmpty())
            {
                continue;
            }

            QList<int> foreignIds;

            for (const auto &sceneParagraph : foreignParagraphs)
            {
                foreignIds.append(sceneParagraph.id());
            }

            auto changeResult = m_sceneParagraphRepository->changeActiveStatusInCascade(foreignIds, active);

            QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changeResult)

            returnedHashOfEntityWithActiveChangedIds.insert(changeResult.value());
        }
    }

    // finally change the entites of this repository

    Result<QList<int>> changedIdsResult = this->databaseTable()->changeActiveStatus(ids, active);

    QLN_RETURN_IF_ERROR(QHash<int QLN_COMMA QList<int>>, changedIdsResult)

    returnedHashOfEntityWithActiveChangedIds.insert(Skribisto::Domain::Entities::Scene, changedIdsResult.value());
    emit m_signalHolder->activeStatusChanged(changedIdsResult.value(), active);

    return Result<QHash<int, QList<int>>>(returnedHashOfEntityWithActiveChangedIds);
}
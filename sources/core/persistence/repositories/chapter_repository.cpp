#include "chapter_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

ChapterRepository::ChapterRepository(Domain::EntitySchema *entitySchema,
                                     InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase,
                                     InterfaceSceneRepository *sceneRepository)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::Chapter>(chapterDatabase),
      m_sceneRepository(sceneRepository)
{
}

Domain::Chapter::ScenesLoader ChapterRepository::fetchScenesLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "scenes" property in the entity Chapter using staticMetaObject
    int propertyIndex = Domain::Chapter::staticMetaObject.indexOfProperty("scenes");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Chapter doesn't have a property named scenes";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto result = this->databaseTable()->getRelatedForeignIds(entityId, "scenes");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();

            return QList<Domain::Scene>();
        }

        QList<int> foreignIds = result.value();

        QList<Domain::Scene> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_sceneRepository->get(foreignId).value());
        }
        return foreignEntities;
    };
}

QHash<int, QList<int>> ChapterRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the scenes in cascade

    Domain::EntitySchema::EntitySchemaRelationship sceneRelationship =
        m_entitySchema->relationship(Domain::Chapter::enumValue(), "scenes");

    for (int entityId : ids)
    {
        if (sceneRelationship.dependency == Domain::EntitySchema::Strong)
        {

            QList<Domain::Scene> foreignScenes = this->fetchScenesLoader().operator()(entityId);

            QList<int> foreignIds;

            for (const auto &scene : foreignScenes)
            {
                foreignIds.append(scene.id());
            }

            returnedHashOfEntityWithRemovedIds.insert(m_sceneRepository->removeInCascade(foreignIds));
        }
    }

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::Chapter, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
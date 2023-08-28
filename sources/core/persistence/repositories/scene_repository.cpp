#include "scene_repository.h"
#ifdef QT_DEBUG
#include <QDebug>
#include <QObject>
#endif

using namespace Repository;
using namespace Contracts::Persistence;

SceneRepository::SceneRepository(Domain::EntitySchema *entitySchema,
                                 InterfaceDatabaseTable<Domain::Scene> *sceneDatabase,
                                 InterfaceSceneParagraphRepository *sceneParagraphRepository)
    : m_entitySchema(entitySchema), Repository::GenericRepository<Domain::Scene>(sceneDatabase),
      m_sceneParagraphRepository(sceneParagraphRepository)
{
}

Domain::Scene::ParagraphsLoader SceneRepository::fetchParagraphsLoader()
{
#ifdef QT_DEBUG
    // verify the presence of "paragraphs" property in the entity Scene using staticMetaObject
    int propertyIndex = Domain::Scene::staticMetaObject.indexOfProperty("paragraphs");
    if (propertyIndex == -1)
    {
        qCritical() << "The entity Scene doesn't have a property named paragraphs";
        qFatal("The application will now exit");
    }
#endif

    return [this](int entityId) {
        auto result = this->databaseTable()->getRelatedForeignIds(entityId, "paragraphs");
        if (result.isError())
        {
            qCritical() << result.error().code() << result.error().message() << result.error().data();

            return QList<Domain::SceneParagraph>();
        }

        QList<int> foreignIds = result.value();

        QList<Domain::SceneParagraph> foreignEntities;
        for (int foreignId : foreignIds)
        {
            foreignEntities.append(m_sceneParagraphRepository->get(foreignId).value());
        }
        return foreignEntities;
    };
}

QHash<int, QList<int>> SceneRepository::removeInCascade(QList<int> ids)
{
    QHash<int, QList<int>> returnedHashOfEntityWithRemovedIds;

    // remove the paragraphs in cascade

    Domain::EntitySchema::EntitySchemaRelationship sceneParagraphRelationship =
        m_entitySchema->relationship(Domain::Scene::enumValue(), "paragraphs");

    for (int entityId : ids)
    {
        if (sceneParagraphRelationship.dependency == Domain::EntitySchema::Strong)
        {

            QList<Domain::SceneParagraph> foreignParagraphs = this->fetchParagraphsLoader().operator()(entityId);

            QList<int> foreignIds;

            for (const auto &sceneParagraph : foreignParagraphs)
            {
                foreignIds.append(sceneParagraph.id());
            }

            returnedHashOfEntityWithRemovedIds.insert(m_sceneParagraphRepository->removeInCascade(foreignIds));
        }
    }

    // finally remove the entites of this repository

    Result<QList<int>> removedIds = this->databaseTable()->remove(ids);
    returnedHashOfEntityWithRemovedIds.insert(Domain::Entities::Scene, removedIds.value());

    return returnedHashOfEntityWithRemovedIds;
}
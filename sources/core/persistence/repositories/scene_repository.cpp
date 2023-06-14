#include "scene_repository.h"

using namespace Repository;

SceneRepository::SceneRepository(InterfaceDatabaseTable<Domain::Scene>          *sceneDatabase,
                                 InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Scene>(sceneDatabase), m_sceneParagraphDatabase(
        sceneParagraphDatabase)
{}

Domain::Scene::ParagraphsLoader SceneRepository::fetchParagraphsLoader()
{
    return [this](int entityId) {
               auto result = m_sceneDatabase->getRelatedForeignIds(entityId, "paragraphs");

               if (result.isError())
               {
                   qCritical() << result.error().code() << result.error().message() << result.error().data();

                   return QList<Domain::SceneParagraph>();
               }

               QList<int> foreignIds = result.value();

               QList<Domain::SceneParagraph> foreignEntities;

               for (int foreignId : foreignIds)
               {
                   foreignEntities.append(m_sceneParagraphDatabase->get(foreignId).value());
               }
               return foreignEntities;
    };
}

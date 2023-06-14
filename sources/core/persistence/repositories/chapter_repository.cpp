#include "chapter_repository.h"

using namespace Repository;

ChapterRepository::ChapterRepository(InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase,
                                     InterfaceDatabaseTable<Domain::Scene>   *sceneDatabase)
    : QObject(nullptr), Repository::GenericRepository<Domain::Chapter>(chapterDatabase), m_sceneDatabase(sceneDatabase)
{}

Domain::Chapter::ScenesLoader ChapterRepository::fetchScenesLoader()
{
    return [this](int entityId) {
               auto result = m_chapterDatabase->getRelatedForeignIds(entityId, "scenes");

               if (result.isError())
               {
                   qCritical() << result.error().code() << result.error().message() << result.error().data();

                   return QList<Domain::Scene>();
               }

               QList<int> foreignIds = result.value();

               QList<Domain::Scene> foreignEntities;

               for (int foreignId : foreignIds)
               {
                   foreignEntities.append(m_sceneDatabase->get(foreignId).value());
               }
               return foreignEntities;
    };
}

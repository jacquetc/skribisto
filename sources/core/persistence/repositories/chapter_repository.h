#pragma once

#include "chapter.h"

#include "persistence/interface_scene_repository.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_chapter_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT ChapterRepository : public Repository::GenericRepository<Domain::Chapter>,
                                                 public Contracts::Persistence::InterfaceChapterRepository
{
  public:
    explicit ChapterRepository(Domain::EntitySchema *entitySchema,
                               InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase,
                               InterfaceSceneRepository *sceneRepository);

    Domain::Chapter::ScenesLoader fetchScenesLoader() override;

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;

    InterfaceSceneRepository *m_sceneRepository;
};

} // namespace Repository
#pragma once

#include "scene.h"

#include "persistence/interface_scene_paragraph_repository.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_scene_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT SceneRepository : public Repository::GenericRepository<Domain::Scene>,
                                               public Contracts::Persistence::InterfaceSceneRepository
{
  public:
    explicit SceneRepository(Domain::EntitySchema *entitySchema, InterfaceDatabaseTable<Domain::Scene> *sceneDatabase,
                             InterfaceSceneParagraphRepository *sceneParagraphRepository);

    Domain::Scene::ParagraphsLoader fetchParagraphsLoader() override;

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;

    InterfaceSceneParagraphRepository *m_sceneParagraphRepository;
};

} // namespace Repository
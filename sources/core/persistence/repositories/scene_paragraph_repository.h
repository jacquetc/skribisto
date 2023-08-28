#pragma once

#include "scene_paragraph.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT SceneParagraphRepository : public Repository::GenericRepository<Domain::SceneParagraph>,
                                                        public Contracts::Persistence::InterfaceSceneParagraphRepository
{
  public:
    explicit SceneParagraphRepository(Domain::EntitySchema *entitySchema,
                                      InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase);

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;
};

} // namespace Repository
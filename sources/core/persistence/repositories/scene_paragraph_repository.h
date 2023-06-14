#pragma once

#include "scene_paragraph.h"
#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_scene_paragraph_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository {
class SKR_PERSISTENCE_EXPORT SceneParagraphRepository : public QObject,
                                                        public Repository::GenericRepository<Domain::SceneParagraph>,
                                                        public Contracts::Persistence::InterfaceSceneParagraphRepository
{
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceSceneParagraphRepository)

public:

    explicit SceneParagraphRepository(InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase);

private:

    InterfaceDatabaseTable<Domain::SceneParagraph> *m_sceneParagraphDatabase;
};
} // namespace Repository

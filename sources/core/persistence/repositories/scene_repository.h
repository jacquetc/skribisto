#pragma once

#include "scene.h"
#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_scene_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository {
class SKR_PERSISTENCE_EXPORT SceneRepository : public QObject,
                                               public Repository::GenericRepository<Domain::Scene>,
                                               public Contracts::Persistence::InterfaceSceneRepository {
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceSceneRepository)

public:

    explicit SceneRepository(InterfaceDatabaseTable<Domain::Scene>          *sceneDatabase,
                             InterfaceDatabaseTable<Domain::SceneParagraph> *sceneParagraphDatabase);


    Domain::Scene::ParagraphsLoader fetchParagraphsLoader() override;

private:

    InterfaceDatabaseTable<Domain::Scene> *m_sceneDatabase;

    InterfaceDatabaseTable<Domain::SceneParagraph> *m_sceneParagraphDatabase;
};
} // namespace Repository

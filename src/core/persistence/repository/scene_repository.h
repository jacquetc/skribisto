// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "persistence_export.h"
#include "repository/interface_scene_paragraph_repository.h"
#include "repository/interface_scene_repository.h"
#include "scene.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT SceneRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Scene>,
      public Skribisto::Contracts::Repository::InterfaceSceneRepository
{
  public:
    explicit SceneRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Scene> *sceneDatabase,
                             InterfaceSceneParagraphRepository *sceneParagraphRepository);

    SignalHolder *signalHolder() override;

    Result<Skribisto::Domain::Scene> update(Skribisto::Domain::Scene &&entity) override;
    Result<Skribisto::Domain::Scene> getWithDetails(int entityId) override;

    Skribisto::Domain::Scene::ParagraphsLoader fetchParagraphsLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceSceneParagraphRepository *m_sceneParagraphRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
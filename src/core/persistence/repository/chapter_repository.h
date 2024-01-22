// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter.h"
#include "persistence_export.h"
#include "repository/interface_chapter_repository.h"
#include "repository/interface_scene_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT ChapterRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Chapter>,
      public Skribisto::Contracts::Repository::InterfaceChapterRepository
{
  public:
    explicit ChapterRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Chapter> *chapterDatabase,
                               InterfaceSceneRepository *sceneRepository);

    SignalHolder *signalHolder() override;

    Result<Skribisto::Domain::Chapter> update(Skribisto::Domain::Chapter &&entity) override;
    Result<Skribisto::Domain::Chapter> getWithDetails(int entityId) override;

    Skribisto::Domain::Chapter::ScenesLoader fetchScenesLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceSceneRepository *m_sceneRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "atelier.h"
#include "persistence_export.h"
#include "repository/interface_atelier_repository.h"
#include "repository/interface_git_repository.h"
#include "repository/interface_workspace_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT AtelierRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Atelier>,
      public Skribisto::Contracts::Repository::InterfaceAtelierRepository
{
  public:
    explicit AtelierRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Atelier> *atelierDatabase,
                               InterfaceGitRepository *gitRepository,
                               InterfaceWorkspaceRepository *workspaceRepository);

    SignalHolder *signalHolder() override;

    Result<Skribisto::Domain::Atelier> update(Skribisto::Domain::Atelier &&entity) override;
    Result<Skribisto::Domain::Atelier> getWithDetails(int entityId) override;

    Skribisto::Domain::Atelier::GitLoader fetchGitLoader() override;

    Skribisto::Domain::Atelier::WorkspacesLoader fetchWorkspacesLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceGitRepository *m_gitRepository;
    InterfaceWorkspaceRepository *m_workspaceRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
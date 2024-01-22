// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "persistence_export.h"
#include "repository/interface_file_repository.h"
#include "repository/interface_workspace_repository.h"
#include "workspace.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT WorkspaceRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Workspace>,
      public Skribisto::Contracts::Repository::InterfaceWorkspaceRepository
{
  public:
    explicit WorkspaceRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Workspace> *workspaceDatabase,
                                 InterfaceFileRepository *fileRepository);

    SignalHolder *signalHolder() override;

    Result<Skribisto::Domain::Workspace> update(Skribisto::Domain::Workspace &&entity) override;
    Result<Skribisto::Domain::Workspace> getWithDetails(int entityId) override;

    Skribisto::Domain::Workspace::FilesLoader fetchFilesLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceFileRepository *m_fileRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
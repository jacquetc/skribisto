// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "git.h"
#include "persistence_export.h"
#include "repository/interface_git_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT GitRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Git>,
      public Skribisto::Contracts::Repository::InterfaceGitRepository
{
  public:
    explicit GitRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Git> *gitDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "persistence_export.h"
#include "recent_atelier.h"
#include "repository/interface_recent_atelier_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT RecentAtelierRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::RecentAtelier>,
      public Skribisto::Contracts::Repository::InterfaceRecentAtelierRepository
{
  public:
    explicit RecentAtelierRepository(
        InterfaceDatabaseTableGroup<Skribisto::Domain::RecentAtelier> *recentAtelierDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
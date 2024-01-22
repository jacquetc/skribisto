// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "file.h"
#include "persistence_export.h"
#include "repository/interface_file_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT FileRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::File>,
      public Skribisto::Contracts::Repository::InterfaceFileRepository
{
  public:
    explicit FileRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::File> *fileDatabase);

    SignalHolder *signalHolder() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
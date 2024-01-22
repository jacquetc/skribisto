// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "atelier.h"
#include "contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Skribisto::Contracts::Repository
{

class SKRIBISTO_CONTRACTS_EXPORT InterfaceAtelierRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Skribisto::Domain::Atelier>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceAtelierRepository()
    {
    }

    virtual Result<Skribisto::Domain::Atelier> update(Skribisto::Domain::Atelier &&entity) = 0;
    virtual Result<Skribisto::Domain::Atelier> getWithDetails(int entityId) = 0;

    virtual Skribisto::Domain::Atelier::GitLoader fetchGitLoader() = 0;

    virtual Skribisto::Domain::Atelier::WorkspacesLoader fetchWorkspacesLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace Skribisto::Contracts::Repository
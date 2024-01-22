// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter.h"
#include "contracts_export.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Skribisto::Contracts::Repository
{

class SKRIBISTO_CONTRACTS_EXPORT InterfaceChapterRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Skribisto::Domain::Chapter>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceChapterRepository()
    {
    }

    virtual Result<Skribisto::Domain::Chapter> update(Skribisto::Domain::Chapter &&entity) = 0;
    virtual Result<Skribisto::Domain::Chapter> getWithDetails(int entityId) = 0;

    virtual Skribisto::Domain::Chapter::ScenesLoader fetchScenesLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace Skribisto::Contracts::Repository
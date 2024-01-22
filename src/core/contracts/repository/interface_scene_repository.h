// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "contracts_export.h"
#include "scene.h"
#include <QObject>
#include <qleany/common/result.h>
#include <qleany/contracts/repository/interface_generic_repository.h>
#include <qleany/contracts/repository/interface_repository.h>

using namespace Qleany;

namespace Skribisto::Contracts::Repository
{

class SKRIBISTO_CONTRACTS_EXPORT InterfaceSceneRepository
    : public virtual Qleany::Contracts::Repository::InterfaceGenericRepository<Skribisto::Domain::Scene>,
      public Qleany::Contracts::Repository::InterfaceRepository
{
  public:
    virtual ~InterfaceSceneRepository()
    {
    }

    virtual Result<Skribisto::Domain::Scene> update(Skribisto::Domain::Scene &&entity) = 0;
    virtual Result<Skribisto::Domain::Scene> getWithDetails(int entityId) = 0;

    virtual Skribisto::Domain::Scene::ParagraphsLoader fetchParagraphsLoader() = 0;

    virtual Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) = 0;
    virtual Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) = 0;
};
} // namespace Skribisto::Contracts::Repository
#pragma once

#include "scene.h"
#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence {
class SKR_CONTRACTS_EXPORT InterfaceSceneRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Scene>,
      public InterfaceRepository {
public:

    virtual ~InterfaceSceneRepository()
    {}

    virtual Domain::Scene::ParagraphsLoader fetchParagraphsLoader() = 0;
};
} // namespace Contracts::Persistence
#define InterfaceSceneRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceSceneRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceSceneRepository, InterfaceSceneRepository_iid)

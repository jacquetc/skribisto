#pragma once

#include "scene_paragraph.h"
#include "contracts_global.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence {
class SKR_CONTRACTS_EXPORT InterfaceSceneParagraphRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::SceneParagraph>,
      public InterfaceRepository {
public:

    virtual ~InterfaceSceneParagraphRepository()
    {}
};
} // namespace Contracts::Persistence
#define InterfaceSceneParagraphRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceSceneParagraphRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceSceneParagraphRepository, InterfaceSceneParagraphRepository_iid)

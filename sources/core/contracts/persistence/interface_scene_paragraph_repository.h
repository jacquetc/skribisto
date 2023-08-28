#pragma once

#include "contracts_global.h"
#include "entity_schema.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"
#include "scene_paragraph.h"

namespace Contracts::Persistence
{
class SKR_CONTRACTS_EXPORT InterfaceSceneParagraphRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::SceneParagraph>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceSceneParagraphRepository()
    {
    }

    virtual QHash<int, QList<int>> removeInCascade(QList<int> ids) = 0;
};
} // namespace Contracts::Persistence
#define InterfaceSceneParagraphRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceSceneParagraphRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceSceneParagraphRepository, InterfaceSceneParagraphRepository_iid)
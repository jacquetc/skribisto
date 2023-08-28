#pragma once

#include "book.h"
#include "contracts_global.h"
#include "entity_schema.h"
#include "interface_generic_repository.h"
#include "interface_repository.h"

namespace Contracts::Persistence
{
class SKR_CONTRACTS_EXPORT InterfaceBookRepository
    : public virtual Contracts::Persistence::InterfaceGenericRepository<Domain::Book>,
      public InterfaceRepository
{
  public:
    virtual ~InterfaceBookRepository()
    {
    }

    virtual Domain::Book::ChaptersLoader fetchChaptersLoader() = 0;

    virtual Domain::Book::AuthorLoader fetchAuthorLoader() = 0;

    virtual QHash<int, QList<int>> removeInCascade(QList<int> ids) = 0;
};
} // namespace Contracts::Persistence
#define InterfaceBookRepository_iid "eu.skribisto.Contracts.Persistence.InterfaceBookRepository"
Q_DECLARE_INTERFACE(Contracts::Persistence::InterfaceBookRepository, InterfaceBookRepository_iid)
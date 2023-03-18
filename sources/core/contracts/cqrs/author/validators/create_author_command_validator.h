#pragma once

#include "contracts_global.h"
#include "dto/author/create_author_dto.h"
#include "persistence/interface_author_repository.h"
#include "result.h"

using namespace Contracts::Persistence;
using namespace Contracts::DTO::Author;

namespace Contracts::CQRS::Author::Validators
{
class SKR_CONTRACTS_EXPORT CreateAuthorCommandValidator
{
  public:
    CreateAuthorCommandValidator(QSharedPointer<InterfaceAuthorRepository> authorRepository)
        : m_repository(authorRepository)
    {
    }

    Result<void> validate(const CreateAuthorDTO &dto) const
    {

        if (dto.relative().isNull())
        {
            return Result<void>(Error("CreateAuthorCommandValidator", Error::Critical, "relative_uuid_missing"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
};
} // namespace Contracts::CQRS::Author::Validators

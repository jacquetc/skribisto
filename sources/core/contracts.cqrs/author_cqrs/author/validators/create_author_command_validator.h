#pragma once

#include "author/create_author_dto.h"

#include "persistence/interface_author_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Author;

namespace Contracts::CQRS::Author::Validators
{
class CreateAuthorCommandValidator
{
  public:
    CreateAuthorCommandValidator(InterfaceAuthorRepository *authorRepository) : m_authorRepository(authorRepository)
    {
    }

    Result<void> validate(const CreateAuthorDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceAuthorRepository *m_authorRepository;
};
} // namespace Contracts::CQRS::Author::Validators
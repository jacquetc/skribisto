#pragma once

#include "persistence/interface_author_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

namespace Contracts::CQRS::Author::Validators
{
class RemoveAuthorCommandValidator
{
  public:
    RemoveAuthorCommandValidator(InterfaceAuthorRepository *authorRepository) : m_authorRepository(authorRepository)
    {
    }

    Result<void> validate(int id) const

    {

        Result<bool> existsResult = m_authorRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceAuthorRepository *m_authorRepository;
};
} // namespace Contracts::CQRS::Author::Validators
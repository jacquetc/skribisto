#pragma once

#include "author/update_author_dto.h"

#include "persistence/interface_author_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Author;

namespace Contracts::CQRS::Author::Validators
{
class UpdateAuthorCommandValidator
{
  public:
    UpdateAuthorCommandValidator(QSharedPointer<InterfaceAuthorRepository> authorRepository)
        : m_authorRepository(authorRepository)

    {
    }

    Result<void> validate(const UpdateAuthorDTO &dto) const

    {

        Result<bool> existsResult = m_authorRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceAuthorRepository> m_authorRepository;
};
} // namespace Contracts::CQRS::Author::Validators
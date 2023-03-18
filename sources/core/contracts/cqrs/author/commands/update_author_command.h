#pragma once

#include "contracts_global.h"
#include "dto/author/update_author_dto.h"

namespace Contracts::CQRS::Author::Commands
{
class SKR_CONTRACTS_EXPORT UpdateAuthorCommand
{
  public:
    UpdateAuthorCommand()
    {
    }

    Contracts::DTO::Author::UpdateAuthorDTO req;
};
} // namespace Contracts::CQRS::Author::Commands

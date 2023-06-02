#pragma once

#include "contracts_global.h"
#include "create_author_dto.h"

namespace Contracts::CQRS::Author::Commands
{
class SKR_CONTRACTS_EXPORT CreateAuthorCommand
{
  public:
    CreateAuthorCommand()
    {
    }

    Contracts::DTO::Author::CreateAuthorDTO req;
};
} // namespace Contracts::CQRS::Author::Commands

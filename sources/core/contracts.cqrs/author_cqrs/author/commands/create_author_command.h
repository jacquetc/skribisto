#pragma once

#include "author/create_author_dto.h"

namespace Contracts::CQRS::Author::Commands
{
class CreateAuthorCommand
{
  public:
    CreateAuthorCommand()
    {
    }

    Contracts::DTO::Author::CreateAuthorDTO req;
};
} // namespace Contracts::CQRS::Author::Commands
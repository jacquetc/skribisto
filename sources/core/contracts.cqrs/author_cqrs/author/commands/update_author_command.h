#pragma once

#include "author/update_author_dto.h"

namespace Contracts::CQRS::Author::Commands
{
class UpdateAuthorCommand
{
  public:
    UpdateAuthorCommand()
    {
    }

    Contracts::DTO::Author::UpdateAuthorDTO req;
};
} // namespace Contracts::CQRS::Author::Commands
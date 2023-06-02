#pragma once

#include "contracts_global.h"
#include "create_chapter_dto.h"

namespace Contracts::CQRS::Chapter::Commands
{
class SKR_CONTRACTS_EXPORT CreateChapterCommand
{
  public:
    CreateChapterCommand()
    {
    }

    Contracts::DTO::Chapter::CreateChapterDTO req;
};
} // namespace Contracts::CQRS::Chapter::Commands

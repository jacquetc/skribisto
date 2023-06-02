#pragma once

#include "contracts_global.h"
#include "update_chapter_dto.h"

namespace Contracts::CQRS::Chapter::Commands
{
class SKR_CONTRACTS_EXPORT UpdateChapterCommand
{
  public:
    UpdateChapterCommand()
    {
    }

    Contracts::DTO::Chapter::UpdateChapterDTO req;
};
} // namespace Contracts::CQRS::Chapter::Commands

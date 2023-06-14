#pragma once

#include "chapter/update_chapter_dto.h"

namespace Contracts::CQRS::Chapter::Commands
{
class UpdateChapterCommand
{
  public:
    UpdateChapterCommand()
    {
    }

    Contracts::DTO::Chapter::UpdateChapterDTO req;
};
} // namespace Contracts::CQRS::Chapter::Commands
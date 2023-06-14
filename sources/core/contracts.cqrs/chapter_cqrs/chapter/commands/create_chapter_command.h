#pragma once

#include "chapter/create_chapter_dto.h"

namespace Contracts::CQRS::Chapter::Commands
{
class CreateChapterCommand
{
  public:
    CreateChapterCommand()
    {
    }

    Contracts::DTO::Chapter::CreateChapterDTO req;
};
} // namespace Contracts::CQRS::Chapter::Commands
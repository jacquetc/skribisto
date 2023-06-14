#pragma once

#include "chapter/move_chapter_dto.h"

namespace Contracts::CQRS::Chapter::Commands
{
class MoveChapterCommand
{
  public:
    MoveChapterCommand()
    {
    }

    Contracts::DTO::Chapter::MoveChapterDTO req;
};
} // namespace Contracts::CQRS::Chapter::Commands
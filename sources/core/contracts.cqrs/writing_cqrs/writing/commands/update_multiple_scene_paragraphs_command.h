#pragma once

#include "writing/update_multiple_scene_paragraphs_dto.h"

namespace Contracts::CQRS::Writing::Commands
{
class UpdateMultipleSceneParagraphsCommand
{
  public:
    UpdateMultipleSceneParagraphsCommand()
    {
    }

    Contracts::DTO::Writing::UpdateMultipleSceneParagraphsDTO req;
};
} // namespace Contracts::CQRS::Writing::Commands
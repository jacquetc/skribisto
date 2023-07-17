#pragma once

#include "writing/update_scene_paragraph_dto.h"

namespace Contracts::CQRS::Writing::Commands
{
class UpdateSceneParagraphCommand
{
  public:
    UpdateSceneParagraphCommand()
    {
    }

    Contracts::DTO::Writing::UpdateSceneParagraphDTO req;
};
} // namespace Contracts::CQRS::Writing::Commands
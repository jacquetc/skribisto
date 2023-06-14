#pragma once

#include "scene_paragraph/update_scene_paragraph_dto.h"

namespace Contracts::CQRS::SceneParagraph::Commands
{
class UpdateSceneParagraphCommand
{
  public:
    UpdateSceneParagraphCommand()
    {
    }

    Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO req;
};
} // namespace Contracts::CQRS::SceneParagraph::Commands
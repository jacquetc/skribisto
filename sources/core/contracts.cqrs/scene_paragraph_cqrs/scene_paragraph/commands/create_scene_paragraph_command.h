#pragma once

#include "scene_paragraph/create_scene_paragraph_dto.h"

namespace Contracts::CQRS::SceneParagraph::Commands
{
class CreateSceneParagraphCommand
{
  public:
    CreateSceneParagraphCommand()
    {
    }

    Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO req;
};
} // namespace Contracts::CQRS::SceneParagraph::Commands
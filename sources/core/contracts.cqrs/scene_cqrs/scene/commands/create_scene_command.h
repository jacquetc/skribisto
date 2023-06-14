#pragma once

#include "scene/create_scene_dto.h"

namespace Contracts::CQRS::Scene::Commands
{
class CreateSceneCommand
{
  public:
    CreateSceneCommand()
    {
    }

    Contracts::DTO::Scene::CreateSceneDTO req;
};
} // namespace Contracts::CQRS::Scene::Commands
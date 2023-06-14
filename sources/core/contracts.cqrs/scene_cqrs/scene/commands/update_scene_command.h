#pragma once

#include "scene/update_scene_dto.h"

namespace Contracts::CQRS::Scene::Commands
{
class UpdateSceneCommand
{
  public:
    UpdateSceneCommand()
    {
    }

    Contracts::DTO::Scene::UpdateSceneDTO req;
};
} // namespace Contracts::CQRS::Scene::Commands
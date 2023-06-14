#pragma once

#include "scene/move_scene_dto.h"

namespace Contracts::CQRS::Scene::Commands
{
class MoveSceneCommand
{
  public:
    MoveSceneCommand()
    {
    }

    Contracts::DTO::Scene::MoveSceneDTO req;
};
} // namespace Contracts::CQRS::Scene::Commands
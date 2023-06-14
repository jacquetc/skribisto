#pragma once

namespace Contracts::CQRS::Scene::Commands
{
class RemoveSceneCommand
{
  public:
    RemoveSceneCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Scene::Commands
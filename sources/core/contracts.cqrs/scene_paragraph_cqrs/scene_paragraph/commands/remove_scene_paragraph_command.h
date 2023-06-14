#pragma once

namespace Contracts::CQRS::SceneParagraph::Commands
{
class RemoveSceneParagraphCommand
{
  public:
    RemoveSceneParagraphCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::SceneParagraph::Commands
#pragma once

namespace Contracts::CQRS::Chapter::Commands
{
class RemoveChapterCommand
{
  public:
    RemoveChapterCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Chapter::Commands
#pragma once

namespace Contracts::CQRS::Author::Commands
{
class RemoveAuthorCommand
{
  public:
    RemoveAuthorCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Author::Commands
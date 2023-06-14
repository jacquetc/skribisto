#pragma once

namespace Contracts::CQRS::Author::Queries
{
class GetAuthorQuery
{
  public:
    GetAuthorQuery()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Author::Queries
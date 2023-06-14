#pragma once

namespace Contracts::CQRS::Chapter::Queries
{
class GetChapterQuery
{
  public:
    GetChapterQuery()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Chapter::Queries
#pragma once

#include "writing/get_all_scene_paragraphs_dto.h"

namespace Contracts::CQRS::Writing::Queries
{
class GetAllSceneParagraphsQuery
{
  public:
    GetAllSceneParagraphsQuery()
    {
    }

    Contracts::DTO::Writing::GetAllSceneParagraphsDTO req;
};
} // namespace Contracts::CQRS::Writing::Queries
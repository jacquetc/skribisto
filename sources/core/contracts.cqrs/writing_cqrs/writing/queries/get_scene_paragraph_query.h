#pragma once

#include "writing/get_scene_paragraph_dto.h"

namespace Contracts::CQRS::Writing::Queries
{
class GetSceneParagraphQuery
{
  public:
    GetSceneParagraphQuery()
    {
    }

    Contracts::DTO::Writing::GetSceneParagraphDTO req;
};
} // namespace Contracts::CQRS::Writing::Queries
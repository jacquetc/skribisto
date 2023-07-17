#pragma once

#include "writing/get_scene_paragraph_id_dto.h"

namespace Contracts::CQRS::Writing::Queries
{
class GetSceneParagraphIdQuery
{
  public:
    GetSceneParagraphIdQuery()
    {
    }

    Contracts::DTO::Writing::GetSceneParagraphIdDTO req;
};
} // namespace Contracts::CQRS::Writing::Queries
#pragma once

#include "writing/get_scene_paragraph_id_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Writing;

namespace Contracts::CQRS::Writing::Validators
{
class GetSceneParagraphIdQueryValidator
{
  public:
    GetSceneParagraphIdQueryValidator(QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
        : m_sceneParagraphRepository(sceneParagraphRepository)
    {
    }

    Result<void> validate(const GetSceneParagraphIdDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
};
} // namespace Contracts::CQRS::Writing::Validators
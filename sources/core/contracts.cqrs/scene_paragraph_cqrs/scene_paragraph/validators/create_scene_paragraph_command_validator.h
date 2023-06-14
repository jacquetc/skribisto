#pragma once

#include "scene_paragraph/create_scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::SceneParagraph;

namespace Contracts::CQRS::SceneParagraph::Validators
{
class CreateSceneParagraphCommandValidator
{
  public:
    CreateSceneParagraphCommandValidator(QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
        : m_sceneParagraphRepository(sceneParagraphRepository)

    {
    }

    Result<void> validate(const CreateSceneParagraphDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
};
} // namespace Contracts::CQRS::SceneParagraph::Validators
#pragma once

#include "scene_paragraph/update_scene_paragraph_dto.h"

#include "persistence/interface_scene_paragraph_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::SceneParagraph;

namespace Contracts::CQRS::SceneParagraph::Validators
{
class UpdateSceneParagraphCommandValidator
{
  public:
    UpdateSceneParagraphCommandValidator(QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
        : m_sceneParagraphRepository(sceneParagraphRepository)

    {
    }

    Result<void> validate(const UpdateSceneParagraphDTO &dto) const

    {

        Result<bool> existsResult = m_sceneParagraphRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
};
} // namespace Contracts::CQRS::SceneParagraph::Validators
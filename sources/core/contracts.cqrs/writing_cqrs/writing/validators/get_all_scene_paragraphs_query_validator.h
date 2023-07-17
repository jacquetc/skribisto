#pragma once

#include "writing/get_all_scene_paragraphs_dto.h"

#include "persistence/interface_scene_repository.h"

#include "persistence/interface_scene_paragraph_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Writing;

namespace Contracts::CQRS::Writing::Validators
{
class GetAllSceneParagraphsQueryValidator
{
  public:
    GetAllSceneParagraphsQueryValidator(QSharedPointer<InterfaceSceneRepository> sceneRepository,
                                        QSharedPointer<InterfaceSceneParagraphRepository> sceneParagraphRepository)
        : m_sceneRepository(sceneRepository), m_sceneParagraphRepository(sceneParagraphRepository)
    {
    }

    Result<void> validate(const GetAllSceneParagraphsDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;

    QSharedPointer<InterfaceSceneParagraphRepository> m_sceneParagraphRepository;
};
} // namespace Contracts::CQRS::Writing::Validators
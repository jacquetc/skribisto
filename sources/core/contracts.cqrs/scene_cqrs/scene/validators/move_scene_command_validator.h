#pragma once

#include "scene/move_scene_dto.h"

#include "persistence/interface_chapter_repository.h"

#include "persistence/interface_scene_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Scene;

namespace Contracts::CQRS::Scene::Validators
{
class MoveSceneCommandValidator
{
  public:
    MoveSceneCommandValidator(InterfaceChapterRepository *chapterRepository, InterfaceSceneRepository *sceneRepository)
        : m_chapterRepository(chapterRepository), m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(const MoveSceneDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceChapterRepository *m_chapterRepository;

    InterfaceSceneRepository *m_sceneRepository;
};
} // namespace Contracts::CQRS::Scene::Validators
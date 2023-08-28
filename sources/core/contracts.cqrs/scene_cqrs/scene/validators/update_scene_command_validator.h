#pragma once

#include "scene/update_scene_dto.h"

#include "persistence/interface_scene_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Scene;

namespace Contracts::CQRS::Scene::Validators
{
class UpdateSceneCommandValidator
{
  public:
    UpdateSceneCommandValidator(InterfaceSceneRepository *sceneRepository) : m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(const UpdateSceneDTO &dto) const

    {

        Result<bool> existsResult = m_sceneRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    InterfaceSceneRepository *m_sceneRepository;
};
} // namespace Contracts::CQRS::Scene::Validators
#pragma once

#include "scene/create_scene_dto.h"

#include "persistence/interface_scene_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

using namespace Contracts::DTO::Scene;

namespace Contracts::CQRS::Scene::Validators
{
class CreateSceneCommandValidator
{
  public:
    CreateSceneCommandValidator(QSharedPointer<InterfaceSceneRepository> sceneRepository)
        : m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(const CreateSceneDTO &dto) const

    {

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
};
} // namespace Contracts::CQRS::Scene::Validators
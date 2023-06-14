#pragma once

#include "persistence/interface_scene_repository.h"

#include "result.h"

using namespace Contracts::Persistence;

namespace Contracts::CQRS::Scene::Validators
{
class RemoveSceneCommandValidator
{
  public:
    RemoveSceneCommandValidator(QSharedPointer<InterfaceSceneRepository> sceneRepository)
        : m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(int id) const

    {

        Result<bool> existsResult = m_sceneRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "id_already_exists"));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
    QSharedPointer<InterfaceSceneRepository> m_sceneRepository;
};
} // namespace Contracts::CQRS::Scene::Validators
#pragma once

#include "application_scene_export.h"
#include "persistence/interface_scene_repository.h"
#include "result.h"
#include "scene/commands/create_scene_command.h"
#include "scene/scene_dto.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;

namespace Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT CreateSceneCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateSceneCommandHandler(InterfaceSceneRepository *repository);

    Result<SceneDTO> handle(QPromise<Result<void>> &progressPromise, const CreateSceneCommand &request);
    Result<SceneDTO> restore();

  signals:
    void sceneCreated(Contracts::DTO::Scene::SceneDTO sceneDto);
    void sceneRemoved(int id);

  private:
    InterfaceSceneRepository *m_repository; // A pointer to the interface repositories object.
    Result<SceneDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateSceneCommand &request);
    Result<SceneDTO> restoreImpl();
    Result<SceneDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Commands
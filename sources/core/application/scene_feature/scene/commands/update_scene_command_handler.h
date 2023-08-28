#pragma once

#include "application_scene_export.h"
#include "scene/commands/update_scene_command.h"
#include "scene/scene_dto.h"

#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;

namespace Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT UpdateSceneCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateSceneCommandHandler(InterfaceSceneRepository *repository);
    Result<SceneDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateSceneCommand &request);
    Result<SceneDTO> restore();

  signals:
    void sceneUpdated(Contracts::DTO::Scene::SceneDTO sceneDto);

  private:
    InterfaceSceneRepository *m_repository;
    Result<SceneDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateSceneCommand &request);
    Result<SceneDTO> restoreImpl();
    Result<SceneDTO> saveOldState();
    Result<SceneDTO> m_newState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Commands
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_export.h"
#include "scene/commands/update_scene_command.h"
#include "scene/scene_dto.h"

#include "repository/interface_scene_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Commands;

namespace Skribisto::Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT UpdateSceneCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateSceneCommandHandler(InterfaceSceneRepository *repository);
    Result<SceneDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateSceneCommand &request);
    Result<SceneDTO> restore();

  signals:
    void sceneUpdated(Skribisto::Contracts::DTO::Scene::SceneDTO sceneDto);
    void sceneDetailsUpdated(int id);

  private:
    InterfaceSceneRepository *m_repository;
    Result<SceneDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateSceneCommand &request);
    Result<SceneDTO> restoreImpl();
    Result<SceneDTO> saveOldState();
    Result<SceneDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Scene::Commands
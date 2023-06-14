#pragma once

#include "application_scene_export.h"
#include "scene/commands/remove_scene_command.h"
#include "scene/scene_dto.h"

#include "persistence/interface_scene_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;

namespace Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT RemoveSceneCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveSceneCommandHandler(QSharedPointer<InterfaceSceneRepository> repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveSceneCommand &request);
    Result<int> restore();

  signals:
    void sceneCreated(Contracts::DTO::Scene::SceneDTO sceneDto);
    void sceneRemoved(int id);

  private:
    QSharedPointer<InterfaceSceneRepository> m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveSceneCommand &request);
    Result<int> restoreImpl();
    Domain::Scene m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Commands
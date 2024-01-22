// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_export.h"
#include "repository/interface_scene_repository.h"
#include "scene/commands/create_scene_command.h"
#include "scene/scene_dto.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Commands;

namespace Skribisto::Application::Features::Scene::Commands
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT CreateSceneCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateSceneCommandHandler(InterfaceSceneRepository *repository);

    Result<SceneDTO> handle(QPromise<Result<void>> &progressPromise, const CreateSceneCommand &request);
    Result<SceneDTO> restore();

  signals:
    void sceneCreated(Skribisto::Contracts::DTO::Scene::SceneDTO sceneDto);
    void sceneRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfaceSceneRepository *m_repository;
    Result<SceneDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateSceneCommand &request);
    Result<SceneDTO> restoreImpl();
    Result<Skribisto::Domain::Scene> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    QList<Skribisto::Domain::Scene> m_oldOwnerScenes;
    QList<Skribisto::Domain::Scene> m_ownerScenesNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Skribisto::Application::Features::Scene::Commands
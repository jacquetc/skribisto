// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_scene_export.h"
#include "scene/queries/get_scene_query.h"
#include "scene/scene_dto.h"

#include "repository/interface_scene_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Queries;

namespace Skribisto::Application::Features::Scene::Queries
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT GetSceneQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneQueryHandler(InterfaceSceneRepository *repository);
    Result<SceneDTO> handle(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query);

  private:
    InterfaceSceneRepository *m_repository;
    Result<SceneDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Scene::Queries
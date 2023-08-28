#pragma once

#include "application_scene_export.h"
#include "scene/queries/get_scene_query.h"
#include "scene/scene_with_details_dto.h"

#include "persistence/interface_scene_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Queries;

namespace Application::Features::Scene::Queries
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT GetSceneWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetSceneWithDetailsQueryHandler(InterfaceSceneRepository *repository);
    Result<SceneWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query);

  private:
    InterfaceSceneRepository *m_repository;
    Result<SceneWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Queries
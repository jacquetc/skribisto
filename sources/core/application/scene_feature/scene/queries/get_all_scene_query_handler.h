#pragma once

#include "application_scene_export.h"
#include "scene/scene_dto.h"

#include "persistence/interface_scene_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;

namespace Application::Features::Scene::Queries
{
class SKRIBISTO_APPLICATION_SCENE_EXPORT GetAllSceneQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllSceneQueryHandler(QSharedPointer<InterfaceSceneRepository> repository);
    Result<QList<SceneDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    QSharedPointer<InterfaceSceneRepository> m_repository;
    Result<QList<SceneDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Scene::Queries
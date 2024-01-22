// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "scene/create_scene_dto.h"
#include "scene/scene_dto.h"
#include "scene/scene_with_details_dto.h"
#include "scene/update_scene_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::Scene;

namespace Skribisto::Controller::Scene
{

class SKRIBISTO_CONTROLLER_EXPORT SceneController : public QObject
{
    Q_OBJECT
  public:
    explicit SceneController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                             QSharedPointer<EventDispatcher> eventDispatcher);

    static SceneController *instance();

    Q_INVOKABLE QCoro::Task<SceneDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<SceneWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE static Contracts::DTO::Scene::CreateSceneDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Scene::UpdateSceneDTO getUpdateDTO();

  public slots:

    QCoro::Task<SceneDTO> create(const CreateSceneDTO &dto);

    QCoro::Task<SceneDTO> update(const UpdateSceneDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<SceneController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    SceneController() = delete;
    SceneController(const SceneController &) = delete;
    SceneController &operator=(const SceneController &) = delete;
};

} // namespace Skribisto::Controller::Scene
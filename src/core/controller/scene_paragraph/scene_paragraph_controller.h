// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "scene_paragraph/create_scene_paragraph_dto.h"
#include "scene_paragraph/scene_paragraph_dto.h"
#include "scene_paragraph/update_scene_paragraph_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::SceneParagraph;

namespace Skribisto::Controller::SceneParagraph
{

class SKRIBISTO_CONTROLLER_EXPORT SceneParagraphController : public QObject
{
    Q_OBJECT
  public:
    explicit SceneParagraphController(InterfaceRepositoryProvider *repositoryProvider,
                                      ThreadedUndoRedoSystem *undo_redo_system,
                                      QSharedPointer<EventDispatcher> eventDispatcher);

    static SceneParagraphController *instance();

    Q_INVOKABLE QCoro::Task<SceneParagraphDTO> get(int id) const;

    Q_INVOKABLE static Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO getUpdateDTO();

  public slots:

    QCoro::Task<SceneParagraphDTO> create(const CreateSceneParagraphDTO &dto);

    QCoro::Task<SceneParagraphDTO> update(const UpdateSceneParagraphDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<SceneParagraphController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    SceneParagraphController() = delete;
    SceneParagraphController(const SceneParagraphController &) = delete;
    SceneParagraphController &operator=(const SceneParagraphController &) = delete;
};

} // namespace Skribisto::Controller::SceneParagraph
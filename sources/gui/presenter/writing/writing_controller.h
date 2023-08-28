#pragma once

#include "event_dispatcher.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "undo_redo/threaded_undo_redo_system.h"
#include "writing/scene_paragraph_changed_dto.h"
#include "writing/update_scene_paragraph_dto.h"

using namespace Contracts::DTO::Writing;
using namespace Presenter;
using namespace Contracts::Persistence;
using namespace Presenter::UndoRedo;

namespace Presenter::Writing
{
class SKR_PRESENTER_EXPORT WritingController : public QObject
{
    Q_OBJECT
  public:
    explicit WritingController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                               ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher);
    static WritingController *instance();
    void updateSceneParagraph(const UpdateSceneParagraphDTO &dto);

  signals:
    void sceneParagraphChanged(const SceneParagraphChangedDTO &dto);

  private:
    static QScopedPointer<WritingController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    EventDispatcher *m_eventDispatcher;
    WritingController() = delete;
    WritingController(const WritingController &) = delete;
    WritingController &operator=(const WritingController &) = delete;
};

} // namespace Presenter::Writing

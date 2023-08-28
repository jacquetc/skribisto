#pragma once

#include "event_dispatcher.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "system/load_system_dto.h"
#include "system/save_system_as_dto.h"
#include "undo_redo/threaded_undo_redo_system.h"

using namespace Contracts::DTO::System;
using namespace Presenter;
using namespace Contracts::Persistence;
using namespace Presenter::UndoRedo;

namespace Presenter::System
{

class SKR_PRESENTER_EXPORT SystemController : public QObject
{
    Q_OBJECT
  public:
    SystemController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                     ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher);
    static SystemController *instance();

    void loadSystem(const LoadSystemDTO &dto);

    void saveSystem();

    void saveSystemAs(const SaveSystemAsDTO &dto);

    void closeSystem();

  signals:
    void loadSystemProgressFinished();
    void loadSystemProgressRangeChanged(int minimum, int maximum);
    void loadSystemProgressTextChanged(const QString &progressText);
    void loadSystemProgressValueChanged(int progressValue);
    void systemLoaded();
    void systemSaved();
    void systemClosed();

  private:
    static QScopedPointer<SystemController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    EventDispatcher *m_eventDispatcher;
    SystemController() = delete;
    SystemController(const SystemController &) = delete;
    SystemController &operator=(const SystemController &) = delete;
};

} // namespace Presenter::System

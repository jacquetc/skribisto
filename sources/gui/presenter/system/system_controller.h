#pragma once

#include "dto/system/load_system_dto.h"
#include "dto/system/save_system_as_dto.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "result.h"
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
    SystemController(InterfaceRepositoryProvider *repositoryProvider);
    static SystemController *instance();

    static void loadSystem(const LoadSystemDTO &dto);

    static void saveSystem();

    static void saveSystemAs(const SaveSystemAsDTO &dto);

    static void closeSystem();

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
    static InterfaceRepositoryProvider *s_repositoryProvider;
    static ThreadedUndoRedoSystem *s_undo_redo_system;
    SystemController() = delete;
    SystemController(const SystemController &) = delete;
    SystemController &operator=(const SystemController &) = delete;
};

} // namespace Presenter::System

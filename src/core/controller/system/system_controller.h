// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "system/create_new_atelier_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include "system/load_atelier_dto.h"

#include "system/get_current_time_reply_dto.h"
#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::System;

namespace Skribisto::Controller::System
{

class SKRIBISTO_CONTROLLER_EXPORT SystemController : public QObject
{
    Q_OBJECT
  public:
    explicit SystemController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                              QSharedPointer<EventDispatcher> eventDispatcher);

    static SystemController *instance();

    Q_INVOKABLE QCoro::Task<GetCurrentTimeReplyDTO> getCurrentTime() const;

  public slots:

    QCoro::Task<> createNewAtelier(CreateNewAtelierDTO dto);

    QCoro::Task<> loadAtelier(LoadAtelierDTO dto);

    QCoro::Task<> saveAtelier();

    QCoro::Task<> closeAtelier();

  private:
    static QPointer<SystemController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    SystemController() = delete;
    SystemController(const SystemController &) = delete;
    SystemController &operator=(const SystemController &) = delete;
};

} // namespace Skribisto::Controller::System
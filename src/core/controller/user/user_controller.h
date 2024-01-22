// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include "event_dispatcher.h"
#include "user/update_user_dto.h"
#include "user/user_dto.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::User;

namespace Skribisto::Controller::User
{

class SKRIBISTO_CONTROLLER_EXPORT UserController : public QObject
{
    Q_OBJECT
  public:
    explicit UserController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                            QSharedPointer<EventDispatcher> eventDispatcher);

    static UserController *instance();

    Q_INVOKABLE QCoro::Task<UserDTO> get(int id) const;

    Q_INVOKABLE static Contracts::DTO::User::UpdateUserDTO getUpdateDTO();

  public slots:

    QCoro::Task<UserDTO> update(const UpdateUserDTO &dto);

  private:
    static QPointer<UserController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    UserController() = delete;
    UserController(const UserController &) = delete;
    UserController &operator=(const UserController &) = delete;
};

} // namespace Skribisto::Controller::User
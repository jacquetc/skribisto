// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_user_export.h"
#include "user/commands/update_user_command.h"
#include "user/user_dto.h"

#include "repository/interface_user_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::User;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::User::Commands;

namespace Skribisto::Application::Features::User::Commands
{
class SKRIBISTO_APPLICATION_USER_EXPORT UpdateUserCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateUserCommandHandler(InterfaceUserRepository *repository);
    Result<UserDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateUserCommand &request);
    Result<UserDTO> restore();

  signals:
    void userUpdated(Skribisto::Contracts::DTO::User::UserDTO userDto);
    void userDetailsUpdated(int id);

  private:
    InterfaceUserRepository *m_repository;
    Result<UserDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateUserCommand &request);
    Result<UserDTO> restoreImpl();
    Result<UserDTO> saveOldState();
    Result<UserDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::User::Commands
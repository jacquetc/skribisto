// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_user_export.h"
#include "repository/interface_user_repository.h"
#include "user/commands/create_user_command.h"
#include "user/user_dto.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::User;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::User::Commands;

namespace Skribisto::Application::Features::User::Commands
{
class SKRIBISTO_APPLICATION_USER_EXPORT CreateUserCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateUserCommandHandler(InterfaceUserRepository *repository);

    Result<UserDTO> handle(QPromise<Result<void>> &progressPromise, const CreateUserCommand &request);
    Result<UserDTO> restore();

  signals:
    void userCreated(Skribisto::Contracts::DTO::User::UserDTO userDto);
    void userRemoved(int id);

  private:
    InterfaceUserRepository *m_repository;
    Result<UserDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateUserCommand &request);
    Result<UserDTO> restoreImpl();
    Result<Skribisto::Domain::User> m_newEntity;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Skribisto::Application::Features::User::Commands
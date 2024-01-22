// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_user_export.h"
#include "user/queries/get_user_query.h"
#include "user/user_with_details_dto.h"

#include "repository/interface_user_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::User;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::User::Queries;

namespace Skribisto::Application::Features::User::Queries
{
class SKRIBISTO_APPLICATION_USER_EXPORT GetUserWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetUserWithDetailsQueryHandler(InterfaceUserRepository *repository);
    Result<UserWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetUserQuery &query);

  private:
    InterfaceUserRepository *m_repository;
    Result<UserWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetUserQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::User::Queries
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_user_with_details_query_handler.h"
#include "repository/interface_user_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::User::Queries;

GetUserWithDetailsQueryHandler::GetUserWithDetailsQueryHandler(InterfaceUserRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<UserWithDetailsDTO> GetUserWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                  const GetUserQuery &query)
{
    Result<UserWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<UserWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetUserQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<UserWithDetailsDTO> GetUserWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                      const GetUserQuery &query)
{
    qDebug() << "GetUserWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto userResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(UserWithDetailsDTO, userResult)

    Skribisto::Domain::User user = userResult.value();

    // map
    auto userWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserWithDetailsDTO>(user);

    qDebug() << "GetUserWithDetailsQueryHandler::handleImpl done";

    return Result<UserWithDetailsDTO>(userWithDetailsDTO);
}

bool GetUserWithDetailsQueryHandler::s_mappingRegistered = false;

void GetUserWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::User,
                                                           Contracts::DTO::User::UserWithDetailsDTO>();
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_user_query_handler.h"
#include "repository/interface_user_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::User::Queries;

GetUserQueryHandler::GetUserQueryHandler(InterfaceUserRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<UserDTO> GetUserQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetUserQuery &query)
{
    Result<UserDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<UserDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetUserQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<UserDTO> GetUserQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetUserQuery &query)
{
    qDebug() << "GetUserQueryHandler::handleImpl called with id" << query.id;

    // do
    auto userResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(UserDTO, userResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::User, UserDTO>(userResult.value());

    qDebug() << "GetUserQueryHandler::handleImpl done";

    return Result<UserDTO>(dto);
}

bool GetUserQueryHandler::s_mappingRegistered = false;

void GetUserQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::User, Contracts::DTO::User::UserDTO>(
        true, true);
}
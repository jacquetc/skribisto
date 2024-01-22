// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_workspace_query_handler.h"
#include "repository/interface_workspace_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Workspace::Queries;

GetWorkspaceQueryHandler::GetWorkspaceQueryHandler(InterfaceWorkspaceRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<WorkspaceDTO> GetWorkspaceQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                      const GetWorkspaceQuery &query)
{
    Result<WorkspaceDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<WorkspaceDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetWorkspaceQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<WorkspaceDTO> GetWorkspaceQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                          const GetWorkspaceQuery &query)
{
    qDebug() << "GetWorkspaceQueryHandler::handleImpl called with id" << query.id;

    // do
    auto workspaceResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(WorkspaceDTO, workspaceResult)

    // map
    auto dto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceDTO>(workspaceResult.value());

    qDebug() << "GetWorkspaceQueryHandler::handleImpl done";

    return Result<WorkspaceDTO>(dto);
}

bool GetWorkspaceQueryHandler::s_mappingRegistered = false;

void GetWorkspaceQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Workspace,
                                                           Contracts::DTO::Workspace::WorkspaceDTO>(true, true);
}
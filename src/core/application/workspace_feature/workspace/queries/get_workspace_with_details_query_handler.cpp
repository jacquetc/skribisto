// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_workspace_with_details_query_handler.h"
#include "repository/interface_workspace_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Workspace::Queries;

GetWorkspaceWithDetailsQueryHandler::GetWorkspaceWithDetailsQueryHandler(InterfaceWorkspaceRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<WorkspaceWithDetailsDTO> GetWorkspaceWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                            const GetWorkspaceQuery &query)
{
    Result<WorkspaceWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<WorkspaceWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetWorkspaceQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<WorkspaceWithDetailsDTO> GetWorkspaceWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                                const GetWorkspaceQuery &query)
{
    qDebug() << "GetWorkspaceWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto workspaceResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(WorkspaceWithDetailsDTO, workspaceResult)

    Skribisto::Domain::Workspace workspace = workspaceResult.value();

    // map
    auto workspaceWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceWithDetailsDTO>(workspace);

    qDebug() << "GetWorkspaceWithDetailsQueryHandler::handleImpl done";

    return Result<WorkspaceWithDetailsDTO>(workspaceWithDetailsDTO);
}

bool GetWorkspaceWithDetailsQueryHandler::s_mappingRegistered = false;

void GetWorkspaceWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Workspace,
                                                           Contracts::DTO::Workspace::WorkspaceWithDetailsDTO>();
}
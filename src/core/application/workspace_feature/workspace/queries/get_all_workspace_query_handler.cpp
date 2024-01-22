// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_workspace_query_handler.h"
#include "repository/interface_workspace_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Workspace::Queries;

GetAllWorkspaceQueryHandler::GetAllWorkspaceQueryHandler(InterfaceWorkspaceRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<WorkspaceDTO>> GetAllWorkspaceQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllWorkspaceQueryHandler::handle called";

    Result<QList<WorkspaceDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<WorkspaceDTO>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllWorkspaceQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<QList<WorkspaceDTO>> GetAllWorkspaceQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllWorkspaceQueryHandler::handleImpl called";

    // do
    auto workspaceResult = m_repository->getAll();

    QLN_RETURN_IF_ERROR(QList<WorkspaceDTO>, workspaceResult)

    // map
    QList<WorkspaceDTO> dtoList;

    for (const Skribisto::Domain::Workspace &workspace : workspaceResult.value())
    {
        auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceDTO>(workspace);
        dtoList.append(dto);
    }

    qDebug() << "GetAllWorkspaceQueryHandler::handleImpl done";

    return Result<QList<WorkspaceDTO>>(dtoList);
}

bool GetAllWorkspaceQueryHandler::s_mappingRegistered = false;

void GetAllWorkspaceQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Workspace,
                                                           Contracts::DTO::Workspace::WorkspaceDTO>(true, true);
}
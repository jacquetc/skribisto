// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_workspace_command_handler.h"
#include "repository/interface_workspace_repository.h"
#include "workspace/validators/update_workspace_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Workspace;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Workspace::Commands;
using namespace Skribisto::Contracts::CQRS::Workspace::Validators;
using namespace Skribisto::Application::Features::Workspace::Commands;

UpdateWorkspaceCommandHandler::UpdateWorkspaceCommandHandler(InterfaceWorkspaceRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<WorkspaceDTO> UpdateWorkspaceCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                           const UpdateWorkspaceCommand &request)
{
    Result<WorkspaceDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<WorkspaceDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateWorkspaceCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<WorkspaceDTO> UpdateWorkspaceCommandHandler::restore()
{
    Result<WorkspaceDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<WorkspaceDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateWorkspaceCommand restore:" << ex.what();
    }
    return result;
}

Result<WorkspaceDTO> UpdateWorkspaceCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                               const UpdateWorkspaceCommand &request)
{
    qDebug() << "UpdateWorkspaceCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateWorkspaceCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(WorkspaceDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::Workspace> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(WorkspaceDTO, currentResult)

        // map
        m_undoState =
            Result<WorkspaceDTO>(Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceDTO>(
                currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<WorkspaceDTO, UpdateWorkspaceDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto workspace =
        Qleany::Tools::AutoMapper::AutoMapper::map<UpdateWorkspaceDTO, Skribisto::Domain::Workspace>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        workspace.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto workspaceResult = m_repository->update(std::move(workspace));

    if (workspaceResult.hasError())
    {
        return Result<WorkspaceDTO>(workspaceResult.error());
    }

    // map
    auto workspaceDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceDTO>(workspaceResult.value());

    emit workspaceUpdated(workspaceDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit workspaceDetailsUpdated(workspaceDto.id());
    }

    qDebug() << "UpdateWorkspaceCommandHandler::handleImpl done";

    return Result<WorkspaceDTO>(workspaceDto);
}

Result<WorkspaceDTO> UpdateWorkspaceCommandHandler::restoreImpl()
{
    qDebug() << "UpdateWorkspaceCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto workspace =
        Qleany::Tools::AutoMapper::AutoMapper::map<WorkspaceDTO, Skribisto::Domain::Workspace>(m_undoState.value());

    // do
    auto workspaceResult = m_repository->update(std::move(workspace));

    QLN_RETURN_IF_ERROR(WorkspaceDTO, workspaceResult)

    // map
    auto workspaceDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Workspace, WorkspaceDTO>(workspaceResult.value());

    emit workspaceUpdated(workspaceDto);

    qDebug() << "UpdateWorkspaceCommandHandler::restoreImpl done";

    return Result<WorkspaceDTO>(workspaceDto);
}

bool UpdateWorkspaceCommandHandler::s_mappingRegistered = false;

void UpdateWorkspaceCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Workspace,
                                                           Contracts::DTO::Workspace::WorkspaceDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Workspace::UpdateWorkspaceDTO,
                                                           Contracts::DTO::Workspace::WorkspaceDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Workspace::UpdateWorkspaceDTO,
                                                           Skribisto::Domain::Workspace>();
}
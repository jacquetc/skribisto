// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_workspace_export.h"
#include "workspace/commands/update_workspace_command.h"
#include "workspace/workspace_dto.h"

#include "repository/interface_workspace_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Workspace;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Workspace::Commands;

namespace Skribisto::Application::Features::Workspace::Commands
{
class SKRIBISTO_APPLICATION_WORKSPACE_EXPORT UpdateWorkspaceCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateWorkspaceCommandHandler(InterfaceWorkspaceRepository *repository);
    Result<WorkspaceDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateWorkspaceCommand &request);
    Result<WorkspaceDTO> restore();

  signals:
    void workspaceUpdated(Skribisto::Contracts::DTO::Workspace::WorkspaceDTO workspaceDto);
    void workspaceDetailsUpdated(int id);

  private:
    InterfaceWorkspaceRepository *m_repository;
    Result<WorkspaceDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateWorkspaceCommand &request);
    Result<WorkspaceDTO> restoreImpl();
    Result<WorkspaceDTO> saveOldState();
    Result<WorkspaceDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Workspace::Commands
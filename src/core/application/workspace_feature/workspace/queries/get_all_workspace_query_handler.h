// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_workspace_export.h"
#include "workspace/workspace_dto.h"

#include "repository/interface_workspace_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Workspace;
using namespace Skribisto::Contracts::Repository;

namespace Skribisto::Application::Features::Workspace::Queries
{
class SKRIBISTO_APPLICATION_WORKSPACE_EXPORT GetAllWorkspaceQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllWorkspaceQueryHandler(InterfaceWorkspaceRepository *repository);
    Result<QList<WorkspaceDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceWorkspaceRepository *m_repository;
    Result<QList<WorkspaceDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Workspace::Queries
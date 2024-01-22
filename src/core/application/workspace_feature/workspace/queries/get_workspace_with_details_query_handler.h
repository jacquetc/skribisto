// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_workspace_export.h"
#include "workspace/queries/get_workspace_query.h"
#include "workspace/workspace_with_details_dto.h"

#include "repository/interface_workspace_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Workspace;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Workspace::Queries;

namespace Skribisto::Application::Features::Workspace::Queries
{
class SKRIBISTO_APPLICATION_WORKSPACE_EXPORT GetWorkspaceWithDetailsQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetWorkspaceWithDetailsQueryHandler(InterfaceWorkspaceRepository *repository);
    Result<WorkspaceWithDetailsDTO> handle(QPromise<Result<void>> &progressPromise, const GetWorkspaceQuery &query);

  private:
    InterfaceWorkspaceRepository *m_repository;
    Result<WorkspaceWithDetailsDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetWorkspaceQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Workspace::Queries
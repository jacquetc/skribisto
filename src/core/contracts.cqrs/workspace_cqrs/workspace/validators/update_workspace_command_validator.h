// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "workspace/update_workspace_dto.h"


#include "repository/interface_workspace_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::Workspace;

namespace Skribisto::Contracts::CQRS::Workspace::Validators
{
class UpdateWorkspaceCommandValidator
{
  public:
    UpdateWorkspaceCommandValidator(InterfaceWorkspaceRepository *workspaceRepository)
        :  m_workspaceRepository(workspaceRepository)
    {
    }

    Result<void> validate(const UpdateWorkspaceDTO &dto) const

    {




        Result<bool> existsResult = m_workspaceRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceWorkspaceRepository *m_workspaceRepository;

};
} // namespace Skribisto::Contracts::CQRS::Workspace::Validators
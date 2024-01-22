// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "workspace/update_workspace_dto.h"


namespace Skribisto::Contracts::CQRS::Workspace::Commands
{
class UpdateWorkspaceCommand
{
  public:
    UpdateWorkspaceCommand()
    {
    }



    Skribisto::Contracts::DTO::Workspace::UpdateWorkspaceDTO req;


};
} // namespace Skribisto::Contracts::CQRS::Workspace::Commands
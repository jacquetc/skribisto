// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "user/update_user_dto.h"


namespace Skribisto::Contracts::CQRS::User::Commands
{
class UpdateUserCommand
{
  public:
    UpdateUserCommand()
    {
    }



    Skribisto::Contracts::DTO::User::UpdateUserDTO req;


};
} // namespace Skribisto::Contracts::CQRS::User::Commands
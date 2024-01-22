// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "user/create_user_dto.h"


#include "repository/interface_user_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::User;

namespace Skribisto::Contracts::CQRS::User::Validators
{
class CreateUserCommandValidator
{
  public:
    CreateUserCommandValidator(InterfaceUserRepository *userRepository)
        :  m_userRepository(userRepository)
    {
    }

    Result<void> validate(const CreateUserDTO &dto) const

    {





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceUserRepository *m_userRepository;

};
} // namespace Skribisto::Contracts::CQRS::User::Validators
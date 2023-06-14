#pragma once

#include "system/save_system_as_dto.h"

namespace Contracts::CQRS::System::Commands
{
class SaveSystemAsCommand
{
  public:
    SaveSystemAsCommand()
    {
    }

    Contracts::DTO::System::SaveSystemAsDTO req;
};
} // namespace Contracts::CQRS::System::Commands
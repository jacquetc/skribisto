#pragma once

#include "contracts_global.h"
#include "save_system_as_dto.h"

namespace Contracts::CQRS::System::Commands
{
class SKR_CONTRACTS_EXPORT SaveSystemAsCommand
{
  public:
    SaveSystemAsCommand()
    {
    }

    Contracts::DTO::System::SaveSystemAsDTO req;
};
} // namespace Contracts::CQRS::System::Commands

#pragma once

#include "contracts_global.h"
#include "dto/system/load_system_dto.h"

namespace Contracts::CQRS::System::Commands
{
class SKR_CONTRACTS_EXPORT LoadSystemCommand
{
  public:
    LoadSystemCommand()
    {
    }

    Contracts::DTO::System::LoadSystemDTO req;
};
} // namespace Contracts::CQRS::System::Commands

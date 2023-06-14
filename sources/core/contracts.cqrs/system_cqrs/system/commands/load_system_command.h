#pragma once

#include "system/load_system_dto.h"

namespace Contracts::CQRS::System::Commands
{
class LoadSystemCommand
{
  public:
    LoadSystemCommand()
    {
    }

    Contracts::DTO::System::LoadSystemDTO req;
};
} // namespace Contracts::CQRS::System::Commands
#pragma once
#include "contracts_global.h"
#include "dto/system/load_system_dto.h"
#include "result.h"

using namespace Contracts::DTO::System;

namespace Contracts::Infrastructure::Skrib
{
class SKR_CONTRACTS_EXPORT InterfaceSkribLoader
{
  public:
    virtual ~InterfaceSkribLoader()
    {
    }

    virtual Result<void> load(const LoadSystemDTO &dto) = 0;
};
} // namespace Contracts::Infrastructure::Skrib

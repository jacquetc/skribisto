#pragma once
#include "contracts_global.h"
#include "result.h"
#include "system/load_system_dto.h"
#include <QPromise>

using namespace Contracts::DTO::System;

namespace Contracts::Infrastructure::Skrib
{
class SKR_CONTRACTS_EXPORT InterfaceSkribLoader
{
  public:
    virtual ~InterfaceSkribLoader()
    {
    }

    virtual Result<void> load(QPromise<Result<void>> &progressPromise, const LoadSystemDTO &dto) = 0;
};
} // namespace Contracts::Infrastructure::Skrib

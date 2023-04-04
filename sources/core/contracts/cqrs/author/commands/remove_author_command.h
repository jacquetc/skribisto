#pragma once

#include "contracts_global.h"
#include <QUuid>

namespace Contracts::CQRS::Author::Commands
{
class SKR_CONTRACTS_EXPORT RemoveAuthorCommand
{
  public:
    RemoveAuthorCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Author::Commands

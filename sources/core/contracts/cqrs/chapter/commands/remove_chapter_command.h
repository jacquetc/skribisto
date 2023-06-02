#pragma once

#include "contracts_global.h"
#include <QUuid>

namespace Contracts::CQRS::Chapter::Commands
{
class SKR_CONTRACTS_EXPORT RemoveChapterCommand
{
  public:
    RemoveChapterCommand()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Chapter::Commands

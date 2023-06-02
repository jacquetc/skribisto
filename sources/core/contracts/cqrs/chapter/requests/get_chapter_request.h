#pragma once

#include "contracts_global.h"
#include <QUuid>

namespace Contracts::CQRS::Chapter::Requests
{
class SKR_CONTRACTS_EXPORT GetChapterRequest
{
  public:
    GetChapterRequest()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Chapter::Requests

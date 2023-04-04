#pragma once

#include "contracts_global.h"
#include <QUuid>

namespace Contracts::CQRS::Author::Requests
{
class SKR_CONTRACTS_EXPORT GetAuthorRequest
{
  public:
    GetAuthorRequest()
    {
    }

    int id = -1;
};
} // namespace Contracts::CQRS::Author::Requests

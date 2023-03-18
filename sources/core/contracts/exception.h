#pragma once
#include "contracts_global.h"
#include <QException>

namespace Contracts
{
class SKR_CONTRACTS_EXPORT Exception : public QException
{
  public:
    void raise() const override
    {
        throw *this;
    }

    Exception *clone() const override
    {
        return new Exception(*this);
    }
};
} // namespace Contracts

#include "contracts_global.h"
#include "persistence/interface_repository.h"
#include <QSharedPointer>

#pragma once
namespace Contracts::Persistence {
class SKR_CONTRACTS_EXPORT InterfaceRepositoryProvider {
public:

    virtual ~InterfaceRepositoryProvider()
    {}

    virtual void                               registerRepository(const QString                    & name,
                                                                  QSharedPointer<InterfaceRepository>repository) = 0;
    virtual QSharedPointer<InterfaceRepository>repository(const QString& name)                                   = 0;
};
} // namespace Contracts::Persistence

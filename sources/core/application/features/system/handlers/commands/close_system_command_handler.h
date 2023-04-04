#pragma once

#include "application_global.h"
#include "cqrs/system/commands/close_system_command.h"
#include "handler.h"
#include "persistence/interface_repository_provider.h"

#include "result.h"

using namespace Contracts::CQRS::System::Commands;
using namespace Contracts::Persistence;

namespace Application::Features::System::Commands
{
class SKR_APPLICATION_EXPORT CloseSystemCommandHandler : public Handler
{
    Q_OBJECT
  public:
    explicit CloseSystemCommandHandler(InterfaceRepositoryProvider *repositoryProvider);
    Result<void> handle(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);

    Result<void> restore();
  signals:
    void systemClosed();

  private:
    InterfaceRepositoryProvider *m_repositoryProvider;
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);
};
} // namespace Application::Features::System::Commands

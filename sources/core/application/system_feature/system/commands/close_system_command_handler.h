#pragma once

#include "application_system_export.h"
#include "result.h"
#include "system/commands/close_system_command.h"
#include <QPromise>

using namespace Contracts::CQRS::System::Commands;

namespace Application::Features::System::Commands
{
class SKRIBISTO_APPLICATION_SYSTEM_EXPORT CloseSystemCommandHandler : public QObject
{
    Q_OBJECT

  public:
    CloseSystemCommandHandler();

    Result<void> handle(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);
    Result<void> restore()
    {
        Q_UNREACHABLE();
        return Result<void>();
    }

  signals:

    void systemClosed();

  private:
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const CloseSystemCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};
} // namespace Application::Features::System::Commands

#pragma once

#include "application_system_export.h"
#include "result.h"
#include "system/commands/save_system_as_command.h"
#include <QPromise>

using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Commands;

namespace Application::Features::System::Commands
{
class SKRIBISTO_APPLICATION_SYSTEM_EXPORT SaveSystemAsCommandHandler : public QObject
{
    Q_OBJECT

  public:
    SaveSystemAsCommandHandler();

    Result<void> handle(QPromise<Result<void>> &progressPromise, const SaveSystemAsCommand &request);
    Result<void> restore()
    {
        Q_UNREACHABLE();
        return Result<void>();
    }
  signals:

    void systemSaved();

  private:
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const SaveSystemAsCommand &request);
    static bool s_mappingRegistered;
    void registerMappings();
};
} // namespace Application::Features::System::Commands

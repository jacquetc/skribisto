#pragma once

#include "application_system_export.h"
#include "infrastructure/skrib/interface_skrib_loader.h"
#include "system/commands/load_system_command.h"

#include "result.h"

#include <QPromise>

using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Commands;
using namespace Contracts::Infrastructure::Skrib;

namespace Application::Features::System::Commands
{
class SKRIBISTO_APPLICATION_SYSTEM_EXPORT LoadSystemCommandHandler : public QObject
{
    Q_OBJECT

  public:
    LoadSystemCommandHandler(InterfaceSkribLoader *skribLoader);

    Result<void> handle(QPromise<Result<void>> &progressPromise, const LoadSystemCommand &request);
    Result<void> restore()
    {
        Q_UNREACHABLE();
        return Result<void>();
    }
  signals:

    void systemLoaded();

  private:
    InterfaceSkribLoader *m_skribLoader;

    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const LoadSystemCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};
} // namespace Application::Features::System::Commands

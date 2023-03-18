#pragma once

#include "application_global.h"
#include "cqrs/system/commands/load_system_command.h"
#include "handler.h"
#include "infrastructure/skrib/interface_skrib_loader.h"
#include "result.h"

using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Commands;
using namespace Contracts::Infrastructure::Skrib;

namespace Application::Features::System::Commands
{
class SKR_APPLICATION_EXPORT LoadSystemCommandHandler : public Handler
{
    Q_OBJECT
  public:
    LoadSystemCommandHandler(InterfaceSkribLoader *skribLoader);
    Result<void> handle(const LoadSystemCommand &request);

    Result<void> restore();
signals:
    void systemLoaded();

  private:
    InterfaceSkribLoader *m_skribLoader;
    Result<void> handleImpl(const LoadSystemCommand &request);
};
} // namespace Application::Features::System::Commands

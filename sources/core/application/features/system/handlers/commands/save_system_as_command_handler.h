#pragma once

#include "application_global.h"
#include "cqrs/system/commands/save_system_as_command.h"
#include "handler.h"
#include "result.h"

using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Commands;

namespace Application::Features::System::Commands
{
class SKR_APPLICATION_EXPORT SaveSystemAsCommandHandler : public Handler
{
    Q_OBJECT
  public:
    SaveSystemAsCommandHandler();
    Result<void> handle(const SaveSystemAsCommand &request);

  signals:
    void systemSaved();

  private:
};
} // namespace Application::Features::System::Commands

#include "load_system_command_handler.h"
#include "cqrs/system/validators/load_system_command_validator.h"
#include <QDebug>

using namespace Application::Features::System::Commands;
using namespace Contracts::CQRS::Author::Validators;
using namespace Contracts::Infrastructure::Skrib;

LoadSystemCommandHandler::LoadSystemCommandHandler(InterfaceSkribLoader *skribLoader)
    : Handler(), m_skribLoader(skribLoader)
{
}

Result<void> LoadSystemCommandHandler::handle(const LoadSystemCommand &request)
{

    Result<void> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(this, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateAuthorCommand:" << ex.what();
    }

    return result;
}

Result<void> LoadSystemCommandHandler::handleImpl(const LoadSystemCommand &request)
{
    // validate:
    auto validator = LoadSystemCommandValidator();
    Result<void> validatorResult = validator.validate(request.req);
    if (validatorResult.hasError())
    {
        return Result<void>(validatorResult.error());
    }

    // do
    Result<void> loadResult = m_skribLoader->load(request.req);
    if (loadResult.hasError())
    {
        return Result<void>(loadResult.error());
    }

    emit systemLoaded();

    return Result<void>();
}

Result<void> LoadSystemCommandHandler::restore()
{
    Q_UNREACHABLE();
}

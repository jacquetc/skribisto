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

Result<void> LoadSystemCommandHandler::handle(QPromise<Result<void>> &progressPromise, const LoadSystemCommand &request)
{

    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling LoadSystemCommand:" << ex.what();
    }

    return result;
}

Result<void> LoadSystemCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                  const LoadSystemCommand &request)
{
    progressPromise.setProgressRange(0, 100);

    // validate:
    auto validator = LoadSystemCommandValidator();
    Result<void> validatorResult = validator.validate(request.req);
    if (validatorResult.hasError())
    {
        return Result<void>(validatorResult.error());
    }

    // do
    Result<void> loadResult = m_skribLoader->load(progressPromise, request.req);
    if (loadResult.hasError())
    {
        return Result<void>(loadResult.error());
    }

    progressPromise.setProgressValue(100);
    emit systemLoaded();

    return Result<void>();
}

Result<void> LoadSystemCommandHandler::restore()
{
    Q_UNREACHABLE();
}

#include "load_system_command_handler.h"
#include "automapper/automapper.h"
#include "system/validators/load_system_command_validator.h"


using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Validators;
using namespace Application::Features::System::Commands;
using namespace Contracts::Infrastructure::Skrib;

LoadSystemCommandHandler::LoadSystemCommandHandler(InterfaceSkribLoader *skribLoader)
    : m_skribLoader(skribLoader)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void>LoadSystemCommandHandler::handle(QPromise<Result<void> >& progressPromise,
                                             const LoadSystemCommand& request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception& ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling LoadSystemCommand:" << ex.what();
    }
    return result;
}

Result<void>LoadSystemCommandHandler::handleImpl(QPromise<Result<void> >& progressPromise,
                                                 const LoadSystemCommand& request)
{
    progressPromise.setProgressRange(0, 100);

    // validate:
    auto validator               = LoadSystemCommandValidator();
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

bool LoadSystemCommandHandler::s_mappingRegistered = false;

void LoadSystemCommandHandler::registerMappings()
{
}

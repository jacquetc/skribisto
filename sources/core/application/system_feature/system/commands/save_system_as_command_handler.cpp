#include "save_system_as_command_handler.h"
#include "automapper/automapper.h"
#include "system/validators/save_system_as_command_validator.h"

using namespace Contracts::DTO::System;
using namespace Contracts::CQRS::System::Validators;
using namespace Application::Features::System::Commands;

SaveSystemAsCommandHandler::SaveSystemAsCommandHandler()

{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> SaveSystemAsCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const SaveSystemAsCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling SaveSystemAsCommand:" << ex.what();
    }
    return result;
}

Result<void> SaveSystemAsCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const SaveSystemAsCommand &request)
{
    qDebug() << "SaveSystemAsCommandHandler::handleImpl called";

    Q_UNIMPLEMENTED();

    // emit signal
    emit systemSaved();

    // Return
    return Result<void>();
}

bool SaveSystemAsCommandHandler::s_mappingRegistered = false;

void SaveSystemAsCommandHandler::registerMappings()
{
}

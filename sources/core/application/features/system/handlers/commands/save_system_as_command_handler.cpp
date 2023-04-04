#include "save_system_as_command_handler.h"
#include "cqrs/system/validators/save_system_as_command_validator.h"

using namespace Application::Features::System::Commands;
using namespace Contracts::CQRS::Author::Validators;

SaveSystemAsCommandHandler::SaveSystemAsCommandHandler() : Handler()
{
}

Result<void> SaveSystemAsCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const SaveSystemAsCommand &request)
{

    // validate:
    auto validator = SaveSystemAsCommandValidator();
    Result<void> validatorResult = validator.validate(request.req);
    if (validatorResult.hasError())
    {
        return Result<void>(validatorResult.error());
    }

    //    SkribFileContext *context = new SkribFileContext(request.req.fileName());
    //    auto initResult = context->init();
    //    if (initResult.hasError())
    //    {
    //        return Result<void>(initResult.error());
    //    }

    emit systemSaved();

    return Result<void>();
}

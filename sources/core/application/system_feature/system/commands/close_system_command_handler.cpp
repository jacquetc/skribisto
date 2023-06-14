#include "close_system_command_handler.h"
#include <QDebug>


using namespace Application::Features::System::Commands;

CloseSystemCommandHandler::CloseSystemCommandHandler()
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void>CloseSystemCommandHandler::handle(QPromise<Result<void> > & progressPromise,
                                              const CloseSystemCommand& request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception& ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CloseSystemCommand:" << ex.what();
    }
    return result;
}

Result<void>CloseSystemCommandHandler::handleImpl(QPromise<Result<void> > & progressPromise,
                                                  const CloseSystemCommand& request)
{
    Q_UNIMPLEMENTED();

    // emit signal
    emit systemClosed();

    // Return
    return Result<void>();
}

bool CloseSystemCommandHandler::s_mappingRegistered = false;

void CloseSystemCommandHandler::registerMappings()
{
}

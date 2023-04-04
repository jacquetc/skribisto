#include "close_system_command_handler.h"
#include <QDebug>

using namespace Application::Features::System::Commands;
using namespace Contracts::Persistence;

CloseSystemCommandHandler::CloseSystemCommandHandler(InterfaceRepositoryProvider *repositoryProvider)
    : Handler{}, m_repositoryProvider(repositoryProvider)
{
}

Result<void> CloseSystemCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                               const CloseSystemCommand &request)
{

    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CloseSystemCommand:" << ex.what();
    }

    return result;
}

Result<void> CloseSystemCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const CloseSystemCommand &request)
{

    // do

    m_repositoryProvider->repository(InterfaceRepositoryProvider::Author).clear();
    m_repositoryProvider->repository(InterfaceRepositoryProvider::Atelier).clear();
    m_repositoryProvider->repository(InterfaceRepositoryProvider::Book).clear();

    emit systemClosed();

    return Result<void>();
}

Result<void> CloseSystemCommandHandler::restore()
{
    Q_UNREACHABLE();
}

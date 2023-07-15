#include "remove_author_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;
using namespace Application::Features::Author::Commands;

RemoveAuthorCommandHandler::RemoveAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveAuthorCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                               const RemoveAuthorCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveAuthorCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveAuthorCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveAuthorCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveAuthorCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const RemoveAuthorCommand &request)
{
    Result<Domain::Author> authorResult = m_repository->get(request.id);

    if (Q_UNLIKELY(authorResult.hasError()))
    {
        qDebug() << "Error getting author from repository:" << authorResult.error().message();
        return Result<int>(authorResult.error());
    }

    // save old entity
    m_oldState = authorResult.value();

    int authorId;

    auto deleteResult = m_repository->remove(authorId);

    if (Q_UNLIKELY(!deleteResult.hasError()))
    {
        qDebug() << "Error deleting author from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit authorRemoved(deleteResult.value());

    qDebug() << "Author removed:" << authorId;

    return Result<int>(authorId);
}

Result<int> RemoveAuthorCommandHandler::restoreImpl()
{

    // Add the author to the repository
    auto authorResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(authorResult.hasError()))
    {
        return Result<int>(authorResult.error());
    }

    auto authorDTO = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());

    emit authorCreated(authorDTO);
    qDebug() << "Author added:" << authorDTO.id();

    // Return the UUID of the newly created author as a Result object
    return Result<int>(0);
}

bool RemoveAuthorCommandHandler::s_mappingRegistered = false;

void RemoveAuthorCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
}
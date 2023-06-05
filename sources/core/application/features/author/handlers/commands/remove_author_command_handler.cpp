#include "remove_author_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;
using namespace Application::Features::Author::Commands;

RemoveAuthorCommandHandler::RemoveAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : Handler(), m_repository(repository)
{
}

Result<AuthorDTO> RemoveAuthorCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const RemoveAuthorCommand &request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveAuthorCommand:" << ex.what();
    }

    return result;
}

Result<AuthorDTO> RemoveAuthorCommandHandler::restore()
{
    Result<AuthorDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveAuthorCommand restore:" << ex.what();
    }

    return result;
}

Result<AuthorDTO> RemoveAuthorCommandHandler::handleImpl(const RemoveAuthorCommand &request)
{
    Result<Domain::Author> authorResult = m_repository->get(request.id);
    if (authorResult.hasError())
    {
        qDebug() << "Error getting author from repository:" << authorResult.error().message();
        return Result<AuthorDTO>(authorResult.error());
    }

    auto deleteResult = m_repository->remove(std::move(authorResult.value()));
    if (deleteResult.hasError())
    {
        qDebug() << "Error deleting author from repository:" << deleteResult.error().message();
        return Result<AuthorDTO>(deleteResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<AuthorDTO, Domain::Author>(deleteResult.value());

    // save
    m_oldState = Result<AuthorDTO>(dto);

    emit authorRemoved(dto);

    qDebug() << "Author removed:" << dto.uuid();

    return Result<AuthorDTO>(dto);
}

Result<AuthorDTO> RemoveAuthorCommandHandler::restoreImpl()
{

    // Map the create author command to a domain author object
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(m_oldState.value());

    // Add the author to the repository
    auto authorResult = m_repository->add(std::move(author));
    if (authorResult.hasError())
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    auto authorDTO = AutoMapper::AutoMapper::map<AuthorDTO>(authorResult.value());

    emit authorCreated(authorDTO);
    qDebug() << "Author added:" << authorDTO.uuid();

    // Return the UUID of the newly created author as a Result object
    return Result<AuthorDTO>(authorDTO);
}

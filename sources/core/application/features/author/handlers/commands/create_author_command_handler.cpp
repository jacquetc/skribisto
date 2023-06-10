#include "create_author_command_handler.h"
#include "automapper/automapper.h"
#include "cqrs/author/validators/create_author_command_validator.h"
#include "persistence/interface_author_repository.h"

using namespace Domain;
using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Validators;
using namespace Application::Features::Author::Commands;

CreateAuthorCommandHandler::CreateAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository>repository)
    : Handler(), m_repository(repository)
{}

Result<AuthorDTO>CreateAuthorCommandHandler::handle(QPromise<Result<void> >  & progressPromise,
                                                    const CreateAuthorCommand& request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception& ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateAuthorCommand:" << ex.what();
    }
    return result;
}

Result<AuthorDTO>CreateAuthorCommandHandler::restore()
{
    Result<AuthorDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception& ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateAuthorCommand restore:" << ex.what();
    }
    return result;
}

Result<AuthorDTO>CreateAuthorCommandHandler::handleImpl(const CreateAuthorCommand& request)
{
    qDebug() << "CreateAuthorCommandHandler::handleImpl called";
    Domain::Author author;

    if (m_oldState.isEmpty())
    {
        // Validate the create author command using the validator
        auto validator               = CreateAuthorCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (validatorResult.hasError())
        {
            return Result<AuthorDTO>(validatorResult.error());
        }

        // Map the create author command to a domain author object and generate
        // a UUID
        Domain::Author author = AutoMapper::AutoMapper::map<CreateAuthorDTO, Domain::Author>(
            request.req);

        // allow for forcing the uuid
        if (author.uuid().isNull())
        {
            author.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        author.setCreationDate(QDateTime::currentDateTime());
        author.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        // Map the create author command to a domain author object and generate
        // a UUID
        author = AutoMapper::AutoMapper::map<AuthorDTO, Domain::Author>(m_oldState.value());
    }

    // Add the author to the repository

    m_repository->beginChanges();
    auto authorResult = m_repository->add(std::move(author));

    if (authorResult.hasError())
    {
        m_repository->cancelChanges();
        return Result<AuthorDTO>(authorResult.error());
    }
    m_repository->saveChanges();

    auto authorDTO = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());

    m_oldState = Result<AuthorDTO>(authorDTO);

    emit authorCreated(authorDTO);

    qDebug() << "Author added:" << authorDTO.uuid();

    // Return the UUID of the newly created author as a Result object
    return Result<AuthorDTO>(authorDTO);
}

Result<AuthorDTO>CreateAuthorCommandHandler::restoreImpl()
{
    Result<Domain::Author> authorResult = m_repository->get(m_oldState.value().uuid());

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
    auto authorDTO = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(deleteResult.value());

    emit authorRemoved(authorDTO);

    qDebug() << "Author removed:" << deleteResult.value().uuid();

    return Result<AuthorDTO>(authorDTO);
}

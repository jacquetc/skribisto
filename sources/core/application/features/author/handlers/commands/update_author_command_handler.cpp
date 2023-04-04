#include "update_author_command_handler.h"
#include "automapper/automapper.h"
#include "cqrs/author/validators/update_author_command_validator.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;
using namespace Contracts::CQRS::Author::Validators;
using namespace Application::Features::Author::Commands;

UpdateAuthorCommandHandler::UpdateAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : Handler(), m_repository(repository)
{
}
Result<AuthorDTO> UpdateAuthorCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const UpdateAuthorCommand &request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(this, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateAuthorCommand:" << ex.what();
    }

    return result;
}

Result<AuthorDTO> UpdateAuthorCommandHandler::restore()
{
    Result<AuthorDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(this, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateAuthorCommand restore:" << ex.what();
    }

    return result;
}

Result<AuthorDTO> UpdateAuthorCommandHandler::handleImpl(const UpdateAuthorCommand &request)
{
    qDebug() << "UpdateAuthorCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateAuthorCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);
    if (validatorResult.hasError())
    {
        return Result<AuthorDTO>(validatorResult.error());
    }

    // map
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(request.req);

    // set update timestamp only on first pass
    if (m_oldState.isEmpty())
    {
        author.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_oldState.isEmpty())
    {
        Result<Domain::Author> saveResult = m_repository->get(request.req.id());
        if (saveResult.hasError())
        {
            qDebug() << "Error getting author from repository:" << saveResult.error().message();
            return Result<AuthorDTO>(saveResult.error());
        }

        // map
        m_oldState = Result<AuthorDTO>(AutoMapper::AutoMapper::map<AuthorDTO>(saveResult.value()));
    }
    // do
    auto authorResult = m_repository->update(std::move(author));
    if (authorResult.hasError())
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto authorDto = AutoMapper::AutoMapper::map<AuthorDTO>(authorResult.value());

    emit authorUpdated(authorDto);

    qDebug() << "UpdateAuthorCommandHandler::handleImpl done";

    return Result<AuthorDTO>(authorDto);
}

Result<AuthorDTO> UpdateAuthorCommandHandler::restoreImpl()
{
    qDebug() << "UpdateAuthorCommandHandler::restoreImpl called with id" << m_oldState.value().uuid();
    // map
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(m_oldState.value());

    // do
    auto authorResult = m_repository->update(std::move(author));
    if (authorResult.hasError())
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto authorDto = AutoMapper::AutoMapper::map<AuthorDTO>(authorResult.value());

    emit authorUpdated(authorDto);

    qDebug() << "UpdateAuthorCommandHandler::restoreImpl done";

    return Result<AuthorDTO>(authorDto);
}

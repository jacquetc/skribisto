#include "update_author_command_handler.h"
#include "author/validators/update_author_command_validator.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;
using namespace Contracts::CQRS::Author::Validators;
using namespace Application::Features::Author::Commands;

UpdateAuthorCommandHandler::UpdateAuthorCommandHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<AuthorDTO> UpdateAuthorCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const UpdateAuthorCommand &request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
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
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateAuthorCommand restore:" << ex.what();
    }
    return result;
}

Result<AuthorDTO> UpdateAuthorCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const UpdateAuthorCommand &request)
{
    qDebug() << "UpdateAuthorCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateAuthorCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<AuthorDTO>(validatorResult.error());
    }

    // map
    auto author = AutoMapper::AutoMapper::map<UpdateAuthorDTO, Domain::Author>(request.req);

    // set update timestamp only on first pass
    if (m_newState.isEmpty())
    {
        author.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_newState.isEmpty())
    {
        Result<Domain::Author> saveResult = m_repository->get(request.req.id());

        if (Q_UNLIKELY(saveResult.hasError()))
        {
            qDebug() << "Error getting author from repository:" << saveResult.error().message();
            return Result<AuthorDTO>(saveResult.error());
        }

        // map
        m_newState = Result<AuthorDTO>(AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(saveResult.value()));
    }

    // do
    auto authorResult = m_repository->update(std::move(author));

    if (authorResult.hasError())
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto authorDto = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());

    emit authorUpdated(authorDto);

    qDebug() << "UpdateAuthorCommandHandler::handleImpl done";

    return Result<AuthorDTO>(authorDto);
}

Result<AuthorDTO> UpdateAuthorCommandHandler::restoreImpl()
{
    qDebug() << "UpdateAuthorCommandHandler::restoreImpl called with id" << m_newState.value().uuid();

    // map
    auto author = AutoMapper::AutoMapper::map<AuthorDTO, Domain::Author>(m_newState.value());

    // do
    auto authorResult = m_repository->update(std::move(author));

    if (Q_UNLIKELY(authorResult.hasError()))
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto authorDto = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());

    emit authorUpdated(authorDto);

    qDebug() << "UpdateAuthorCommandHandler::restoreImpl done";

    return Result<AuthorDTO>(authorDto);
}

bool UpdateAuthorCommandHandler::s_mappingRegistered = false;

void UpdateAuthorCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Author::UpdateAuthorDTO, Domain::Author>();
}
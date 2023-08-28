#include "create_author_command_handler.h"
#include "author/validators/create_author_command_validator.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Validators;
using namespace Application::Features::Author::Commands;

CreateAuthorCommandHandler::CreateAuthorCommandHandler(InterfaceAuthorRepository *repository,
                                                       InterfaceBookRepository *ownerRepository)
    : m_repository(repository), m_ownerRepository(ownerRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<AuthorDTO> CreateAuthorCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                     const CreateAuthorCommand &request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateAuthorCommand:" << ex.what();
    }
    return result;
}

Result<AuthorDTO> CreateAuthorCommandHandler::restore()
{
    Result<AuthorDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateAuthorCommand restore:" << ex.what();
    }
    return result;
}

Result<AuthorDTO> CreateAuthorCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                         const CreateAuthorCommand &request)
{
    qDebug() << "CreateAuthorCommandHandler::handleImpl called";
    Domain::Author author;
    Domain::Book ownerEntity;

    if (m_newEntity.isEmpty())
    {
        // Validate the create Author command using the validator
        auto validator = CreateAuthorCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<AuthorDTO>(validatorResult.error());
        }

        CreateAuthorDTO createDTO = request.req;

        // Map the create Author command to a domain Author object and
        // generate a UUID
        author = AutoMapper::AutoMapper::map<CreateAuthorDTO, Domain::Author>(createDTO);

        // allow for forcing the uuid
        if (author.uuid().isNull())
        {
            author.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        author.setCreationDate(QDateTime::currentDateTime());
        author.setUpdateDate(QDateTime::currentDateTime());

        // Get the owner entity
        int ownerId = createDTO.bookId();

        auto originalOwnerEntityResult = m_ownerRepository->get(ownerId);
        if (Q_UNLIKELY(originalOwnerEntityResult.hasError()))
        {
            return Result<AuthorDTO>(originalOwnerEntityResult.error());
        }
        auto originalOwnerEntity = originalOwnerEntityResult.value();
        originalOwnerEntity.setAuthorLoader(m_ownerRepository->fetchAuthorLoader());

        // force the fetching before saving the owner entity
        auto author = originalOwnerEntity.author();

        // save
        m_oldOwnerEntity = originalOwnerEntity;

        // Add the author to the owner entity
        originalOwnerEntity.setAuthor(author);

        m_ownerEntityNewState = originalOwnerEntity;
        ownerEntity = originalOwnerEntity;
    }
    else
    {
        author = m_newEntity.value();
        ownerEntity = m_ownerEntityNewState;
    }

    // Add the author to the repository

    m_repository->beginChanges();
    auto authorResult = m_repository->add(std::move(author));

    if (Q_UNLIKELY(authorResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<AuthorDTO>(authorResult.error());
    }

    // Apply the addition of the new entity to the owner entity

    auto updatedOwnerEntityResult = m_ownerRepository->update(std::move(ownerEntity));

    if (Q_UNLIKELY(updatedOwnerEntityResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<AuthorDTO>(updatedOwnerEntityResult.error());
    }

    m_repository->saveChanges();

    m_newEntity = authorResult;

    auto authorDTO = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());
    emit authorCreated(authorDTO);

    qDebug() << "Author added:" << authorDTO.id();

    // Return the DTO of the newly created Author as a Result object
    return Result<AuthorDTO>(authorDTO);
}

Result<AuthorDTO> CreateAuthorCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newEntity.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Author from repository:" << deleteResult.error().message();
        return Result<AuthorDTO>(deleteResult.error());
    }

    emit authorRemoved(deleteResult.value());

    qDebug() << "Author removed:" << deleteResult.value();

    return Result<AuthorDTO>(AuthorDTO());
}

bool CreateAuthorCommandHandler::s_mappingRegistered = false;

void CreateAuthorCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Author::CreateAuthorDTO, Domain::Author>();
}
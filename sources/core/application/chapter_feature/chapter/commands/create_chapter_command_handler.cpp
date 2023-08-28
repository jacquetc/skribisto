#include "create_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "chapter/validators/create_chapter_command_validator.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Validators;
using namespace Application::Features::Chapter::Commands;

CreateChapterCommandHandler::CreateChapterCommandHandler(InterfaceChapterRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ChapterDTO> CreateChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                       const CreateChapterCommand &request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand:" << ex.what();
    }
    return result;
}

Result<ChapterDTO> CreateChapterCommandHandler::restore()
{
    Result<ChapterDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<ChapterDTO> CreateChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                           const CreateChapterCommand &request)
{
    qDebug() << "CreateChapterCommandHandler::handleImpl called";
    Domain::Chapter chapter;

    if (m_newState.isEmpty())
    {
        // Validate the create Chapter command using the validator
        auto validator = CreateChapterCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<ChapterDTO>(validatorResult.error());
        }

        // Map the create Chapter command to a domain Chapter object and
        // generate a UUID
        chapter = AutoMapper::AutoMapper::map<CreateChapterDTO, Domain::Chapter>(request.req);

        // allow for forcing the uuid
        if (chapter.uuid().isNull())
        {
            chapter.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        chapter.setCreationDate(QDateTime::currentDateTime());
        chapter.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        // Map the create chapter command to a domain chapter object and
        // generate a UUID
        chapter = AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(m_newState.value());
    }

    // Add the chapter to the repository

    m_repository->beginChanges();
    auto chapterResult = m_repository->add(std::move(chapter));

    if (Q_UNLIKELY(chapterResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<ChapterDTO>(chapterResult.error());
    }
    m_repository->saveChanges();

    auto chapterDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    m_newState = Result<ChapterDTO>(chapterDTO);

    emit chapterCreated(chapterDTO);

    qDebug() << "Chapter added:" << chapterDTO.id();

    // Return the DTO of the newly created Chapter as a Result object
    return Result<ChapterDTO>(chapterDTO);
}

Result<ChapterDTO> CreateChapterCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newState.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Chapter from repository:" << deleteResult.error().message();
        return Result<ChapterDTO>(deleteResult.error());
    }

    emit chapterRemoved(deleteResult.value());

    qDebug() << "Chapter removed:" << deleteResult.value();

    return Result<ChapterDTO>(ChapterDTO());
}

bool CreateChapterCommandHandler::s_mappingRegistered = false;

void CreateChapterCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Chapter, Contracts::DTO::Chapter::ChapterDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Chapter::CreateChapterDTO, Domain::Chapter>();
}
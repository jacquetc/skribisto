#include "create_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "cqrs/chapter/validators/create_chapter_command_validator.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Validators;
using namespace Application::Features::Chapter::Commands;

CreateChapterCommandHandler::CreateChapterCommandHandler(QSharedPointer<InterfaceChapterRepository>repository)
    : Handler(), m_repository(repository)
{}

Result<ChapterDTO>CreateChapterCommandHandler::handle(QPromise<Result<void> >   & progressPromise,
                                                      const CreateChapterCommand& request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception& ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand:" << ex.what();
    }
    return result;
}

Result<ChapterDTO>CreateChapterCommandHandler::restore()
{
    Result<ChapterDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception& ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<ChapterDTO>CreateChapterCommandHandler::handleImpl(const CreateChapterCommand& request)
{
    qDebug() << "CreateChapterCommandHandler::handleImpl called";
    Domain::Chapter chapter;

    if (m_oldState.isEmpty())
    {
        // Validate the create chapter command using the validator
        auto validator               = CreateChapterCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (validatorResult.hasError())
        {
            return Result<ChapterDTO>(validatorResult.error());
        }

        // Map the create chapter command to a domain chapter object and
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
        chapter = AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(m_oldState.value());
    }

    // Add the chapter to the repository

    m_repository->beginChanges();
    auto chapterResult = m_repository->add(std::move(chapter));

    if (chapterResult.hasError())
    {
        m_repository->cancelChanges();
        return Result<ChapterDTO>(chapterResult.error());
    }
    m_repository->saveChanges();

    auto chapterDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    m_oldState = Result<ChapterDTO>(chapterDTO);

    emit chapterCreated(chapterDTO);

    qDebug() << "Chapter added:" << chapterDTO.uuid();

    // Return the UUID of the newly created chapter as a Result object
    return Result<ChapterDTO>(chapterDTO);
}

Result<ChapterDTO>CreateChapterCommandHandler::restoreImpl()
{
    Result<Domain::Chapter> chapterResult = m_repository->get(m_oldState.value().uuid());

    if (chapterResult.hasError())
    {
        qDebug() << "Error getting chapter from repository:" << chapterResult.error().message();
        return Result<ChapterDTO>(chapterResult.error());
    }

    auto deleteResult = m_repository->remove(std::move(chapterResult.value()));

    if (deleteResult.hasError())
    {
        qDebug() << "Error deleting chapter from repository:" << deleteResult.error().message();
        return Result<ChapterDTO>(deleteResult.error());
    }
    auto chapterDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(deleteResult.value());

    emit chapterRemoved(chapterDTO);

    qDebug() << "Chapter removed:" << deleteResult.value().uuid();

    return Result<ChapterDTO>(chapterDTO);
}

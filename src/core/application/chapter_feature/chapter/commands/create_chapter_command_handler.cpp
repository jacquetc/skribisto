// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_chapter_command_handler.h"
#include "chapter/validators/create_chapter_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

#include "book.h"

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Validators;
using namespace Skribisto::Application::Features::Chapter::Commands;

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
        result = Result<ChapterDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<ChapterDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<ChapterDTO> CreateChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                           const CreateChapterCommand &request)
{
    qDebug() << "CreateChapterCommandHandler::handleImpl called";
    Skribisto::Domain::Chapter chapter;
    CreateChapterDTO createDTO = request.req;

    QList<Skribisto::Domain::Chapter> ownerEntityChapters;

    // Get the entities from owner
    int ownerId = createDTO.bookId();
    m_ownerId = ownerId;

    if (m_firstPass)
    {
        // Validate the create Chapter command using the validator
        auto validator = CreateChapterCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(ChapterDTO, validatorResult);

        // Map the create Chapter command to a domain Chapter object and
        // generate a UUID
        chapter = Qleany::Tools::AutoMapper::AutoMapper::map<CreateChapterDTO, Skribisto::Domain::Chapter>(createDTO);

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
        chapter = m_newEntity.value();
    }

    // Add the chapter to the repository

    m_repository->beginChanges();
    auto chapterResult = m_repository->add(std::move(chapter));

    QLN_RETURN_IF_ERROR_WITH_ACTION(ChapterDTO, chapterResult, m_repository->cancelChanges();)

    // Get the newly created Chapter entity
    chapter = chapterResult.value();
    // Save the newly created entity
    m_newEntity = chapterResult;

    //  Manage relation to owner

    int position = -1;

    if (m_firstPass)
    {

        auto originalOwnerChaptersResult = m_repository->getEntitiesInRelationOf(Book::schema, ownerId, "chapters");
        if (Q_UNLIKELY(originalOwnerChaptersResult.hasError()))
        {
            return Result<ChapterDTO>(originalOwnerChaptersResult.error());
        }
        auto originalOwnerChapters = originalOwnerChaptersResult.value();

        // save
        m_oldOwnerChapters = originalOwnerChapters;

        // Insert to the right position

        position = createDTO.position();
        if (position == -1)
        {
            position = originalOwnerChapters.count();
        }
        if (position > originalOwnerChapters.count())
        {
            position = originalOwnerChapters.count();
        }
        else if (position < 0)
        {
            position = 0;
        }

        m_position = position;

        originalOwnerChapters.insert(position, chapter);

        m_ownerChaptersNewState = originalOwnerChapters;
        ownerEntityChapters = originalOwnerChapters;
    }
    else
    {
        ownerEntityChapters = m_ownerChaptersNewState;
        position = m_position;
    }

    // Add the chapter to the owner entity
    Result<QList<Skribisto::Domain::Chapter>> updateResult =
        m_repository->updateEntitiesInRelationOf(Book::schema, ownerId, "chapters", ownerEntityChapters);

    QLN_RETURN_IF_ERROR_WITH_ACTION(ChapterDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = chapterResult;

    auto chapterDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterDTO>(chapterResult.value());
    emit chapterCreated(chapterDTO);

    // send an insertion signal
    emit relationWithOwnerInserted(chapter.id(), ownerId, position);

    qDebug() << "Chapter added:" << chapterDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Chapter as a Result object
    return Result<ChapterDTO>(chapterDTO);
}

Result<ChapterDTO> CreateChapterCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(ChapterDTO, deleteResult)

    emit chapterRemoved(deleteResult.value());

    qDebug() << "Chapter removed:" << deleteResult.value();

    emit relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<ChapterDTO>(ChapterDTO());
}

bool CreateChapterCommandHandler::s_mappingRegistered = false;

void CreateChapterCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Chapter,
                                                           Contracts::DTO::Chapter::ChapterDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Chapter::CreateChapterDTO,
                                                           Skribisto::Domain::Chapter>();
}
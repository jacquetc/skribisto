#include "remove_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;
using namespace Application::Features::Chapter::Commands;

RemoveChapterCommandHandler::RemoveChapterCommandHandler(QSharedPointer<InterfaceChapterRepository>repository)
    : Handler(), m_repository(repository)
{}

Result<ChapterDTO>RemoveChapterCommandHandler::handle(QPromise<Result<void> >   & progressPromise,
                                                      const RemoveChapterCommand& request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception& ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand:" << ex.what();
    }
    return result;
}

Result<ChapterDTO>RemoveChapterCommandHandler::restore()
{
    Result<ChapterDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception& ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<ChapterDTO>RemoveChapterCommandHandler::handleImpl(const RemoveChapterCommand& request)
{
    Result<Domain::Chapter> chapterResult = m_repository->get(request.id);

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

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(deleteResult.value());

    // save
    m_oldState = Result<ChapterDTO>(dto);

    emit chapterRemoved(dto);

    qDebug() << "Chapter removed:" << dto.uuid();

    return Result<ChapterDTO>(dto);
}

Result<ChapterDTO>RemoveChapterCommandHandler::restoreImpl()
{
    // Map the create chapter command to a domain chapter object
    auto chapter = AutoMapper::AutoMapper::map<ChapterDTO, Domain::Chapter>(m_oldState.value());

    // Add the chapter to the repository
    auto chapterResult = m_repository->add(std::move(chapter));

    if (chapterResult.hasError())
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    auto chapterDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    emit chapterCreated(chapterDTO);
    qDebug() << "Chapter added:" << chapterDTO.uuid();

    // Return the UUID of the newly created chapter as a Result object
    return Result<ChapterDTO>(chapterDTO);
}

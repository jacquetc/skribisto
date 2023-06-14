#include "remove_chapter_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Contracts::DTO::Chapter;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Chapter::Commands;
using namespace Application::Features::Chapter::Commands;

RemoveChapterCommandHandler::RemoveChapterCommandHandler(QSharedPointer<InterfaceChapterRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveChapterCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const RemoveChapterCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveChapterCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveChapterCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveChapterCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const RemoveChapterCommand &request)
{
    Result<Domain::Chapter> chapterResult = m_repository->get(request.id);

    if (Q_UNLIKELY(!chapterResult.hasError()))
    {
        qDebug() << "Error getting chapter from repository:" << chapterResult.error().message();
        return Result<int>(chapterResult.error());
    }

    // save old entity
    m_oldState = chapterResult.value();

    int chapterId;

    auto deleteResult = m_repository->remove(chapterId);

    if (Q_UNLIKELY(!deleteResult.hasError()))
    {
        qDebug() << "Error deleting chapter from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit chapterRemoved(deleteResult.value());

    qDebug() << "Chapter removed:" << chapterId;

    return Result<int>(chapterId);
}

Result<int> RemoveChapterCommandHandler::restoreImpl()
{

    // Add the chapter to the repository
    auto chapterResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(!chapterResult.hasError()))
    {
        return Result<int>(chapterResult.error());
    }

    auto chapterDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    emit chapterCreated(chapterDTO);
    qDebug() << "Chapter added:" << chapterDTO.id();

    // Return the UUID of the newly created chapter as a Result object
    return Result<int>(0);
}

bool RemoveChapterCommandHandler::s_mappingRegistered = false;

void RemoveChapterCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Chapter, Contracts::DTO::Chapter::ChapterDTO>(true);
}
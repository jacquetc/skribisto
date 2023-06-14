#include "get_all_chapter_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Application::Features::Chapter::Queries;

GetAllChapterQueryHandler::GetAllChapterQueryHandler(QSharedPointer<InterfaceChapterRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<ChapterDTO>> GetAllChapterQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllChapterQueryHandler::handle called";

    Result<QList<ChapterDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<ChapterDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllChapterQuery:" << ex.what();
    }
    return result;
}

Result<QList<ChapterDTO>> GetAllChapterQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllChapterQueryHandler::handleImpl called";

    // do
    auto chapterResult = m_repository->getAll();

    if (Q_UNLIKELY(!chapterResult.isError()))
    {
        return Result<QList<ChapterDTO>>(chapterResult.error());
    }

    // map
    QList<ChapterDTO> dtoList;

    for (const Domain::Chapter &chapter : chapterResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapter);
        dtoList.append(dto);
    }

    qDebug() << "GetAllChapterQueryHandler::handleImpl done";

    return Result<QList<ChapterDTO>>(dtoList);
}

bool GetAllChapterQueryHandler::s_mappingRegistered = false;

void GetAllChapterQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Chapter, Contracts::DTO::Chapter::ChapterDTO>(true);
}
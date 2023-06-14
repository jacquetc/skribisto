#include "get_chapter_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Application::Features::Chapter::Queries;

GetChapterQueryHandler::GetChapterQueryHandler(QSharedPointer<InterfaceChapterRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ChapterDTO> GetChapterQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetChapterQuery &query)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterQuery:" << ex.what();
    }
    return result;
}

Result<ChapterDTO> GetChapterQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                      const GetChapterQuery &query)
{
    qDebug() << "GetChapterQueryHandler::handleImpl called with id" << query.id;

    // do
    auto chapterResult = m_repository->get(query.id);

    if (Q_UNLIKELY(!chapterResult.isError()))
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    qDebug() << "GetChapterQueryHandler::handleImpl done";

    return Result<ChapterDTO>(dto);
}

bool GetChapterQueryHandler::s_mappingRegistered = false;

void GetChapterQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Chapter, Contracts::DTO::Chapter::ChapterDTO>(true);
}
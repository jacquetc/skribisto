#include "get_chapter_with_details_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Application::Features::Chapter::Queries;

GetChapterWithDetailsQueryHandler::GetChapterWithDetailsQueryHandler(
    QSharedPointer<InterfaceChapterRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<ChapterWithDetailsDTO> GetChapterWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                        const GetChapterQuery &query)
{
    Result<ChapterWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<ChapterWithDetailsDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterQuery:" << ex.what();
    }
    return result;
}

Result<ChapterWithDetailsDTO> GetChapterWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                            const GetChapterQuery &query)
{
    qDebug() << "GetChapterWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto chapterResult = m_repository->get(query.id);

    if (Q_UNLIKELY(chapterResult.isError()))
    {
        return Result<ChapterWithDetailsDTO>(chapterResult.error());
    }

    Domain::Chapter chapter = chapterResult.value();

    // set up lazy loading:

    chapter.setScenesLoader(m_repository->fetchScenesLoader());

    // map
    auto WithDetailsDTO = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterWithDetailsDTO>(chapterResult.value());

    qDebug() << "GetChapterWithDetailsQueryHandler::handleImpl done";

    return Result<ChapterWithDetailsDTO>(WithDetailsDTO);
}

bool GetChapterWithDetailsQueryHandler::s_mappingRegistered = false;

void GetChapterWithDetailsQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Chapter, Contracts::DTO::Chapter::ChapterWithDetailsDTO>();
}
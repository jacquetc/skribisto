// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_chapter_with_details_query_handler.h"
#include "repository/interface_chapter_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Chapter::Queries;

GetChapterWithDetailsQueryHandler::GetChapterWithDetailsQueryHandler(InterfaceChapterRepository *repository)
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
        result = Result<ChapterWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<ChapterWithDetailsDTO> GetChapterWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                            const GetChapterQuery &query)
{
    qDebug() << "GetChapterWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto chapterResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(ChapterWithDetailsDTO, chapterResult)

    Skribisto::Domain::Chapter chapter = chapterResult.value();

    // map
    auto chapterWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterWithDetailsDTO>(chapter);

    qDebug() << "GetChapterWithDetailsQueryHandler::handleImpl done";

    return Result<ChapterWithDetailsDTO>(chapterWithDetailsDTO);
}

bool GetChapterWithDetailsQueryHandler::s_mappingRegistered = false;

void GetChapterWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Chapter,
                                                           Contracts::DTO::Chapter::ChapterWithDetailsDTO>();
}
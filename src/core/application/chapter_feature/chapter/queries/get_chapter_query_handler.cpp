// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_chapter_query_handler.h"
#include "repository/interface_chapter_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Chapter::Queries;

GetChapterQueryHandler::GetChapterQueryHandler(InterfaceChapterRepository *repository) : m_repository(repository)
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
        result = Result<ChapterDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<ChapterDTO> GetChapterQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                      const GetChapterQuery &query)
{
    qDebug() << "GetChapterQueryHandler::handleImpl called with id" << query.id;

    // do
    auto chapterResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(ChapterDTO, chapterResult)

    // map
    auto dto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Chapter, ChapterDTO>(chapterResult.value());

    qDebug() << "GetChapterQueryHandler::handleImpl done";

    return Result<ChapterDTO>(dto);
}

bool GetChapterQueryHandler::s_mappingRegistered = false;

void GetChapterQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Chapter,
                                                           Contracts::DTO::Chapter::ChapterDTO>(true, true);
}
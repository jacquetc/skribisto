#include "get_chapter_request_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Application::Features::Chapter::Queries;

GetChapterRequestHandler::GetChapterRequestHandler(QSharedPointer<InterfaceChapterRepository>repository)
    : Handler(), m_repository(repository)
{}

Result<ChapterDTO>GetChapterRequestHandler::handle(QPromise<Result<void> >& progressPromise,
                                                   const GetChapterRequest& request)
{
    Result<ChapterDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception& ex)
    {
        result = Result<ChapterDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterRequest:" << ex.what();
    }
    return result;
}

Result<ChapterDTO>GetChapterRequestHandler::handleImpl(const GetChapterRequest& request)
{
    qDebug() << "GetChapterRequestHandler::handleImpl called with id" << request.id;

    // do
    auto chapterResult = m_repository->get(request.id);

    if (chapterResult.isError())
    {
        return Result<ChapterDTO>(chapterResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapterResult.value());

    qDebug() << "GetChapterRequestHandler::handleImpl done";

    return Result<ChapterDTO>(dto);
}

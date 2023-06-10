#include "get_chapter_list_request_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_chapter_repository.h"

using namespace Application::Features::Chapter::Queries;

GetChapterListRequestHandler::GetChapterListRequestHandler(QSharedPointer<InterfaceChapterRepository>repository)
    : Handler(), m_repository(repository)
{}

Result<QList<ChapterDTO> >GetChapterListRequestHandler::handle(QPromise<Result<void> >& progressPromise)
{
    qDebug() << "GetChapterListRequestHandler::handle called";

    Result<QList<ChapterDTO> > result;

    try
    {
        result = handleImpl();
    }
    catch (const std::exception& ex)
    {
        result = Result<QList<ChapterDTO> >(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetChapterListRequest:" << ex.what();
    }
    return result;
}

Result<QList<ChapterDTO> >GetChapterListRequestHandler::handleImpl()
{
    qDebug() << "GetChapterListRequestHandler::handleImpl called";

    // do
    auto chapterResult = m_repository->getAll();

    if (chapterResult.isError())
    {
        return Result<QList<ChapterDTO> >(chapterResult.error());
    }

    // map
    QList<ChapterDTO> dtoList;

    for (const Domain::Chapter& chapter : chapterResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<Domain::Chapter, ChapterDTO>(chapter);
        dtoList.append(dto);
    }

    qDebug() << "GetChapterListRequestHandler::handleImpl done";

    return Result<QList<ChapterDTO> >(dtoList);
}

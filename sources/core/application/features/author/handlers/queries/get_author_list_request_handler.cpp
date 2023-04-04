#include "get_author_list_request_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Application::Features::Author::Queries;

GetAuthorListRequestHandler::GetAuthorListRequestHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : Handler(), m_repository(repository)
{
}

Result<QList<AuthorDTO>> GetAuthorListRequestHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAuthorListRequestHandler::handle called";

    Result<QList<AuthorDTO>> result;

    try
    {
        result = handleImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<AuthorDTO>>(Error(this, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAuthorListRequest:" << ex.what();
    }

    return result;
}

Result<QList<AuthorDTO>> GetAuthorListRequestHandler::handleImpl()
{
    qDebug() << "GetAuthorListRequestHandler::handleImpl called";

    // do
    auto authorResult = m_repository->getAll();

    if (authorResult.isError())
    {
        return Result<QList<AuthorDTO>>(authorResult.error());
    }

    // map
    QList<AuthorDTO> dtoList;

    for (const Domain::Author &author : authorResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<AuthorDTO>(author);
        dtoList.append(dto);
    }

    qDebug() << "GetAuthorListRequestHandler::handleImpl done";

    return Result<QList<AuthorDTO>>(dtoList);
}

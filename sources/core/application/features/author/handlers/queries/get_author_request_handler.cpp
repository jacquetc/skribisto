#include "get_author_request_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Application::Features::Author::Queries;

GetAuthorRequestHandler::GetAuthorRequestHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : Handler(), m_repository(repository)
{
}

Result<AuthorDTO> GetAuthorRequestHandler::handle(QPromise<Result<void>> &progressPromise,
                                                  const GetAuthorRequest &request)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(request);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAuthorRequest:" << ex.what();
    }

    return result;
}

Result<AuthorDTO> GetAuthorRequestHandler::handleImpl(const GetAuthorRequest &request)
{
    qDebug() << "GetAuthorRequestHandler::handleImpl called with id" << request.id;

    // do
    auto authorResult = m_repository->get(request.id);

    if (authorResult.isError())
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<AuthorDTO>(authorResult.value());

    qDebug() << "GetAuthorRequestHandler::handleImpl done";

    return Result<AuthorDTO>(dto);
}

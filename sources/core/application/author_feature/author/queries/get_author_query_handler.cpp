#include "get_author_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Application::Features::Author::Queries;

GetAuthorQueryHandler::GetAuthorQueryHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<AuthorDTO> GetAuthorQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetAuthorQuery &query)
{
    Result<AuthorDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<AuthorDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAuthorQuery:" << ex.what();
    }
    return result;
}

Result<AuthorDTO> GetAuthorQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const GetAuthorQuery &query)
{
    qDebug() << "GetAuthorQueryHandler::handleImpl called with id" << query.id;

    // do
    auto authorResult = m_repository->get(query.id);

    if (Q_UNLIKELY(!authorResult.isError()))
    {
        return Result<AuthorDTO>(authorResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(authorResult.value());

    qDebug() << "GetAuthorQueryHandler::handleImpl done";

    return Result<AuthorDTO>(dto);
}

bool GetAuthorQueryHandler::s_mappingRegistered = false;

void GetAuthorQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
}
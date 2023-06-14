#include "get_all_author_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_author_repository.h"

using namespace Application::Features::Author::Queries;

GetAllAuthorQueryHandler::GetAllAuthorQueryHandler(QSharedPointer<InterfaceAuthorRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<AuthorDTO>> GetAllAuthorQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllAuthorQueryHandler::handle called";

    Result<QList<AuthorDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<AuthorDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllAuthorQuery:" << ex.what();
    }
    return result;
}

Result<QList<AuthorDTO>> GetAllAuthorQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllAuthorQueryHandler::handleImpl called";

    // do
    auto authorResult = m_repository->getAll();

    if (Q_UNLIKELY(!authorResult.isError()))
    {
        return Result<QList<AuthorDTO>>(authorResult.error());
    }

    // map
    QList<AuthorDTO> dtoList;

    for (const Domain::Author &author : authorResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<Domain::Author, AuthorDTO>(author);
        dtoList.append(dto);
    }

    qDebug() << "GetAllAuthorQueryHandler::handleImpl done";

    return Result<QList<AuthorDTO>>(dtoList);
}

bool GetAllAuthorQueryHandler::s_mappingRegistered = false;

void GetAllAuthorQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_book_query_handler.h"
#include "repository/interface_book_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Book::Queries;

GetBookQueryHandler::GetBookQueryHandler(InterfaceBookRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BookDTO> GetBookQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetBookQuery &query)
{
    Result<BookDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<BookDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetBookQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<BookDTO> GetBookQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetBookQuery &query)
{
    qDebug() << "GetBookQueryHandler::handleImpl called with id" << query.id;

    // do
    auto bookResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(BookDTO, bookResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Book, BookDTO>(bookResult.value());

    qDebug() << "GetBookQueryHandler::handleImpl done";

    return Result<BookDTO>(dto);
}

bool GetBookQueryHandler::s_mappingRegistered = false;

void GetBookQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Book, Contracts::DTO::Book::BookDTO>(
        true, true);
}
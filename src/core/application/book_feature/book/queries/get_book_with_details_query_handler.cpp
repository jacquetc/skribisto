// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_book_with_details_query_handler.h"
#include "repository/interface_book_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Book::Queries;

GetBookWithDetailsQueryHandler::GetBookWithDetailsQueryHandler(InterfaceBookRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<BookWithDetailsDTO> GetBookWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                  const GetBookQuery &query)
{
    Result<BookWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<BookWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetBookQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<BookWithDetailsDTO> GetBookWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                      const GetBookQuery &query)
{
    qDebug() << "GetBookWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto bookResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(BookWithDetailsDTO, bookResult)

    Skribisto::Domain::Book book = bookResult.value();

    // map
    auto bookWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Book, BookWithDetailsDTO>(book);

    qDebug() << "GetBookWithDetailsQueryHandler::handleImpl done";

    return Result<BookWithDetailsDTO>(bookWithDetailsDTO);
}

bool GetBookWithDetailsQueryHandler::s_mappingRegistered = false;

void GetBookWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Book,
                                                           Contracts::DTO::Book::BookWithDetailsDTO>();
}
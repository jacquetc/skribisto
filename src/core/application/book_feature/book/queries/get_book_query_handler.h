// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_book_export.h"
#include "book/book_dto.h"
#include "book/queries/get_book_query.h"

#include "repository/interface_book_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Book;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Book::Queries;

namespace Skribisto::Application::Features::Book::Queries
{
class SKRIBISTO_APPLICATION_BOOK_EXPORT GetBookQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetBookQueryHandler(InterfaceBookRepository *repository);
    Result<BookDTO> handle(QPromise<Result<void>> &progressPromise, const GetBookQuery &query);

  private:
    InterfaceBookRepository *m_repository;
    Result<BookDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetBookQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Book::Queries
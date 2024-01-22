// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_book_export.h"
#include "book/book_dto.h"
#include "book/commands/update_book_command.h"

#include "repository/interface_book_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Book;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Book::Commands;

namespace Skribisto::Application::Features::Book::Commands
{
class SKRIBISTO_APPLICATION_BOOK_EXPORT UpdateBookCommandHandler : public QObject

{
    Q_OBJECT
  public:
    UpdateBookCommandHandler(InterfaceBookRepository *repository);
    Result<BookDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateBookCommand &request);
    Result<BookDTO> restore();

  signals:
    void bookUpdated(Skribisto::Contracts::DTO::Book::BookDTO bookDto);
    void bookDetailsUpdated(int id);

  private:
    InterfaceBookRepository *m_repository;
    Result<BookDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateBookCommand &request);
    Result<BookDTO> restoreImpl();
    Result<BookDTO> saveOldState();
    Result<BookDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Book::Commands
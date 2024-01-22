// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "book/book_dto.h"
#include "book/book_with_details_dto.h"
#include "book/update_book_dto.h"
#include "controller_export.h"
#include "event_dispatcher.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::Book;

namespace Skribisto::Controller::Book
{

class SKRIBISTO_CONTROLLER_EXPORT BookController : public QObject
{
    Q_OBJECT
  public:
    explicit BookController(InterfaceRepositoryProvider *repositoryProvider, ThreadedUndoRedoSystem *undo_redo_system,
                            QSharedPointer<EventDispatcher> eventDispatcher);

    static BookController *instance();

    Q_INVOKABLE QCoro::Task<BookDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<BookWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE static Contracts::DTO::Book::UpdateBookDTO getUpdateDTO();

  public slots:

    QCoro::Task<BookDTO> update(const UpdateBookDTO &dto);

  private:
    static QPointer<BookController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    BookController() = delete;
    BookController(const BookController &) = delete;
    BookController &operator=(const BookController &) = delete;
};

} // namespace Skribisto::Controller::Book
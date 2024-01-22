// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "book_controller.h"

#include "book/commands/update_book_command.h"
#include "book/commands/update_book_command_handler.h"
#include "book/queries/get_book_query_handler.h"
#include "book/queries/get_book_with_details_query_handler.h"
// #include "book/commands/insert_book_into_xxx_command.h"
// #include "book/commands/update_book_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::Book;
using namespace Skribisto::Application::Features::Book::Commands;
using namespace Skribisto::Application::Features::Book::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<BookController> BookController::s_instance = nullptr;

BookController::BookController(InterfaceRepositoryProvider *repositoryProvider,
                               ThreadedUndoRedoSystem *undo_redo_system,
                               QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    s_instance = this;
}

BookController *BookController::instance()
{
    return s_instance.data();
}

QCoro::Task<BookDTO> BookController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetBookQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));
        GetBookQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->book()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "book");

    // async wait for result signal
    const std::optional<BookDTO> optional_result =
        co_await qCoro(m_eventDispatcher->book(), &BookSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return BookDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<BookWithDetailsDTO> BookController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetBookQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));
        GetBookWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->book()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "book");

    // async wait for result signal
    const std::optional<BookWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->book(), &BookSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return BookWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<BookDTO> BookController::update(const UpdateBookDTO &dto)
{
    UpdateBookCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));

    auto *handler = new UpdateBookCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateBookCommandHandler::bookUpdated, this,
                     [this](BookDTO dto) { emit m_eventDispatcher->book()->updated(dto); });
    QObject::connect(handler, &UpdateBookCommandHandler::bookDetailsUpdated, m_eventDispatcher->book(),
                     &BookSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateBookCommandHandler, UpdateBookCommand>(BookController::tr("Update book"),
                                                                                 handler, query);

    // push command
    m_undo_redo_system->push(command, "book");

    // async wait for result signal
    const std::optional<BookDTO> optional_result =
        co_await qCoro(handler, &UpdateBookCommandHandler::bookUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return BookDTO();
    }

    co_return optional_result.value();
}

UpdateBookDTO BookController::getUpdateDTO()
{
    return UpdateBookDTO();
}

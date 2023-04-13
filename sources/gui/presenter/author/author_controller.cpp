#include "author_controller.h"

#include "cqrs/author/commands/create_author_command.h"
#include "cqrs/author/commands/remove_author_command.h"
#include "cqrs/author/commands/update_author_command.h"
#include "features/author/handlers/commands/create_author_command_handler.h"
#include "features/author/handlers/commands/update_author_command_handler.h"
#include "features/author/handlers/queries/get_author_list_request_handler.h"
#include "features/author/handlers/queries/get_author_request_handler.h"
#include "undo_redo/alter_command.h"
#include "undo_redo/query_command.h"

#include "features/author/handlers/commands/create_author_command_handler.h"
#include "features/author/handlers/commands/remove_author_command_handler.h"
#include "features/author/handlers/commands/update_author_command_handler.h"

using namespace Presenter::Author;
using namespace Presenter::UndoRedo;
using namespace Application::Features::Author::Queries;
using namespace Contracts::CQRS::Author::Commands;
using namespace Application::Features::Author::Commands;

QScopedPointer<AuthorController> AuthorController::s_instance = QScopedPointer<AuthorController>(nullptr);
InterfaceRepositoryProvider *AuthorController::s_repositoryProvider;
ThreadedUndoRedoSystem *AuthorController::s_undo_redo_system;
/*!
    \class AuthorController
    \brief The AuthorController class provides an asynchronous interface for managing Author objects.

    The AuthorController class provides methods for getting, creating, updating, and removing authors in an
    asynchronous manner. It uses the Command and Query Responsibility Segregation (CQRS) pattern to
    separate read and write operations. The class also implements the Singleton pattern, ensuring that
    only one instance of the controller is active at any time.

    The AuthorSignalBridge class is used to emit signals for Author events, such as authorCreated,
    authorRemoved, and authorUpdated. These signals are connected to the corresponding slots in the
    AuthorController class. AuthorSignalBridge isn't meant to be used directly by the UI, only from
    inside the UndoRedoCommand and QueryCommand, the signals are available directly from AuthorController.

    \section1 Example Usage

    \code
    // Get an Author by ID
    AuthorController::instance()->getAsync(authorId);
    \endcode

    \sa AuthorSignalBridge
*/

/*!
    \brief Constructs an AuthorController with the given \a repositoryProvider.
*/
AuthorController::AuthorController(InterfaceRepositoryProvider *repositoryProvider) : QObject(nullptr)
{
    s_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    s_undo_redo_system = ThreadedUndoRedoSystem::instance();

    s_instance.reset(this);
}

/*!
    \brief Returns the singleton instance of the AuthorController.
*/
AuthorController *AuthorController::instance()
{
    return s_instance.data();
}

/*!
    \brief Retrieves an author asynchronously by the given \a uuid.

    When the operation is complete, the getAuthorReplied signal is emitted.
*/
void AuthorController::getAsync(int id)
{

    auto queryCommand = new QueryCommand("get");
    queryCommand->setQueryFunction([=](QPromise<Result<void>> &progressPromise) {
        GetAuthorRequest request;
        request.id = id;
        auto interface = qSharedPointerCast<InterfaceAuthorRepository>(
            s_repositoryProvider->repository(InterfaceRepositoryProvider::Author));
        GetAuthorRequestHandler handler(interface);
        auto result = handler.handle(progressPromise, request);
        if (result.isSuccess())
        {
            emit AuthorController::instance()->getAuthorReplied(result.value());
        }
        return Result<void>(result.error());
    });

    s_undo_redo_system->push(queryCommand, "author");
}

/*!
    \brief Retrieves all authors asynchronously.

    When the operation is complete, the getAuthorListReplied signal is emitted.
*/
void AuthorController::getAllAsync()
{
    auto queryCommand = new QueryCommand("getAll");
    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = qSharedPointerCast<InterfaceAuthorRepository>(
            s_repositoryProvider->repository(InterfaceRepositoryProvider::Author));
        GetAuthorListRequestHandler handler(interface);
        auto result = handler.handle(progressPromise);
        if (result.isSuccess())
        {
            emit AuthorController::instance()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    s_undo_redo_system->push(queryCommand, "author");
}

/*!
    \brief Creates a new author asynchronously with the given \a dto.

    When the operation is complete, the authorCreated signal is emitted.
*/
void AuthorController::createAsync(const CreateAuthorDTO &dto)
{
    CreateAuthorCommand request;
    request.req = dto;

    auto repository = qSharedPointerCast<InterfaceAuthorRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Author));

    auto *handler = new CreateAuthorCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateAuthorCommandHandler::authorCreated, AuthorController::instance(),
                     &AuthorController::authorCreated);
    QObject::connect(handler, &CreateAuthorCommandHandler::authorRemoved, AuthorController::instance(),
                     &AuthorController::authorRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateAuthorCommandHandler, CreateAuthorCommand>(
        AuthorController::tr("Create author"), handler, request);

    // push command
    s_undo_redo_system->push(command, "author");
}

/*!
    \brief Updates an existing author asynchronously with the given \a dto.

    When the operation is complete, the authorUpdated signal is emitted.
*/
void AuthorController::updateAsync(const UpdateAuthorDTO &dto)
{
    UpdateAuthorCommand request;
    request.req = dto;

    auto repository = qSharedPointerCast<InterfaceAuthorRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Author));

    auto *handler = new UpdateAuthorCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateAuthorCommandHandler::authorUpdated, AuthorController::instance(),
                     &AuthorController::authorUpdated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateAuthorCommandHandler, UpdateAuthorCommand>(
        AuthorController::tr("Update author"), handler, request);

    // push command
    s_undo_redo_system->push(command, "author");
}

/*!
    \brief Removes an author asynchronously by the given \a uuid.

    When the operation is complete, the authorRemoved signal is emitted.
*/
void AuthorController::removeAsync(int id)
{
    RemoveAuthorCommand request;
    request.id = id;

    auto repository = qSharedPointerCast<InterfaceAuthorRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Author));

    auto *handler = new RemoveAuthorCommandHandler(repository);

    // connect
    QObject::connect(handler, &RemoveAuthorCommandHandler::authorCreated, AuthorController::instance(),
                     &AuthorController::authorCreated);
    QObject::connect(handler, &RemoveAuthorCommandHandler::authorRemoved, AuthorController::instance(),
                     &AuthorController::authorRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveAuthorCommandHandler, RemoveAuthorCommand>(
        AuthorController::tr("Remove author"), handler, request);

    // push command
    s_undo_redo_system->push(command, "author");
}

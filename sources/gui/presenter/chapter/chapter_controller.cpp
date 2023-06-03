#include "chapter_controller.h"

#include "cqrs/chapter/commands/create_chapter_command.h"
#include "cqrs/chapter/commands/remove_chapter_command.h"
#include "cqrs/chapter/commands/update_chapter_command.h"
#include "features/chapter/handlers/commands/create_chapter_command_handler.h"
#include "features/chapter/handlers/commands/update_chapter_command_handler.h"
#include "features/chapter/handlers/queries/get_chapter_list_request_handler.h"
#include "features/chapter/handlers/queries/get_chapter_request_handler.h"
#include "undo_redo/alter_command.h"
#include "undo_redo/query_command.h"

#include "features/chapter/handlers/commands/create_chapter_command_handler.h"
#include "features/chapter/handlers/commands/remove_chapter_command_handler.h"
#include "features/chapter/handlers/commands/update_chapter_command_handler.h"

using namespace Presenter::Chapter;
using namespace Presenter::UndoRedo;
using namespace Application::Features::Chapter::Queries;
using namespace Contracts::CQRS::Chapter::Commands;
using namespace Application::Features::Chapter::Commands;

QScopedPointer<ChapterController> ChapterController::s_instance = QScopedPointer<ChapterController>(nullptr);
InterfaceRepositoryProvider *ChapterController::s_repositoryProvider;
ThreadedUndoRedoSystem *ChapterController::s_undo_redo_system;
/*!
    \class ChapterController
    \brief The ChapterController class provides an asynchronous interface for managing Chapter objects.

    The ChapterController class provides methods for getting, creating, updating, and removing chapters in an
    asynchronous manner. It uses the Command and Query Responsibility Segregation (CQRS) pattern to
    separate read and write operations. The class also implements the Singleton pattern, ensuring that
    only one instance of the controller is active at any time.

    The ChapterSignalBridge class is used to emit signals for Chapter events, such as chapterCreated,
    chapterRemoved, and chapterUpdated. These signals are connected to the corresponding slots in the
    ChapterController class. ChapterSignalBridge isn't meant to be used directly by the UI, only from
    inside the UndoRedoCommand and QueryCommand, the signals are available directly from ChapterController.

    \section1 Example Usage

    \code
    // Get an Chapter by ID
    ChapterController::instance()->getAsync(chapterId);
    \endcode

    \sa ChapterSignalBridge
*/

/*!
    \brief Constructs an ChapterController with the given \a repositoryProvider.
*/
ChapterController::ChapterController(InterfaceRepositoryProvider *repositoryProvider) : QObject(nullptr)
{
    s_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    s_undo_redo_system = ThreadedUndoRedoSystem::instance();

    s_instance.reset(this);
}

/*!
    \brief Returns the singleton instance of the ChapterController.
*/
ChapterController *ChapterController::instance()
{
    return s_instance.data();
}

/*!
    \brief Retrieves an chapter asynchronously by the given \a uuid.

    When the operation is complete, the getChapterReplied signal is emitted.
*/
void ChapterController::get(int id)
{

    auto queryCommand = new QueryCommand("get");
    queryCommand->setQueryFunction([=](QPromise<Result<void>> &progressPromise) {
        GetChapterRequest request;
        request.id = id;
        auto interface = qSharedPointerCast<InterfaceChapterRepository>(
            s_repositoryProvider->repository(InterfaceRepositoryProvider::Chapter));
        GetChapterRequestHandler handler(interface);
        auto result = handler.handle(progressPromise, request);
        if (result.isSuccess())
        {
            emit ChapterController::instance()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    s_undo_redo_system->push(queryCommand, "chapter");
}

/*!
    \brief Retrieves all chapters asynchronously.

    When the operation is complete, the getChapterListReplied signal is emitted.
*/
void ChapterController::getAll()
{
    auto queryCommand = new QueryCommand("getAll");
    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = qSharedPointerCast<InterfaceChapterRepository>(
            s_repositoryProvider->repository(InterfaceRepositoryProvider::Chapter));
        GetChapterListRequestHandler handler(interface);
        auto result = handler.handle(progressPromise);
        if (result.isSuccess())
        {
            emit ChapterController::instance()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    s_undo_redo_system->push(queryCommand, "chapter");
}

/*!
    \brief Creates a new chapter asynchronously with the given \a dto.

    When the operation is complete, the chapterCreated signal is emitted.
*/
void ChapterController::create(const CreateChapterDTO &dto)
{
    CreateChapterCommand request;
    request.req = dto;

    auto repository = qSharedPointerCast<InterfaceChapterRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Chapter));

    auto *handler = new CreateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateChapterCommandHandler::chapterCreated, ChapterController::instance(),
                     &ChapterController::chapterCreated);
    QObject::connect(handler, &CreateChapterCommandHandler::chapterRemoved, ChapterController::instance(),
                     &ChapterController::chapterRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateChapterCommandHandler, CreateChapterCommand>(
        ChapterController::tr("Create chapter"), handler, request);

    // push command
    s_undo_redo_system->push(command, "chapter");
}

/*!
    \brief Updates an existing chapter asynchronously with the given \a dto.

    When the operation is complete, the chapterUpdated signal is emitted.
*/
void ChapterController::update(const UpdateChapterDTO &dto)
{
    UpdateChapterCommand request;
    request.req = dto;

    auto repository = qSharedPointerCast<InterfaceChapterRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Chapter));

    auto *handler = new UpdateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateChapterCommandHandler::chapterUpdated, ChapterController::instance(),
                     &ChapterController::chapterUpdated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateChapterCommandHandler, UpdateChapterCommand>(
        ChapterController::tr("Update chapter"), handler, request);

    // push command
    s_undo_redo_system->push(command, "chapter");
}

/*!
    \brief Removes an chapter asynchronously by the given \a uuid.

    When the operation is complete, the chapterRemoved signal is emitted.
*/
void ChapterController::remove(int id)
{
    RemoveChapterCommand request;
    request.id = id;

    auto repository = qSharedPointerCast<InterfaceChapterRepository>(
        s_repositoryProvider->repository(InterfaceRepositoryProvider::Chapter));

    auto *handler = new RemoveChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &RemoveChapterCommandHandler::chapterCreated, ChapterController::instance(),
                     &ChapterController::chapterCreated);
    QObject::connect(handler, &RemoveChapterCommandHandler::chapterRemoved, ChapterController::instance(),
                     &ChapterController::chapterRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveChapterCommandHandler, RemoveChapterCommand>(
        ChapterController::tr("Remove chapter"), handler, request);

    // push command
    s_undo_redo_system->push(command, "chapter");
}

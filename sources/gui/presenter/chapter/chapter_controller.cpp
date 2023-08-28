#include "chapter_controller.h"

#include "chapter/commands/create_chapter_command.h"
#include "chapter/commands/create_chapter_command_handler.h"
#include "chapter/commands/remove_chapter_command.h"
#include "chapter/commands/update_chapter_command.h"
#include "chapter/commands/update_chapter_command_handler.h"
#include "chapter/queries/get_all_chapter_query_handler.h"
#include "chapter/queries/get_chapter_query_handler.h"
#include "undo_redo/alter_command.h"
#include "undo_redo/query_command.h"

#include "chapter/commands/create_chapter_command_handler.h"
#include "chapter/commands/remove_chapter_command_handler.h"
#include "chapter/commands/update_chapter_command_handler.h"

using namespace Presenter::Chapter;
using namespace Presenter::UndoRedo;
using namespace Application::Features::Chapter::Queries;
using namespace Contracts::CQRS::Chapter::Commands;
using namespace Application::Features::Chapter::Commands;

QScopedPointer<ChapterController> ChapterController::s_instance = QScopedPointer<ChapterController>(nullptr);

/*!
    \class ChapterController
    \brief The ChapterController class provides an asynchronous interface for
       managing Chapter objects.

    The ChapterController class provides methods for getting, creating,
       updating, and removing chapters in an
    asynchronous manner.
    The signals are available directly from ChapterController.

    \section1 Example Usage

    \code
    // Get an Chapter by ID
    ChapterController::instance()->get(chapterId);
    \endcode
 */

/*!
    \brief Constructs an ChapterController with the given \a repositoryProvider.
 */
ChapterController::ChapterController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                     ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher)
    : QObject(parent)
{
    m_repositoryProvider = repositoryProvider;

    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

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
    \brief Retrieves an chapter asynchronously by the given \a id.

    When the operation is complete, the getChapterReplied signal is emitted.
 */
void ChapterController::get(int id)
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([=](QPromise<Result<void>> &progressPromise) {
        GetChapterQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));
        GetChapterQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit ChapterController::instance()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "chapter");
}

/*!
 * \brief Retrieves a chapter asynchronously with details by the given \a id.
 *
 * When the operation is complete, the getChapterWithDetailsReplied signal is
 * emitted.

 * \param id
 */
void ChapterController::getWithDetails(int id)
{
}

/*!
    \brief Retrieves all chapters asynchronously.

    When the operation is complete, the getChapterListReplied signal is emitted.
 */
void ChapterController::getAll()
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));
        GetAllChapterQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit ChapterController::instance()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "chapter");
}

/*!
    \brief Creates a new chapter asynchronously with the given \a dto.

    When the operation is complete, the chapterCreated signal is emitted.
 */
void ChapterController::create(const CreateChapterDTO &dto)
{
    CreateChapterCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new CreateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateChapterCommandHandler::chapterCreated, ChapterController::instance(),
                     &ChapterController::chapterCreated);
    QObject::connect(handler, &CreateChapterCommandHandler::chapterRemoved, ChapterController::instance(),
                     &ChapterController::chapterRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateChapterCommandHandler, CreateChapterCommand>(
        ChapterController::tr("Create chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");
}

/*!
    \brief Updates an existing chapter asynchronously with the given \a dto.

    When the operation is complete, the chapterUpdated signal is emitted.
 */
void ChapterController::update(const UpdateChapterDTO &dto)
{
    UpdateChapterCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new UpdateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateChapterCommandHandler::chapterUpdated, ChapterController::instance(),
                     &ChapterController::chapterUpdated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateChapterCommandHandler, UpdateChapterCommand>(
        ChapterController::tr("Update chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");
}

/*!
    \brief Removes an chapter asynchronously by the given \a uuid.

    When the operation is complete, the chapterRemoved signal is emitted.
 */
void ChapterController::remove(int id)
{
    RemoveChapterCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new RemoveChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &RemoveChapterCommandHandler::chapterCreated, ChapterController::instance(),
                     &ChapterController::chapterCreated);
    QObject::connect(handler, &RemoveChapterCommandHandler::chapterRemoved, ChapterController::instance(),
                     &ChapterController::chapterRemoved);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveChapterCommandHandler, RemoveChapterCommand>(
        ChapterController::tr("Remove chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");
}

CreateChapterDTO ChapterController::getCreateChapterDTO()
{
    return CreateChapterDTO();
}

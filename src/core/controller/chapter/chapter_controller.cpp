// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "chapter_controller.h"

#include "chapter/commands/create_chapter_command.h"
#include "chapter/commands/create_chapter_command_handler.h"
#include "chapter/commands/remove_chapter_command.h"
#include "chapter/commands/remove_chapter_command_handler.h"
#include "chapter/commands/update_chapter_command.h"
#include "chapter/commands/update_chapter_command_handler.h"
#include "chapter/queries/get_chapter_query_handler.h"
#include "chapter/queries/get_chapter_with_details_query_handler.h"
// #include "chapter/commands/insert_chapter_into_xxx_command.h"
// #include "chapter/commands/update_chapter_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::Chapter;
using namespace Skribisto::Application::Features::Chapter::Commands;
using namespace Skribisto::Application::Features::Chapter::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<ChapterController> ChapterController::s_instance = nullptr;

ChapterController::ChapterController(InterfaceRepositoryProvider *repositoryProvider,
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

ChapterController *ChapterController::instance()
{
    return s_instance.data();
}

QCoro::Task<ChapterDTO> ChapterController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetChapterQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));
        GetChapterQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->chapter()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "chapter");

    // async wait for result signal
    const std::optional<ChapterDTO> optional_result =
        co_await qCoro(m_eventDispatcher->chapter(), &ChapterSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return ChapterDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ChapterWithDetailsDTO> ChapterController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetChapterQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));
        GetChapterWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->chapter()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "chapter");

    // async wait for result signal
    const std::optional<ChapterWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->chapter(), &ChapterSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return ChapterWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ChapterDTO> ChapterController::create(const CreateChapterDTO &dto)
{
    CreateChapterCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new CreateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateChapterCommandHandler::chapterCreated, m_eventDispatcher->chapter(),
                     &ChapterSignals::created);

    QObject::connect(handler, &CreateChapterCommandHandler::relationWithOwnerInserted, this,
                     [this](int id, int ownerId, int position) {
                         auto dto = BookRelationDTO(ownerId, BookRelationDTO::RelationField::Chapters,
                                                    QList<int>() << id, position);
                         emit m_eventDispatcher->book()->relationInserted(dto);
                     });
    QObject::connect(
        handler, &CreateChapterCommandHandler::relationWithOwnerRemoved, this, [this](int id, int ownerId) {
            auto dto = BookRelationDTO(ownerId, BookRelationDTO::RelationField::Chapters, QList<int>() << id, -1);
            emit m_eventDispatcher->book()->relationRemoved(dto);
        });

    QObject::connect(handler, &CreateChapterCommandHandler::chapterRemoved, this,
                     [this](int removedId) { emit m_eventDispatcher->chapter()->removed(QList<int>() << removedId); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateChapterCommandHandler, CreateChapterCommand>(
        ChapterController::tr("Create chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");

    // async wait for result signal
    const std::optional<ChapterDTO> optional_result =
        co_await qCoro(handler, &CreateChapterCommandHandler::chapterCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return ChapterDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<ChapterDTO> ChapterController::update(const UpdateChapterDTO &dto)
{
    UpdateChapterCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new UpdateChapterCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateChapterCommandHandler::chapterUpdated, this,
                     [this](ChapterDTO dto) { emit m_eventDispatcher->chapter()->updated(dto); });
    QObject::connect(handler, &UpdateChapterCommandHandler::chapterDetailsUpdated, m_eventDispatcher->chapter(),
                     &ChapterSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateChapterCommandHandler, UpdateChapterCommand>(
        ChapterController::tr("Update chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");

    // async wait for result signal
    const std::optional<ChapterDTO> optional_result =
        co_await qCoro(handler, &UpdateChapterCommandHandler::chapterUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return ChapterDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> ChapterController::remove(int id)
{
    RemoveChapterCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto *handler = new RemoveChapterCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveChapterCommandHandler, RemoveChapterCommand>(
        ChapterController::tr("Remove chapter"), handler, query);

    // push command
    m_undo_redo_system->push(command, "chapter");

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateChapterDTO ChapterController::getCreateDTO()
{
    return CreateChapterDTO();
}

UpdateChapterDTO ChapterController::getUpdateDTO()
{
    return UpdateChapterDTO();
}

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "system_controller.h"

#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include "system/commands/close_atelier_command.h"
#include "system/commands/close_atelier_command_handler.h"
#include "system/commands/create_new_atelier_command.h"
#include "system/commands/create_new_atelier_command_handler.h"
#include "system/commands/load_atelier_command.h"
#include "system/commands/load_atelier_command_handler.h"
#include "system/commands/save_atelier_command.h"
#include "system/commands/save_atelier_command_handler.h"
#include "system/queries/get_current_time_query.h"
#include "system/queries/get_current_time_query_handler.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::System;
using namespace Skribisto::Application::Features::System::Commands;
using namespace Skribisto::Application::Features::System::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<SystemController> SystemController::s_instance = nullptr;

SystemController::SystemController(InterfaceRepositoryProvider *repositoryProvider,
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

SystemController *SystemController::instance()
{
    return s_instance.data();
}

QCoro::Task<> SystemController::createNewAtelier(CreateNewAtelierDTO dto)
{
    CreateNewAtelierCommand query;

    query.req = dto;

    auto recentAtelierRepository =
        static_cast<InterfaceRecentAtelierRepository *>(m_repositoryProvider->repository("RecentAtelier"));

    auto atelierRepository = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));

    auto gitRepository = static_cast<InterfaceGitRepository *>(m_repositoryProvider->repository("Git"));

    auto workspaceRepository =
        static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));

    auto fileRepository = static_cast<InterfaceFileRepository *>(m_repositoryProvider->repository("File"));

    auto bookRepository = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));

    auto chapterRepository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto sceneRepository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto sceneParagraphRepository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new CreateNewAtelierCommandHandler(recentAtelierRepository, atelierRepository, gitRepository,
                                                       workspaceRepository, fileRepository, bookRepository,
                                                       chapterRepository, sceneRepository, sceneParagraphRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &CreateNewAtelierCommandHandler::createNewAtelierChanged, m_eventDispatcher->system(),
                     &SystemSignals::createNewAtelierChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateNewAtelierCommandHandler, CreateNewAtelierCommand>(
        SystemController::tr("Doing CreateNewAtelier"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "system");

    co_return;
}

QCoro::Task<> SystemController::loadAtelier(LoadAtelierDTO dto)
{
    LoadAtelierCommand query;

    query.req = dto;

    auto recentAtelierRepository =
        static_cast<InterfaceRecentAtelierRepository *>(m_repositoryProvider->repository("RecentAtelier"));

    auto atelierRepository = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));

    auto gitRepository = static_cast<InterfaceGitRepository *>(m_repositoryProvider->repository("Git"));

    auto workspaceRepository =
        static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));

    auto fileRepository = static_cast<InterfaceFileRepository *>(m_repositoryProvider->repository("File"));

    auto bookRepository = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));

    auto chapterRepository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto sceneRepository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto sceneParagraphRepository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new LoadAtelierCommandHandler(recentAtelierRepository, atelierRepository, gitRepository,
                                                  workspaceRepository, fileRepository, bookRepository,
                                                  chapterRepository, sceneRepository, sceneParagraphRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &LoadAtelierCommandHandler::loadAtelierChanged, m_eventDispatcher->system(),
                     &SystemSignals::loadAtelierChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<LoadAtelierCommandHandler, LoadAtelierCommand>(
        SystemController::tr("Doing LoadAtelier"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "system");

    co_return;
}

QCoro::Task<> SystemController::saveAtelier()
{
    SaveAtelierCommand query;

    auto recentAtelierRepository =
        static_cast<InterfaceRecentAtelierRepository *>(m_repositoryProvider->repository("RecentAtelier"));

    auto atelierRepository = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));

    auto gitRepository = static_cast<InterfaceGitRepository *>(m_repositoryProvider->repository("Git"));

    auto workspaceRepository =
        static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));

    auto fileRepository = static_cast<InterfaceFileRepository *>(m_repositoryProvider->repository("File"));

    auto bookRepository = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));

    auto chapterRepository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto sceneRepository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto sceneParagraphRepository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new SaveAtelierCommandHandler(recentAtelierRepository, atelierRepository, gitRepository,
                                                  workspaceRepository, fileRepository, bookRepository,
                                                  chapterRepository, sceneRepository, sceneParagraphRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &SaveAtelierCommandHandler::saveAtelierChanged, m_eventDispatcher->system(),
                     &SystemSignals::saveAtelierChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<SaveAtelierCommandHandler, SaveAtelierCommand>(
        SystemController::tr("Doing SaveAtelier"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "system");

    co_return;
}

QCoro::Task<> SystemController::closeAtelier()
{
    CloseAtelierCommand query;

    auto recentAtelierRepository =
        static_cast<InterfaceRecentAtelierRepository *>(m_repositoryProvider->repository("RecentAtelier"));

    auto atelierRepository = static_cast<InterfaceAtelierRepository *>(m_repositoryProvider->repository("Atelier"));

    auto gitRepository = static_cast<InterfaceGitRepository *>(m_repositoryProvider->repository("Git"));

    auto workspaceRepository =
        static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));

    auto fileRepository = static_cast<InterfaceFileRepository *>(m_repositoryProvider->repository("File"));

    auto bookRepository = static_cast<InterfaceBookRepository *>(m_repositoryProvider->repository("Book"));

    auto chapterRepository = static_cast<InterfaceChapterRepository *>(m_repositoryProvider->repository("Chapter"));

    auto sceneRepository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto *handler =
        new CloseAtelierCommandHandler(recentAtelierRepository, atelierRepository, gitRepository, workspaceRepository,
                                       fileRepository, bookRepository, chapterRepository, sceneRepository);

    Q_UNIMPLEMENTED();

    // connect

    QObject::connect(handler, &CloseAtelierCommandHandler::closeAtelierChanged, m_eventDispatcher->system(),
                     &SystemSignals::closeAtelierChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CloseAtelierCommandHandler, CloseAtelierCommand>(
        SystemController::tr("Doing CloseAtelier"), handler, query);

    // set progress minimum duration
    command->setProgressMinimumDuration(1000);
    m_eventDispatcher->progress()->bindToProgressSignals(command);

    // push command
    m_undo_redo_system->push(command, "system");

    co_return;
}

QCoro::Task<GetCurrentTimeReplyDTO> SystemController::getCurrentTime() const
{
    auto queryCommand = new QueryCommand("GetCurrentTime");

    Q_UNIMPLEMENTED();

    queryCommand->setQueryFunction([this](QPromise<Result<void>> &progressPromise) {
        GetCurrentTimeQuery query;

        GetCurrentTimeQueryHandler handler;
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->system()->getCurrentTimeReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "system");

    // async wait for result signal
    const std::optional<GetCurrentTimeReplyDTO> optional_result = co_await qCoro(
        m_eventDispatcher->system(), &SystemSignals::getCurrentTimeReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return GetCurrentTimeReplyDTO();
    }

    co_return optional_result.value();
}

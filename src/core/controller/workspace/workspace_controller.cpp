// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "workspace_controller.h"

#include "workspace/queries/get_all_workspace_query_handler.h"
#include "workspace/queries/get_workspace_query_handler.h"
#include "workspace/queries/get_workspace_with_details_query_handler.h"
// #include "workspace/commands/insert_workspace_into_xxx_command.h"
// #include "workspace/commands/update_workspace_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::Workspace;
using namespace Skribisto::Application::Features::Workspace::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<WorkspaceController> WorkspaceController::s_instance = nullptr;

WorkspaceController::WorkspaceController(InterfaceRepositoryProvider *repositoryProvider,
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

WorkspaceController *WorkspaceController::instance()
{
    return s_instance.data();
}

QCoro::Task<WorkspaceDTO> WorkspaceController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetWorkspaceQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));
        GetWorkspaceQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->workspace()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "workspace");

    // async wait for result signal
    const std::optional<WorkspaceDTO> optional_result =
        co_await qCoro(m_eventDispatcher->workspace(), &WorkspaceSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return WorkspaceDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<WorkspaceWithDetailsDTO> WorkspaceController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetWorkspaceQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));
        GetWorkspaceWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->workspace()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "workspace");

    // async wait for result signal
    const std::optional<WorkspaceWithDetailsDTO> optional_result =
        co_await qCoro(m_eventDispatcher.get()->workspace(), &WorkspaceSignals::getWithDetailsReplied,
                       std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return WorkspaceWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<QList<WorkspaceDTO>> WorkspaceController::getAll() const
{
    auto queryCommand = new QueryCommand("getAll");

    queryCommand->setQueryFunction([&](QPromise<Result<void>> &progressPromise) {
        auto interface = static_cast<InterfaceWorkspaceRepository *>(m_repositoryProvider->repository("Workspace"));
        GetAllWorkspaceQueryHandler handler(interface);
        auto result = handler.handle(progressPromise);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->workspace()->getAllReplied(result.value());
        }
        return Result<void>(result.error());
    });
    m_undo_redo_system->push(queryCommand, "workspace");

    // async wait for result signal
    const std::optional<QList<WorkspaceDTO>> optional_result = co_await qCoro(
        m_eventDispatcher->workspace(), &WorkspaceSignals::getAllReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return QList<WorkspaceDTO>() << WorkspaceDTO();
    }

    co_return optional_result.value();
}

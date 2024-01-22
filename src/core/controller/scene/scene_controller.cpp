// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "scene_controller.h"

#include "scene/commands/create_scene_command.h"
#include "scene/commands/create_scene_command_handler.h"
#include "scene/commands/remove_scene_command.h"
#include "scene/commands/remove_scene_command_handler.h"
#include "scene/commands/update_scene_command.h"
#include "scene/commands/update_scene_command_handler.h"
#include "scene/queries/get_scene_query_handler.h"
#include "scene/queries/get_scene_with_details_query_handler.h"
// #include "scene/commands/insert_scene_into_xxx_command.h"
// #include "scene/commands/update_scene_into_xxx_command_handler.h"
#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::Scene;
using namespace Skribisto::Application::Features::Scene::Commands;
using namespace Skribisto::Application::Features::Scene::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<SceneController> SceneController::s_instance = nullptr;

SceneController::SceneController(InterfaceRepositoryProvider *repositoryProvider,
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

SceneController *SceneController::instance()
{
    return s_instance.data();
}

QCoro::Task<SceneDTO> SceneController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetSceneQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));
        GetSceneQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->scene()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "scene");

    // async wait for result signal
    const std::optional<SceneDTO> optional_result =
        co_await qCoro(m_eventDispatcher->scene(), &SceneSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return SceneDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<SceneWithDetailsDTO> SceneController::getWithDetails(int id) const
{
    auto queryCommand = new QueryCommand("getWithDetails");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetSceneQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));
        GetSceneWithDetailsQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->scene()->getWithDetailsReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "scene");

    // async wait for result signal
    const std::optional<SceneWithDetailsDTO> optional_result = co_await qCoro(
        m_eventDispatcher.get()->scene(), &SceneSignals::getWithDetailsReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return SceneWithDetailsDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<SceneDTO> SceneController::create(const CreateSceneDTO &dto)
{
    CreateSceneCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto *handler = new CreateSceneCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateSceneCommandHandler::sceneCreated, m_eventDispatcher->scene(),
                     &SceneSignals::created);

    QObject::connect(handler, &CreateSceneCommandHandler::relationWithOwnerInserted, this,
                     [this](int id, int ownerId, int position) {
                         auto dto = ChapterRelationDTO(ownerId, ChapterRelationDTO::RelationField::Scenes,
                                                       QList<int>() << id, position);
                         emit m_eventDispatcher->chapter()->relationInserted(dto);
                     });
    QObject::connect(handler, &CreateSceneCommandHandler::relationWithOwnerRemoved, this, [this](int id, int ownerId) {
        auto dto = ChapterRelationDTO(ownerId, ChapterRelationDTO::RelationField::Scenes, QList<int>() << id, -1);
        emit m_eventDispatcher->chapter()->relationRemoved(dto);
    });

    QObject::connect(handler, &CreateSceneCommandHandler::sceneRemoved, this,
                     [this](int removedId) { emit m_eventDispatcher->scene()->removed(QList<int>() << removedId); });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateSceneCommandHandler, CreateSceneCommand>(SceneController::tr("Create scene"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "scene");

    // async wait for result signal
    const std::optional<SceneDTO> optional_result =
        co_await qCoro(handler, &CreateSceneCommandHandler::sceneCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return SceneDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<SceneDTO> SceneController::update(const UpdateSceneDTO &dto)
{
    UpdateSceneCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto *handler = new UpdateSceneCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateSceneCommandHandler::sceneUpdated, this,
                     [this](SceneDTO dto) { emit m_eventDispatcher->scene()->updated(dto); });
    QObject::connect(handler, &UpdateSceneCommandHandler::sceneDetailsUpdated, m_eventDispatcher->scene(),
                     &SceneSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateSceneCommandHandler, UpdateSceneCommand>(SceneController::tr("Update scene"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "scene");

    // async wait for result signal
    const std::optional<SceneDTO> optional_result =
        co_await qCoro(handler, &UpdateSceneCommandHandler::sceneUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return SceneDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> SceneController::remove(int id)
{
    RemoveSceneCommand query;

    query.id = id;

    auto repository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));

    auto *handler = new RemoveSceneCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveSceneCommandHandler, RemoveSceneCommand>(SceneController::tr("Remove scene"),
                                                                                   handler, query);

    // push command
    m_undo_redo_system->push(command, "scene");

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateSceneDTO SceneController::getCreateDTO()
{
    return CreateSceneDTO();
}

UpdateSceneDTO SceneController::getUpdateDTO()
{
    return UpdateSceneDTO();
}

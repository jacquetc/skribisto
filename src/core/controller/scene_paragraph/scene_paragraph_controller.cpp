// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "scene_paragraph_controller.h"

#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include "scene_paragraph/commands/create_scene_paragraph_command.h"
#include "scene_paragraph/commands/create_scene_paragraph_command_handler.h"
#include "scene_paragraph/commands/remove_scene_paragraph_command.h"
#include "scene_paragraph/commands/remove_scene_paragraph_command_handler.h"
#include "scene_paragraph/commands/update_scene_paragraph_command.h"
#include "scene_paragraph/commands/update_scene_paragraph_command_handler.h"
#include "scene_paragraph/queries/get_scene_paragraph_query_handler.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::SceneParagraph;
using namespace Skribisto::Application::Features::SceneParagraph::Commands;
using namespace Skribisto::Application::Features::SceneParagraph::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<SceneParagraphController> SceneParagraphController::s_instance = nullptr;

SceneParagraphController::SceneParagraphController(InterfaceRepositoryProvider *repositoryProvider,
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

SceneParagraphController *SceneParagraphController::instance()
{
    return s_instance.data();
}

QCoro::Task<SceneParagraphDTO> SceneParagraphController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetSceneParagraphQuery query;
        query.id = id;
        auto interface =
            static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));
        GetSceneParagraphQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->sceneParagraph()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "sceneParagraph");

    // async wait for result signal
    const std::optional<SceneParagraphDTO> optional_result = co_await qCoro(
        m_eventDispatcher->sceneParagraph(), &SceneParagraphSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return SceneParagraphDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<SceneParagraphDTO> SceneParagraphController::create(const CreateSceneParagraphDTO &dto)
{
    CreateSceneParagraphCommand query;

    query.req = dto;

    auto repository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new CreateSceneParagraphCommandHandler(repository);

    // connect
    QObject::connect(handler, &CreateSceneParagraphCommandHandler::sceneParagraphCreated,
                     m_eventDispatcher->sceneParagraph(), &SceneParagraphSignals::created);

    QObject::connect(handler, &CreateSceneParagraphCommandHandler::relationWithOwnerInserted, this,
                     [this](int id, int ownerId, int position) {
                         auto dto = SceneRelationDTO(ownerId, SceneRelationDTO::RelationField::Paragraphs,
                                                     QList<int>() << id, position);
                         emit m_eventDispatcher->scene()->relationInserted(dto);
                     });
    QObject::connect(
        handler, &CreateSceneParagraphCommandHandler::relationWithOwnerRemoved, this, [this](int id, int ownerId) {
            auto dto = SceneRelationDTO(ownerId, SceneRelationDTO::RelationField::Paragraphs, QList<int>() << id, -1);
            emit m_eventDispatcher->scene()->relationRemoved(dto);
        });

    QObject::connect(handler, &CreateSceneParagraphCommandHandler::sceneParagraphRemoved, this, [this](int removedId) {
        emit m_eventDispatcher->sceneParagraph()->removed(QList<int>() << removedId);
    });

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CreateSceneParagraphCommandHandler, CreateSceneParagraphCommand>(
        SceneParagraphController::tr("Create sceneParagraph"), handler, query);

    // push command
    m_undo_redo_system->push(command, "sceneParagraph");

    // async wait for result signal
    const std::optional<SceneParagraphDTO> optional_result = co_await qCoro(
        handler, &CreateSceneParagraphCommandHandler::sceneParagraphCreated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return SceneParagraphDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<SceneParagraphDTO> SceneParagraphController::update(const UpdateSceneParagraphDTO &dto)
{
    UpdateSceneParagraphCommand query;

    query.req = dto;

    auto repository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new UpdateSceneParagraphCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateSceneParagraphCommandHandler::sceneParagraphUpdated, this,
                     [this](SceneParagraphDTO dto) { emit m_eventDispatcher->sceneParagraph()->updated(dto); });
    QObject::connect(handler, &UpdateSceneParagraphCommandHandler::sceneParagraphDetailsUpdated,
                     m_eventDispatcher->sceneParagraph(), &SceneParagraphSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateSceneParagraphCommandHandler, UpdateSceneParagraphCommand>(
        SceneParagraphController::tr("Update sceneParagraph"), handler, query);

    // push command
    m_undo_redo_system->push(command, "sceneParagraph");

    // async wait for result signal
    const std::optional<SceneParagraphDTO> optional_result = co_await qCoro(
        handler, &UpdateSceneParagraphCommandHandler::sceneParagraphUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return SceneParagraphDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<bool> SceneParagraphController::remove(int id)
{
    RemoveSceneParagraphCommand query;

    query.id = id;

    auto repository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new RemoveSceneParagraphCommandHandler(repository);

    // connect
    // no need to connect to removed signal, because it will be emitted by the repository itself

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<RemoveSceneParagraphCommandHandler, RemoveSceneParagraphCommand>(
        SceneParagraphController::tr("Remove sceneParagraph"), handler, query);

    // push command
    m_undo_redo_system->push(command, "sceneParagraph");

    // async wait for result signal
    const std::optional<QList<int>> optional_result =
        co_await qCoro(repository->signalHolder(), &SignalHolder::removed, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return false;
    }

    co_return true;
}

CreateSceneParagraphDTO SceneParagraphController::getCreateDTO()
{
    return CreateSceneParagraphDTO();
}

UpdateSceneParagraphDTO SceneParagraphController::getUpdateDTO()
{
    return UpdateSceneParagraphDTO();
}

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "user_controller.h"

#include "qleany/tools/undo_redo/alter_command.h"
#include "qleany/tools/undo_redo/query_command.h"
#include "user/commands/update_user_command.h"
#include "user/commands/update_user_command_handler.h"
#include "user/queries/get_user_query_handler.h"
#include <QCoroSignal>

using namespace Skribisto::Controller;
using namespace Skribisto::Controller::User;
using namespace Skribisto::Application::Features::User::Commands;
using namespace Skribisto::Application::Features::User::Queries;
using namespace Qleany::Tools::UndoRedo;
using namespace Qleany::Contracts::Repository;

QPointer<UserController> UserController::s_instance = nullptr;

UserController::UserController(InterfaceRepositoryProvider *repositoryProvider,
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

UserController *UserController::instance()
{
    return s_instance.data();
}

QCoro::Task<UserDTO> UserController::get(int id) const
{
    auto queryCommand = new QueryCommand("get");

    queryCommand->setQueryFunction([this, id](QPromise<Result<void>> &progressPromise) {
        GetUserQuery query;
        query.id = id;
        auto interface = static_cast<InterfaceUserRepository *>(m_repositoryProvider->repository("User"));
        GetUserQueryHandler handler(interface);
        auto result = handler.handle(progressPromise, query);

        if (result.isSuccess())
        {
            emit m_eventDispatcher->user()->getReplied(result.value());
        }
        return Result<void>(result.error());
    });

    m_undo_redo_system->push(queryCommand, "user");

    // async wait for result signal
    const std::optional<UserDTO> optional_result =
        co_await qCoro(m_eventDispatcher->user(), &UserSignals::getReplied, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        // for now, I insert one invalid item to the list to show that there was an error
        co_return UserDTO();
    }

    co_return optional_result.value();
}

QCoro::Task<UserDTO> UserController::update(const UpdateUserDTO &dto)
{
    UpdateUserCommand query;

    query.req = dto;

    auto repository = static_cast<InterfaceUserRepository *>(m_repositoryProvider->repository("User"));

    auto *handler = new UpdateUserCommandHandler(repository);

    // connect
    QObject::connect(handler, &UpdateUserCommandHandler::userUpdated, this,
                     [this](UserDTO dto) { emit m_eventDispatcher->user()->updated(dto); });
    QObject::connect(handler, &UpdateUserCommandHandler::userDetailsUpdated, m_eventDispatcher->user(),
                     &UserSignals::allRelationsInvalidated);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateUserCommandHandler, UpdateUserCommand>(UserController::tr("Update user"),
                                                                                 handler, query);

    // push command
    m_undo_redo_system->push(command, "user");

    // async wait for result signal
    const std::optional<UserDTO> optional_result =
        co_await qCoro(handler, &UpdateUserCommandHandler::userUpdated, std::chrono::milliseconds(1000));

    if (!optional_result.has_value())
    {
        co_return UserDTO();
    }

    co_return optional_result.value();
}

UpdateUserDTO UserController::getUpdateDTO()
{
    return UpdateUserDTO();
}

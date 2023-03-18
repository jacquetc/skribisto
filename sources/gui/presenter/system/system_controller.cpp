#include "system_controller.h"
#include "cqrs/system/commands/load_system_command.h"
#include "features/system/handlers/commands/load_system_command_handler.h"
#include "skrib/skrib_loader.h"
#include "undo_redo/alter_command.h"

using namespace Presenter::System;
using namespace Presenter::UndoRedo;
using namespace Contracts::CQRS::System::Commands;
using namespace Infrastructure::Skrib;
using namespace Application::Features::System::Commands;

QScopedPointer<SystemController> SystemController::s_instance = QScopedPointer<SystemController>(nullptr);
InterfaceRepositoryProvider *SystemController::s_repositoryProvider;
ThreadedUndoRedoSystem *SystemController::s_undo_redo_system;

SystemController::SystemController(InterfaceRepositoryProvider *repositoryProvider) : QObject(nullptr)
{
    s_repositoryProvider = repositoryProvider;
    s_undo_redo_system = ThreadedUndoRedoSystem::instance();

    s_instance.reset(this);
}

SystemController *SystemController::instance()
{
    return s_instance.data();
}

void SystemController::loadSystem(const LoadSystemDTO &dto)
{
    LoadSystemCommand request;
    request.req = dto;
    auto *skribLoader = new SkribLoader(s_repositoryProvider);
    auto *handler = new LoadSystemCommandHandler(skribLoader);

    // connect
    QObject::connect(handler, &LoadSystemCommandHandler::systemLoaded, SystemController::instance(),
                     &SystemController::systemLoaded);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<LoadSystemCommandHandler, LoadSystemCommand>(SystemController::tr("Remove author"),
                                                                                 handler, request);

    // push command
    s_undo_redo_system->push(command, UndoRedoCommand::Scope::All);
    s_undo_redo_system->clear();
}

void SystemController::saveSystem()
{
    Q_UNIMPLEMENTED();
}

void System::SystemController::saveSystemAs(const SaveSystemAsDTO &dto)
{
    Q_UNIMPLEMENTED();
    //    SaveSystemAsCommand request;
    //    request.req = dto;
}

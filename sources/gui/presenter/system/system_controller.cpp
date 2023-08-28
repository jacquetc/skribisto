#include "system_controller.h"
#include "event_dispatcher.h"
#include "skrib/skrib_loader.h"
#include "system/commands/close_system_command.h"
#include "system/commands/close_system_command_handler.h"
#include "system/commands/load_system_command.h"
#include "system/commands/load_system_command_handler.h"
#include "undo_redo/alter_command.h"

using namespace Presenter::System;
using namespace Presenter::UndoRedo;
using namespace Contracts::CQRS::System::Commands;
using namespace Application::Features::System::Commands;

QScopedPointer<SystemController> SystemController::s_instance = QScopedPointer<SystemController>(nullptr);

SystemController::SystemController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                   ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher)
    : QObject(parent)
{
    m_repositoryProvider = repositoryProvider;
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

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
    auto *skribLoader = new Infrastructure::Skrib::SkribLoader(m_repositoryProvider);
    auto *handler = new LoadSystemCommandHandler(skribLoader);

    // connect
    QObject::connect(handler, &LoadSystemCommandHandler::systemLoaded, SystemController::instance(),
                     &SystemController::systemLoaded);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<LoadSystemCommandHandler, LoadSystemCommand>(SystemController::tr("Load system"),
                                                                                 handler, request);

    connect(command, &UndoRedoCommand::progressFinished, SystemController::instance(),
            &SystemController::loadSystemProgressFinished);
    connect(command, &UndoRedoCommand::progressRangeChanged, SystemController::instance(),
            &SystemController::loadSystemProgressRangeChanged);
    connect(command, &UndoRedoCommand::progressTextChanged, SystemController::instance(),
            &SystemController::loadSystemProgressTextChanged);
    connect(command, &UndoRedoCommand::progressValueChanged, SystemController::instance(),
            &SystemController::loadSystemProgressValueChanged);

    // push command
    m_undo_redo_system->push(command, "all");
    m_undo_redo_system->clear();
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

void SystemController::closeSystem()
{
    CloseSystemCommand request;
    auto *handler = new CloseSystemCommandHandler(/*s_repositoryProvider*/);

    // connect
    QObject::connect(handler, &CloseSystemCommandHandler::systemClosed, SystemController::instance(),
                     &SystemController::systemClosed);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<CloseSystemCommandHandler, CloseSystemCommand>(SystemController::tr("Close system"),
                                                                                   handler, request);

    // push command
    m_undo_redo_system->push(command, "all");
    m_undo_redo_system->clear();
}

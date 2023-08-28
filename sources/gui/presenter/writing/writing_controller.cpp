#include "writing_controller.h"
#include "undo_redo/alter_command.h"
#include "writing/commands/update_scene_paragraph_command.h"
#include "writing/commands/update_scene_paragraph_command_handler.h"

using namespace Presenter::Writing;
using namespace Presenter::UndoRedo;
using namespace Contracts::CQRS::Writing::Commands;
using namespace Application::Features::Writing::Commands;

QScopedPointer<WritingController> WritingController::s_instance = QScopedPointer<WritingController>(nullptr);

WritingController::WritingController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                                     ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher)
    : QObject(parent)
{
    m_repositoryProvider = repositoryProvider;
    m_undo_redo_system = ThreadedUndoRedoSystem::instance();
    m_eventDispatcher = eventDispatcher;

    s_instance.reset(this);
}

WritingController *WritingController::instance()
{
    return s_instance.data();
}

void WritingController::updateSceneParagraph(const UpdateSceneParagraphDTO &dto)
{
    UpdateSceneParagraphCommand request;

    request.req = dto;

    auto sceneRepository = static_cast<InterfaceSceneRepository *>(m_repositoryProvider->repository("Scene"));
    auto sceneParagraphRepository =
        static_cast<InterfaceSceneParagraphRepository *>(m_repositoryProvider->repository("SceneParagraph"));

    auto *handler = new UpdateSceneParagraphCommandHandler(sceneRepository, sceneParagraphRepository);

    // connect
    QObject::connect(handler, &UpdateSceneParagraphCommandHandler::sceneParagraphChanged, WritingController::instance(),
                     &WritingController::sceneParagraphChanged);

    // Create specialized UndoRedoCommand
    auto command = new AlterCommand<UpdateSceneParagraphCommandHandler, UpdateSceneParagraphCommand>(
        WritingController::tr("Update scene paragraph"), handler, request);

    // push command
    m_undo_redo_system->push(command, "scene", dto.sceneUuid());
}

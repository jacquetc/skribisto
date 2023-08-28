#include "presenter_registration.h"
#include "author/author_controller.h"
#include "chapter/chapter_controller.h"
#include "event_dispatcher.h"
#include "system/system_controller.h"
#include "undo_redo/threaded_undo_redo_system.h"
#include "writing/writing_controller.h"

using namespace Presenter;
PresenterRegistration::PresenterRegistration(QObject *parent, InterfaceRepositoryProvider *repositoryProvider)
    : QObject{parent}
{
    // Event Dispatcher for entities
    auto *eventDispatcher = new EventDispatcher(nullptr);

    // Undo Redo System
    Scopes scopes(QStringList() << "author"
                                << "chapter"
                                << "scene");
    auto *undoRedoSystem = new UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    // initialize controllers
    new System::SystemController(nullptr, repositoryProvider, undoRedoSystem, eventDispatcher);
    new Author::AuthorController(nullptr, repositoryProvider, undoRedoSystem, eventDispatcher);
    new Chapter::ChapterController(nullptr, repositoryProvider, undoRedoSystem, eventDispatcher);
    new Writing::WritingController(nullptr, repositoryProvider, undoRedoSystem, eventDispatcher);
}

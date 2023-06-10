#include "presenter_registration.h"
#include "author/author_controller.h"
#include "chapter/chapter_controller.h"
#include "system/system_controller.h"

using namespace Presenter;
PresenterRegistration::PresenterRegistration(QObject                     *parent,
                                             InterfaceRepositoryProvider *repositoryProvider) : QObject{parent}
{
    Scopes scopes(QStringList() << "author"
                                << "chapter");
    auto *undoRedoSystem = new UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    s_undoRedoSystem.reset(undoRedoSystem);

    // initialize controllers
    new System::SystemController(repositoryProvider);
    new Author::AuthorController(repositoryProvider);
    new Chapter::ChapterController(repositoryProvider);
}

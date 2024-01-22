// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#include "controller_registration.h"
#include "atelier/atelier_controller.h"
#include "book/book_controller.h"
#include "chapter/chapter_controller.h"
#include "event_dispatcher.h"
#include "scene/scene_controller.h"
#include "scene_paragraph/scene_paragraph_controller.h"
#include "system/system_controller.h"
#include "user/user_controller.h"
#include "workspace/workspace_controller.h"
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>
#include <qleany/tools/undo_redo/undo_redo_scopes.h>

using namespace Skribisto::Controller;

ControllerRegistration::ControllerRegistration(QObject *parent, InterfaceRepositoryProvider *repositoryProvider)
    : QObject{parent}
{

    auto dispatcher = QSharedPointer<EventDispatcher>(new EventDispatcher());

    // Undo Redo System
    Scopes scopes(QStringList() << "user"
                                << "book"
                                << "workspace"
                                << "atelier"
                                << "chapter"
                                << "scene"
                                << "sceneParagraph"
                                << "system");
    auto *undoRedoSystem = new Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem(this, scopes);

    // error handling
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::errorSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Error in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                emit dispatcher->error()->errorSent(error);
            });
    connect(undoRedoSystem, &Qleany::Tools::UndoRedo::ThreadedUndoRedoSystem::warningSent, this,
            [dispatcher](Qleany::Error error) {
                qDebug() << "Warning in undo redo system: " << error.status() << error.code() << error.message()
                         << error.data() << error.stackTrace();
                emit dispatcher->error()->warningSent(error);
            });

    // UserController

    new User::UserController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *userSignalHolder = repositoryProvider->repository("User")->signalHolder();

    // removal
    connect(userSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->user(),
            &UserSignals::removed);

    // active status
    connect(repositoryProvider->repository("user")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->user(),
            &UserSignals::activeStatusChanged);

    // BookController

    new Book::BookController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *bookSignalHolder = repositoryProvider->repository("Book")->signalHolder();

    // removal
    connect(bookSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->book(),
            &BookSignals::removed);

    // active status
    connect(repositoryProvider->repository("book")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->book(),
            &BookSignals::activeStatusChanged);

    // WorkspaceController

    new Workspace::WorkspaceController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *workspaceSignalHolder = repositoryProvider->repository("Workspace")->signalHolder();

    // removal
    connect(workspaceSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->workspace(),
            &WorkspaceSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(workspaceSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                AtelierRelationDTO dto(-1, AtelierRelationDTO::RelationField::Workspaces, removedIds, -1);
                emit dispatcher->atelier()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("workspace")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->workspace(),
            &WorkspaceSignals::activeStatusChanged);

    // AtelierController

    new Atelier::AtelierController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *atelierSignalHolder = repositoryProvider->repository("Atelier")->signalHolder();

    // removal
    connect(atelierSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->atelier(),
            &AtelierSignals::removed);

    // active status
    connect(repositoryProvider->repository("atelier")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->atelier(),
            &AtelierSignals::activeStatusChanged);

    // ChapterController

    new Chapter::ChapterController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *chapterSignalHolder = repositoryProvider->repository("Chapter")->signalHolder();

    // removal
    connect(chapterSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->chapter(),
            &ChapterSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(chapterSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                BookRelationDTO dto(-1, BookRelationDTO::RelationField::Chapters, removedIds, -1);
                emit dispatcher->book()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("chapter")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->chapter(),
            &ChapterSignals::activeStatusChanged);

    // SceneController

    new Scene::SceneController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *sceneSignalHolder = repositoryProvider->repository("Scene")->signalHolder();

    // removal
    connect(sceneSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, dispatcher->scene(),
            &SceneSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(sceneSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                ChapterRelationDTO dto(-1, ChapterRelationDTO::RelationField::Scenes, removedIds, -1);
                emit dispatcher->chapter()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("scene")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->scene(),
            &SceneSignals::activeStatusChanged);

    // SceneParagraphController

    new SceneParagraph::SceneParagraphController(repositoryProvider, undoRedoSystem, dispatcher);

    SignalHolder *sceneParagraphSignalHolder = repositoryProvider->repository("SceneParagraph")->signalHolder();

    // removal
    connect(sceneParagraphSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed,
            dispatcher->sceneParagraph(), &SceneParagraphSignals::removed);

    // spread removal signal to all other entity signal holders so as to remove the relations

    connect(sceneParagraphSignalHolder, &Qleany::Contracts::Repository::SignalHolder::removed, this,
            [dispatcher](QList<int> removedIds) {
                SceneRelationDTO dto(-1, SceneRelationDTO::RelationField::Paragraphs, removedIds, -1);
                emit dispatcher->scene()->relationRemoved(dto);
            });

    // active status
    connect(repositoryProvider->repository("sceneParagraph")->signalHolder(),
            &Qleany::Contracts::Repository::SignalHolder::activeStatusChanged, dispatcher->sceneParagraph(),
            &SceneParagraphSignals::activeStatusChanged);

    // SystemController

    new System::SystemController(repositoryProvider, undoRedoSystem, dispatcher);
}

ControllerRegistration::~ControllerRegistration()
{
}
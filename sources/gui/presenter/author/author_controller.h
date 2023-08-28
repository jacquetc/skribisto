#pragma once

#include "author/author_dto.h"
#include "author/create_author_dto.h"
#include "author/update_author_dto.h"
#include "event_dispatcher.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "undo_redo/threaded_undo_redo_system.h"

using namespace Contracts::DTO::Author;
using namespace Presenter;
using namespace Contracts::Persistence;
using namespace Presenter::UndoRedo;

namespace Presenter::Author
{

class SKR_PRESENTER_EXPORT AuthorController : public QObject
{
    Q_OBJECT
  public:
    AuthorController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                     ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher);

    static AuthorController *instance();

    void get(int id);

    void getAll();

    void create(const CreateAuthorDTO &dto);

    void update(const UpdateAuthorDTO &dto);

    void remove(int id);

  signals:
    void getReplied(Contracts::DTO::Author::AuthorDTO dto);
    void getAllReplied(QList<Contracts::DTO::Author::AuthorDTO> dtoList);
    void authorCreated(Contracts::DTO::Author::AuthorDTO dto);
    void authorRemoved(int id);
    void authorUpdated(Contracts::DTO::Author::AuthorDTO dto);

  private:
    static QScopedPointer<AuthorController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    EventDispatcher *m_eventDispatcher;
    AuthorController() = delete;
    AuthorController(const AuthorController &) = delete;
    AuthorController &operator=(const AuthorController &) = delete;
};

} // namespace Presenter::Author

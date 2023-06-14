#pragma once

#include "author/author_dto.h"
#include "author/create_author_dto.h"
#include "author/update_author_dto.h"
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
    AuthorController(InterfaceRepositoryProvider *repositoryProvider);

    static AuthorController *instance();

    static void get(int id);

    static void getAll();

    static void create(const CreateAuthorDTO &dto);

    static void update(const UpdateAuthorDTO &dto);

    static void remove(int id);

  signals:
    void getReplied(Contracts::DTO::Author::AuthorDTO dto);
    void getAllReplied(QList<Contracts::DTO::Author::AuthorDTO> dtoList);
    void authorCreated(Contracts::DTO::Author::AuthorDTO dto);
    void authorRemoved(int id);
    void authorUpdated(Contracts::DTO::Author::AuthorDTO dto);

  private:
    static QScopedPointer<AuthorController> s_instance;
    static InterfaceRepositoryProvider *s_repositoryProvider;
    static ThreadedUndoRedoSystem *s_undo_redo_system;
    AuthorController() = delete;
    AuthorController(const AuthorController &) = delete;
    AuthorController &operator=(const AuthorController &) = delete;
};

} // namespace Presenter::Author

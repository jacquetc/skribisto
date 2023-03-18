#pragma once

#include "dto/author/author_dto.h"
#include "dto/author/create_author_dto.h"
#include "dto/author/update_author_dto.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "result.h"
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

    static void getAsync(const QUuid &uuid);

    static void getAllAsync();

    static void createAsync(const CreateAuthorDTO &dto);

    static void updateAsync(const UpdateAuthorDTO &dto);

    static void removeAsync(const QUuid &uuid);

  signals:
    void getAuthorReplied(Contracts::DTO::Author::AuthorDTO result);
    void getAllReplied(QList<Contracts::DTO::Author::AuthorDTO> result);
    void authorCreated(Contracts::DTO::Author::AuthorDTO result);
    void authorRemoved(Contracts::DTO::Author::AuthorDTO result);
    void authorUpdated(Contracts::DTO::Author::AuthorDTO result);

  private:
    static QScopedPointer<AuthorController> s_instance;
    static InterfaceRepositoryProvider *s_repositoryProvider;
    static ThreadedUndoRedoSystem *s_undo_redo_system;
    AuthorController() = delete;
    AuthorController(const AuthorController &) = delete;
    AuthorController &operator=(const AuthorController &) = delete;
};

} // namespace Presenter::Author

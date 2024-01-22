// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "persistence_registration.h"
#include <qleany/database/database_context.h>
#include <qleany/database/database_table_group.h>

#include "repository/atelier_repository.h"
#include "repository/book_repository.h"
#include "repository/chapter_repository.h"
#include "repository/file_repository.h"
#include "repository/git_repository.h"
#include "repository/recent_atelier_repository.h"
#include "repository/scene_paragraph_repository.h"
#include "repository/scene_repository.h"
#include "repository/user_repository.h"
#include "repository/workspace_repository.h"

using namespace Qleany;
using namespace Qleany::Database;
using namespace Qleany::Repository;
using namespace Skribisto::Persistence;
using namespace Skribisto::Persistence::Repository;

PersistenceRegistration::PersistenceRegistration(QObject *parent) : QObject{parent}
{
    QSharedPointer<DatabaseContext> context(new DatabaseContext());

    // database tables:

    auto *sceneParagraphDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::SceneParagraph>(context);
    auto *sceneDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Scene>(context);
    auto *chapterDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Chapter>(context);
    auto *fileDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::File>(context);
    auto *workspaceDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Workspace>(context);
    auto *gitDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Git>(context);
    auto *bookDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Book>(context);
    auto *atelierDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::Atelier>(context);
    auto *userDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::User>(context);
    auto *recentAtelierDatabaseTableGroup = new DatabaseTableGroup<Skribisto::Domain::RecentAtelier>(context);

    Result<void> initResult = context->init();

    if (initResult.hasError())
    {
        Error error = initResult.error();
        qCritical() << error.className() + "\n" + error.code() + "\n" + error.message() + "\n" + error.data();
    }

    // repositories:

    SceneParagraphRepository *sceneParagraphRepository = new SceneParagraphRepository(sceneParagraphDatabaseTableGroup);
    SceneRepository *sceneRepository = new SceneRepository(sceneDatabaseTableGroup, sceneParagraphRepository);
    ChapterRepository *chapterRepository = new ChapterRepository(chapterDatabaseTableGroup, sceneRepository);
    FileRepository *fileRepository = new FileRepository(fileDatabaseTableGroup);
    WorkspaceRepository *workspaceRepository = new WorkspaceRepository(workspaceDatabaseTableGroup, fileRepository);
    GitRepository *gitRepository = new GitRepository(gitDatabaseTableGroup);
    BookRepository *bookRepository = new BookRepository(bookDatabaseTableGroup, chapterRepository);
    AtelierRepository *atelierRepository =
        new AtelierRepository(atelierDatabaseTableGroup, gitRepository, workspaceRepository);
    UserRepository *userRepository = new UserRepository(userDatabaseTableGroup);
    RecentAtelierRepository *recentAtelierRepository = new RecentAtelierRepository(recentAtelierDatabaseTableGroup);

    // register repositories:

    RepositoryProvider::instance()->registerRepository("sceneParagraph", sceneParagraphRepository);
    RepositoryProvider::instance()->registerRepository("scene", sceneRepository);
    RepositoryProvider::instance()->registerRepository("chapter", chapterRepository);
    RepositoryProvider::instance()->registerRepository("file", fileRepository);
    RepositoryProvider::instance()->registerRepository("workspace", workspaceRepository);
    RepositoryProvider::instance()->registerRepository("git", gitRepository);
    RepositoryProvider::instance()->registerRepository("book", bookRepository);
    RepositoryProvider::instance()->registerRepository("atelier", atelierRepository);
    RepositoryProvider::instance()->registerRepository("user", userRepository);
    RepositoryProvider::instance()->registerRepository("recentAtelier", recentAtelierRepository);
}

RepositoryProvider *PersistenceRegistration::repositoryProvider()
{
    return RepositoryProvider::instance();
}
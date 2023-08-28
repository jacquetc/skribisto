#pragma once

#include "book.h"

#include "persistence/interface_chapter_repository.h"

#include "persistence/interface_author_repository.h"

#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_book_repository.h"
#include "persistence_global.h"

using namespace Contracts::Persistence;

namespace Repository
{

class SKR_PERSISTENCE_EXPORT BookRepository : public Repository::GenericRepository<Domain::Book>,
                                              public Contracts::Persistence::InterfaceBookRepository
{
  public:
    explicit BookRepository(Domain::EntitySchema *entitySchema, InterfaceDatabaseTable<Domain::Book> *bookDatabase,
                            InterfaceChapterRepository *chapterRepository, InterfaceAuthorRepository *authorRepository);

    Domain::Book::ChaptersLoader fetchChaptersLoader() override;

    Domain::Book::AuthorLoader fetchAuthorLoader() override;

    QHash<int, QList<int>> removeInCascade(QList<int> ids) override;

  private:
    Domain::EntitySchema *m_entitySchema;

    InterfaceChapterRepository *m_chapterRepository;

    InterfaceAuthorRepository *m_authorRepository;
};

} // namespace Repository
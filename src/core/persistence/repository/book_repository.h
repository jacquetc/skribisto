// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "book.h"
#include "persistence_export.h"
#include "repository/interface_book_repository.h"
#include "repository/interface_chapter_repository.h"
#include <QReadWriteLock>
#include <qleany/contracts/database/interface_database_table_group.h>
#include <qleany/repository/generic_repository.h>

using namespace Qleany;
using namespace Qleany::Contracts::Repository;
using namespace Skribisto::Contracts::Repository;
using namespace Qleany::Contracts::Database;

namespace Skribisto::Persistence::Repository
{

class SKRIBISTO_PERSISTENCE_EXPORT BookRepository final
    : public Qleany::Repository::GenericRepository<Skribisto::Domain::Book>,
      public Skribisto::Contracts::Repository::InterfaceBookRepository
{
  public:
    explicit BookRepository(InterfaceDatabaseTableGroup<Skribisto::Domain::Book> *bookDatabase,
                            InterfaceChapterRepository *chapterRepository);

    SignalHolder *signalHolder() override;

    Result<Skribisto::Domain::Book> update(Skribisto::Domain::Book &&entity) override;
    Result<Skribisto::Domain::Book> getWithDetails(int entityId) override;

    Skribisto::Domain::Book::ChaptersLoader fetchChaptersLoader() override;

    Result<QHash<int, QList<int>>> removeInCascade(QList<int> ids) override;
    Result<QHash<int, QList<int>>> changeActiveStatusInCascade(QList<int> ids, bool active) override;

  private:
    InterfaceChapterRepository *m_chapterRepository;
    QScopedPointer<SignalHolder> m_signalHolder;
    QReadWriteLock m_lock;
};

} // namespace Skribisto::Persistence::Repository
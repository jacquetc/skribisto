#pragma once

#include "chapter.h"
#include "database/interface_database_table.h"
#include "generic_repository.h"
#include "persistence/interface_chapter_repository.h"
#include "persistence_global.h"
#include <QObject>

namespace Repository {
class SKR_PERSISTENCE_EXPORT ChapterRepository : public QObject,
                                                 public Repository::GenericRepository<Domain::Chapter>,
                                                 public Contracts::Persistence::InterfaceChapterRepository {
    Q_OBJECT
    Q_INTERFACES(Contracts::Persistence::InterfaceChapterRepository)

public:

    explicit ChapterRepository(InterfaceDatabaseTable<Domain::Chapter> *chapterDatabase,
                               InterfaceDatabaseTable<Domain::Scene>   *sceneDatabase);


    Domain::Chapter::ScenesLoader fetchScenesLoader() override;

private:

    InterfaceDatabaseTable<Domain::Chapter> *m_chapterDatabase;

    InterfaceDatabaseTable<Domain::Scene> *m_sceneDatabase;
};
} // namespace Repository

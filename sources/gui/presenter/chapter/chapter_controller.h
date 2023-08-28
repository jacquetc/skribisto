#pragma once

#include "chapter/chapter_dto.h"
#include "chapter/chapter_with_details_dto.h"
#include "chapter/create_chapter_dto.h"
#include "chapter/update_chapter_dto.h"
#include "event_dispatcher.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "undo_redo/threaded_undo_redo_system.h"

using namespace Contracts::DTO::Chapter;
using namespace Presenter;
using namespace Contracts::Persistence;
using namespace Presenter::UndoRedo;

namespace Presenter::Chapter
{
class SKR_PRESENTER_EXPORT ChapterController : public QObject
{
    Q_OBJECT

  public:
    ChapterController(QObject *parent, InterfaceRepositoryProvider *repositoryProvider,
                      ThreadedUndoRedoSystem *undo_redo_system, EventDispatcher *eventDispatcher);

    static ChapterController *instance();

  public slots:

    void get(int id);

    void getWithDetails(int id);

    void getAll();

    void create(const CreateChapterDTO &dto);

    void update(const UpdateChapterDTO &dto);

    void remove(int id);

    static Contracts::DTO::Chapter::CreateChapterDTO getCreateChapterDTO();

  signals:

    void getReplied(Contracts::DTO::Chapter::ChapterDTO dto);
    void getWithDetailsReplied(Contracts::DTO::Chapter::ChapterWithDetailsDTO dto);
    void getAllReplied(QList<Contracts::DTO::Chapter::ChapterDTO> dtoList);
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO dto);
    void chapterRemoved(int id);
    void chapterUpdated(Contracts::DTO::Chapter::ChapterDTO dto);

  private:
    static QScopedPointer<ChapterController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    EventDispatcher *m_eventDispatcher;
    ChapterController() = delete;
    ChapterController(const ChapterController &) = delete;
    ChapterController &operator=(const ChapterController &) = delete;
};
} // namespace Presenter::Chapter

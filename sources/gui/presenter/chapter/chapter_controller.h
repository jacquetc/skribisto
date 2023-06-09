#pragma once

#include "QtQml/qjsvalue.h"
#include "chapter_dto.h"
#include "create_chapter_dto.h"
#include "persistence/interface_repository_provider.h"
#include "presenter_global.h"
#include "undo_redo/threaded_undo_redo_system.h"
#include "update_chapter_dto.h"

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
    ChapterController(InterfaceRepositoryProvider *repositoryProvider);

    static ChapterController *instance();

  public slots:

    static void get(int id);

    static void getAll();

    static void create(const CreateChapterDTO &dto);

    static void update(const UpdateChapterDTO &dto);

    static void remove(int id);

    static Contracts::DTO::Chapter::CreateChapterDTO getCreateChapterDTO();

  signals:
    void getReplied(Contracts::DTO::Chapter::ChapterDTO dto);
    void getAllReplied(QList<Contracts::DTO::Chapter::ChapterDTO> dtoList);
    void chapterCreated(Contracts::DTO::Chapter::ChapterDTO dto);
    void chapterRemoved(Contracts::DTO::Chapter::ChapterDTO dto);
    void chapterUpdated(Contracts::DTO::Chapter::ChapterDTO dto);

  private:
    static QScopedPointer<ChapterController> s_instance;
    static InterfaceRepositoryProvider *s_repositoryProvider;
    static ThreadedUndoRedoSystem *s_undo_redo_system;
    ChapterController() = delete;
    ChapterController(const ChapterController &) = delete;
    ChapterController &operator=(const ChapterController &) = delete;
};

} // namespace Presenter::Chapter

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter/chapter_dto.h"
#include "chapter/chapter_with_details_dto.h"
#include "chapter/create_chapter_dto.h"
#include "chapter/update_chapter_dto.h"
#include "controller_export.h"
#include "event_dispatcher.h"
#include <qleany/contracts/repository/interface_repository_provider.h>

#include <QCoroTask>
#include <QObject>
#include <QPointer>
#include <QSharedPointer>
#include <qleany/tools/undo_redo/threaded_undo_redo_system.h>

using namespace Qleany::Contracts::Repository;
using namespace Qleany::Tools::UndoRedo;
using namespace Skribisto::Contracts::DTO::Chapter;

namespace Skribisto::Controller::Chapter
{

class SKRIBISTO_CONTROLLER_EXPORT ChapterController : public QObject
{
    Q_OBJECT
  public:
    explicit ChapterController(InterfaceRepositoryProvider *repositoryProvider,
                               ThreadedUndoRedoSystem *undo_redo_system,
                               QSharedPointer<EventDispatcher> eventDispatcher);

    static ChapterController *instance();

    Q_INVOKABLE QCoro::Task<ChapterDTO> get(int id) const;

    Q_INVOKABLE QCoro::Task<ChapterWithDetailsDTO> getWithDetails(int id) const;

    Q_INVOKABLE static Contracts::DTO::Chapter::CreateChapterDTO getCreateDTO();

    Q_INVOKABLE static Contracts::DTO::Chapter::UpdateChapterDTO getUpdateDTO();

  public slots:

    QCoro::Task<ChapterDTO> create(const CreateChapterDTO &dto);

    QCoro::Task<ChapterDTO> update(const UpdateChapterDTO &dto);

    QCoro::Task<bool> remove(int id);

  private:
    static QPointer<ChapterController> s_instance;
    InterfaceRepositoryProvider *m_repositoryProvider;
    ThreadedUndoRedoSystem *m_undo_redo_system;
    QSharedPointer<EventDispatcher> m_eventDispatcher;
    ChapterController() = delete;
    ChapterController(const ChapterController &) = delete;
    ChapterController &operator=(const ChapterController &) = delete;
};

} // namespace Skribisto::Controller::Chapter
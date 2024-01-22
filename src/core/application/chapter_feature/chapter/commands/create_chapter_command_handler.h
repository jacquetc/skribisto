// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_chapter_export.h"
#include "chapter/chapter_dto.h"
#include "chapter/commands/create_chapter_command.h"
#include "repository/interface_chapter_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::Chapter;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Chapter::Commands;

namespace Skribisto::Application::Features::Chapter::Commands
{
class SKRIBISTO_APPLICATION_CHAPTER_EXPORT CreateChapterCommandHandler : public QObject
{
    Q_OBJECT
  public:
    CreateChapterCommandHandler(InterfaceChapterRepository *repository);

    Result<ChapterDTO> handle(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restore();

  signals:
    void chapterCreated(Skribisto::Contracts::DTO::Chapter::ChapterDTO chapterDto);
    void chapterRemoved(int id);

    void relationWithOwnerInserted(int id, int ownerId, int position);
    void relationWithOwnerRemoved(int id, int ownerId);

  private:
    InterfaceChapterRepository *m_repository;
    Result<ChapterDTO> handleImpl(QPromise<Result<void>> &progressPromise, const CreateChapterCommand &request);
    Result<ChapterDTO> restoreImpl();
    Result<Skribisto::Domain::Chapter> m_newEntity;

    int m_ownerId = -1;
    int m_position = -1;

    QList<Skribisto::Domain::Chapter> m_oldOwnerChapters;
    QList<Skribisto::Domain::Chapter> m_ownerChaptersNewState;

    static bool s_mappingRegistered;
    void registerMappings();
    bool m_firstPass = true;
};

} // namespace Skribisto::Application::Features::Chapter::Commands
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_system_export.h"

#include "system/commands/save_atelier_command.h"

#include "repository/interface_atelier_repository.h"
#include "repository/interface_book_repository.h"
#include "repository/interface_chapter_repository.h"
#include "repository/interface_file_repository.h"
#include "repository/interface_git_repository.h"
#include "repository/interface_recent_atelier_repository.h"
#include "repository/interface_scene_paragraph_repository.h"
#include "repository/interface_scene_repository.h"
#include "repository/interface_workspace_repository.h"
#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::CQRS::System::Commands;

namespace Skribisto::Application::Features::System::Commands
{
class SKRIBISTO_APPLICATION_SYSTEM_EXPORT SaveAtelierCommandHandler : public QObject
{
    Q_OBJECT
  public:
    SaveAtelierCommandHandler(InterfaceRecentAtelierRepository *recentAtelierRepository,
                              InterfaceAtelierRepository *atelierRepository, InterfaceGitRepository *gitRepository,
                              InterfaceWorkspaceRepository *workspaceRepository,
                              InterfaceFileRepository *fileRepository, InterfaceBookRepository *bookRepository,
                              InterfaceChapterRepository *chapterRepository, InterfaceSceneRepository *sceneRepository,
                              InterfaceSceneParagraphRepository *sceneParagraphRepository);

    Result<void> handle(QPromise<Result<void>> &progressPromise, const SaveAtelierCommand &request);

    Result<void> restore();

  signals:

    void saveAtelierChanged();

  private:
    InterfaceRecentAtelierRepository *m_recentAtelierRepository;
    InterfaceAtelierRepository *m_atelierRepository;
    InterfaceGitRepository *m_gitRepository;
    InterfaceWorkspaceRepository *m_workspaceRepository;
    InterfaceFileRepository *m_fileRepository;
    InterfaceBookRepository *m_bookRepository;
    InterfaceChapterRepository *m_chapterRepository;
    InterfaceSceneRepository *m_sceneRepository;
    InterfaceSceneParagraphRepository *m_sceneParagraphRepository;
    Result<void> handleImpl(QPromise<Result<void>> &progressPromise, const SaveAtelierCommand &request);

    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::System::Commands
// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "system/load_atelier_dto.h"


#include "repository/interface_recent_atelier_repository.h"

#include "repository/interface_atelier_repository.h"

#include "repository/interface_git_repository.h"

#include "repository/interface_workspace_repository.h"

#include "repository/interface_file_repository.h"

#include "repository/interface_book_repository.h"

#include "repository/interface_chapter_repository.h"

#include "repository/interface_scene_repository.h"

#include "repository/interface_scene_paragraph_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::System;

namespace Skribisto::Contracts::CQRS::System::Validators
{
class LoadAtelierCommandValidator
{
  public:
    LoadAtelierCommandValidator(InterfaceRecentAtelierRepository *recentAtelierRepository,InterfaceAtelierRepository *atelierRepository,InterfaceGitRepository *gitRepository,InterfaceWorkspaceRepository *workspaceRepository,InterfaceFileRepository *fileRepository,InterfaceBookRepository *bookRepository,InterfaceChapterRepository *chapterRepository,InterfaceSceneRepository *sceneRepository,InterfaceSceneParagraphRepository *sceneParagraphRepository)
        :  m_recentAtelierRepository(recentAtelierRepository), m_atelierRepository(atelierRepository), m_gitRepository(gitRepository), m_workspaceRepository(workspaceRepository), m_fileRepository(fileRepository), m_bookRepository(bookRepository), m_chapterRepository(chapterRepository), m_sceneRepository(sceneRepository), m_sceneParagraphRepository(sceneParagraphRepository)
    {
    }

    Result<void> validate(const LoadAtelierDTO &dto) const

    {





        // Return that is Ok :
        return Result<void>();
    }

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

};
} // namespace Skribisto::Contracts::CQRS::System::Validators
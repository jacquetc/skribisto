// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "load_atelier_command_handler.h"
#include "system/load_atelier_dto.h"
#include "system/validators/load_atelier_command_validator.h"
#include <QDebug>
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::System;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::System::Validators;
using namespace Skribisto::Application::Features::System::Commands;

LoadAtelierCommandHandler::LoadAtelierCommandHandler(
    InterfaceRecentAtelierRepository *recentAtelierRepository, InterfaceAtelierRepository *atelierRepository,
    InterfaceGitRepository *gitRepository, InterfaceWorkspaceRepository *workspaceRepository,
    InterfaceFileRepository *fileRepository, InterfaceBookRepository *bookRepository,
    InterfaceChapterRepository *chapterRepository, InterfaceSceneRepository *sceneRepository,
    InterfaceSceneParagraphRepository *sceneParagraphRepository)
    : m_recentAtelierRepository(recentAtelierRepository), m_atelierRepository(atelierRepository),
      m_gitRepository(gitRepository), m_workspaceRepository(workspaceRepository), m_fileRepository(fileRepository),
      m_bookRepository(bookRepository), m_chapterRepository(chapterRepository), m_sceneRepository(sceneRepository),
      m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> LoadAtelierCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                               const LoadAtelierCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling LoadAtelierCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<void> LoadAtelierCommandHandler::restore()
{

    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> LoadAtelierCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                   const LoadAtelierCommand &request)
{
    qDebug() << "LoadAtelierCommandHandler::handleImpl called";

    // Validate the create system command using the validator
    auto validator = LoadAtelierCommandValidator(m_recentAtelierRepository, m_atelierRepository, m_gitRepository,
                                                 m_workspaceRepository, m_fileRepository, m_bookRepository,
                                                 m_chapterRepository, m_sceneRepository, m_sceneParagraphRepository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(void, validatorResult);

    // implement logic here which will not be repeated on restore
    // system = Qleany::Tools::AutoMapper::AutoMapper::map<LoadAtelierDTO, Skribisto::Domain::System>(request.req);

    m_recentAtelierRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_recentAtelierRepository->saveChanges();

    // emit signal
    // emit loadAtelierChanged();

    // Return
    return Result<void>();
}

bool LoadAtelierCommandHandler::s_mappingRegistered = false;

void LoadAtelierCommandHandler::registerMappings()
{
}
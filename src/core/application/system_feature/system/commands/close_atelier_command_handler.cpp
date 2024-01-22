// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "close_atelier_command_handler.h"
#include <qleany/tools/automapper/automapper.h>

#include <QDebug>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Application::Features::System::Commands;

CloseAtelierCommandHandler::CloseAtelierCommandHandler(
    InterfaceRecentAtelierRepository *recentAtelierRepository, InterfaceAtelierRepository *atelierRepository,
    InterfaceGitRepository *gitRepository, InterfaceWorkspaceRepository *workspaceRepository,
    InterfaceFileRepository *fileRepository, InterfaceBookRepository *bookRepository,
    InterfaceChapterRepository *chapterRepository, InterfaceSceneRepository *sceneRepository)
    : m_recentAtelierRepository(recentAtelierRepository), m_atelierRepository(atelierRepository),
      m_gitRepository(gitRepository), m_workspaceRepository(workspaceRepository), m_fileRepository(fileRepository),
      m_bookRepository(bookRepository), m_chapterRepository(chapterRepository), m_sceneRepository(sceneRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<void> CloseAtelierCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                const CloseAtelierCommand &request)
{
    Result<void> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<void>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CloseAtelierCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<void> CloseAtelierCommandHandler::restore()
{

    Q_UNREACHABLE();
    return Result<void>();
}

Result<void> CloseAtelierCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                    const CloseAtelierCommand &request)
{
    qDebug() << "CloseAtelierCommandHandler::handleImpl called";

    // implement logic here which will not be repeated on restore
    // system = Qleany::Tools::AutoMapper::AutoMapper::map<void, Skribisto::Domain::System>(request.req);

    m_recentAtelierRepository->beginChanges();

    // play here with the repositories
    Q_UNIMPLEMENTED();

    m_recentAtelierRepository->saveChanges();

    // emit signal
    // emit closeAtelierChanged();

    // Return
    return Result<void>();
}

bool CloseAtelierCommandHandler::s_mappingRegistered = false;

void CloseAtelierCommandHandler::registerMappings()
{
}
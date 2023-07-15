#include "remove_scene_paragraph_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_paragraph_repository.h"

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Commands;
using namespace Application::Features::SceneParagraph::Commands;

RemoveSceneParagraphCommandHandler::RemoveSceneParagraphCommandHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveSceneParagraphCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                       const RemoveSceneParagraphCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneParagraphCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneParagraphCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneParagraphCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                           const RemoveSceneParagraphCommand &request)
{
    Result<Domain::SceneParagraph> sceneParagraphResult = m_repository->get(request.id);

    if (Q_UNLIKELY(sceneParagraphResult.hasError()))
    {
        qDebug() << "Error getting sceneParagraph from repository:" << sceneParagraphResult.error().message();
        return Result<int>(sceneParagraphResult.error());
    }

    // save old entity
    m_oldState = sceneParagraphResult.value();

    int sceneParagraphId;

    auto deleteResult = m_repository->remove(sceneParagraphId);

    if (Q_UNLIKELY(!deleteResult.hasError()))
    {
        qDebug() << "Error deleting sceneParagraph from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit sceneParagraphRemoved(deleteResult.value());

    qDebug() << "SceneParagraph removed:" << sceneParagraphId;

    return Result<int>(sceneParagraphId);
}

Result<int> RemoveSceneParagraphCommandHandler::restoreImpl()
{

    // Add the sceneParagraph to the repository
    auto sceneParagraphResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(sceneParagraphResult.hasError()))
    {
        return Result<int>(sceneParagraphResult.error());
    }

    auto sceneParagraphDTO =
        AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraphResult.value());

    emit sceneParagraphCreated(sceneParagraphDTO);
    qDebug() << "SceneParagraph added:" << sceneParagraphDTO.id();

    // Return the UUID of the newly created sceneParagraph as a Result object
    return Result<int>(0);
}

bool RemoveSceneParagraphCommandHandler::s_mappingRegistered = false;

void RemoveSceneParagraphCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::SceneParagraph, Contracts::DTO::SceneParagraph::SceneParagraphDTO>(
        true);
}
#include "remove_scene_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;
using namespace Application::Features::Scene::Commands;

RemoveSceneCommandHandler::RemoveSceneCommandHandler(QSharedPointer<InterfaceSceneRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveSceneCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                              const RemoveSceneCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneCommand:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                  const RemoveSceneCommand &request)
{
    Result<Domain::Scene> sceneResult = m_repository->get(request.id);

    if (Q_UNLIKELY(sceneResult.hasError()))
    {
        qDebug() << "Error getting scene from repository:" << sceneResult.error().message();
        return Result<int>(sceneResult.error());
    }

    // save old entity
    m_oldState = sceneResult.value();

    int sceneId;

    auto deleteResult = m_repository->remove(sceneId);

    if (Q_UNLIKELY(!deleteResult.hasError()))
    {
        qDebug() << "Error deleting scene from repository:" << deleteResult.error().message();
        return Result<int>(deleteResult.error());
    }

    emit sceneRemoved(deleteResult.value());

    qDebug() << "Scene removed:" << sceneId;

    return Result<int>(sceneId);
}

Result<int> RemoveSceneCommandHandler::restoreImpl()
{

    // Add the scene to the repository
    auto sceneResult = m_repository->add(std::move(m_oldState));

    if (Q_UNLIKELY(sceneResult.hasError()))
    {
        return Result<int>(sceneResult.error());
    }

    auto sceneDTO = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(sceneResult.value());

    emit sceneCreated(sceneDTO);
    qDebug() << "Scene added:" << sceneDTO.id();

    // Return the UUID of the newly created scene as a Result object
    return Result<int>(0);
}

bool RemoveSceneCommandHandler::s_mappingRegistered = false;

void RemoveSceneCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
}
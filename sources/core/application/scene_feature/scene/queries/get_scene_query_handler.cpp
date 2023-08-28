#include "get_scene_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"

using namespace Application::Features::Scene::Queries;

GetSceneQueryHandler::GetSceneQueryHandler(InterfaceSceneRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneDTO> GetSceneQueryHandler::handle(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query)
{
    Result<SceneDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneQuery:" << ex.what();
    }
    return result;
}

Result<SceneDTO> GetSceneQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query)
{
    qDebug() << "GetSceneQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneResult = m_repository->get(query.id);

    if (Q_UNLIKELY(sceneResult.isError()))
    {
        return Result<SceneDTO>(sceneResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(sceneResult.value());

    qDebug() << "GetSceneQueryHandler::handleImpl done";

    return Result<SceneDTO>(dto);
}

bool GetSceneQueryHandler::s_mappingRegistered = false;

void GetSceneQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_scene_query_handler.h"
#include "repository/interface_scene_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Scene::Queries;

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
        result = Result<SceneDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<SceneDTO> GetSceneQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise, const GetSceneQuery &query)
{
    qDebug() << "GetSceneQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(SceneDTO, sceneResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneDTO>(sceneResult.value());

    qDebug() << "GetSceneQueryHandler::handleImpl done";

    return Result<SceneDTO>(dto);
}

bool GetSceneQueryHandler::s_mappingRegistered = false;

void GetSceneQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Scene, Contracts::DTO::Scene::SceneDTO>(
        true, true);
}
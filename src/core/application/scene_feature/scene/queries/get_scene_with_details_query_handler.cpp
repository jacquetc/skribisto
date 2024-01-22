// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_scene_with_details_query_handler.h"
#include "repository/interface_scene_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Scene::Queries;

GetSceneWithDetailsQueryHandler::GetSceneWithDetailsQueryHandler(InterfaceSceneRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneWithDetailsDTO> GetSceneWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                    const GetSceneQuery &query)
{
    Result<SceneWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<SceneWithDetailsDTO> GetSceneWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                        const GetSceneQuery &query)
{
    qDebug() << "GetSceneWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(SceneWithDetailsDTO, sceneResult)

    Skribisto::Domain::Scene scene = sceneResult.value();

    // map
    auto sceneWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneWithDetailsDTO>(scene);

    qDebug() << "GetSceneWithDetailsQueryHandler::handleImpl done";

    return Result<SceneWithDetailsDTO>(sceneWithDetailsDTO);
}

bool GetSceneWithDetailsQueryHandler::s_mappingRegistered = false;

void GetSceneWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Scene,
                                                           Contracts::DTO::Scene::SceneWithDetailsDTO>();
}
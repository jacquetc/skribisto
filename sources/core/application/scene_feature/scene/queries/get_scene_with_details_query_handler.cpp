#include "get_scene_with_details_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"

using namespace Application::Features::Scene::Queries;

GetSceneWithDetailsQueryHandler::GetSceneWithDetailsQueryHandler(QSharedPointer<InterfaceSceneRepository> repository)
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
        result = Result<SceneWithDetailsDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneQuery:" << ex.what();
    }
    return result;
}

Result<SceneWithDetailsDTO> GetSceneWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                        const GetSceneQuery &query)
{
    qDebug() << "GetSceneWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneResult = m_repository->get(query.id);

    if (Q_UNLIKELY(!sceneResult.isError()))
    {
        return Result<SceneWithDetailsDTO>(sceneResult.error());
    }

    Domain::Scene scene = sceneResult.value();

    // set up lazy loading:

    scene.setParagraphsLoader(m_repository->fetchParagraphsLoader());

    // map
    auto WithDetailsDTO = AutoMapper::AutoMapper::map<Domain::Scene, SceneWithDetailsDTO>(sceneResult.value());

    qDebug() << "GetSceneWithDetailsQueryHandler::handleImpl done";

    return Result<SceneWithDetailsDTO>(WithDetailsDTO);
}

bool GetSceneWithDetailsQueryHandler::s_mappingRegistered = false;

void GetSceneWithDetailsQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneWithDetailsDTO>();
}
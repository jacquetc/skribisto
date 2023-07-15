#include "get_all_scene_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"

using namespace Application::Features::Scene::Queries;

GetAllSceneQueryHandler::GetAllSceneQueryHandler(QSharedPointer<InterfaceSceneRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<SceneDTO>> GetAllSceneQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllSceneQueryHandler::handle called";

    Result<QList<SceneDTO>> result;

    try
    {
        result = handleImpl(progressPromise);
    }
    catch (const std::exception &ex)
    {
        result = Result<QList<SceneDTO>>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllSceneQuery:" << ex.what();
    }
    return result;
}

Result<QList<SceneDTO>> GetAllSceneQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllSceneQueryHandler::handleImpl called";

    // do
    auto sceneResult = m_repository->getAll();

    if (Q_UNLIKELY(sceneResult.isError()))
    {
        return Result<QList<SceneDTO>>(sceneResult.error());
    }

    // map
    QList<SceneDTO> dtoList;

    for (const Domain::Scene &scene : sceneResult.value())
    {
        auto dto = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(scene);
        dtoList.append(dto);
    }

    qDebug() << "GetAllSceneQueryHandler::handleImpl done";

    return Result<QList<SceneDTO>>(dtoList);
}

bool GetAllSceneQueryHandler::s_mappingRegistered = false;

void GetAllSceneQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
}
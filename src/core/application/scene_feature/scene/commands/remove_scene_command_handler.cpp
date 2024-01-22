// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_scene_command_handler.h"
#include "repository/interface_scene_repository.h"
#include "scene/validators/remove_scene_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Commands;
using namespace Skribisto::Application::Features::Scene::Commands;
using namespace Skribisto::Contracts::CQRS::Scene::Validators;

RemoveSceneCommandHandler::RemoveSceneCommandHandler(InterfaceSceneRepository *repository) : m_repository(repository)
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
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                  const RemoveSceneCommand &request)
{
    int sceneId = request.id;

    // Validate the command using the validator
    auto validator = RemoveSceneCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(sceneId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<Skribisto::Domain::Scene> sceneResult = m_repository->get(sceneId);

    QLN_RETURN_IF_ERROR(int, sceneResult)

    // save old entity
    m_oldState = sceneResult.value();

    auto deleteResult = m_repository->removeInCascade(QList<int>() << sceneId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove signals
    // emit sceneRemoved(deleteResult.value());

    qDebug() << "Scene removed:" << sceneId;

    return Result<int>(sceneId);
}

Result<int> RemoveSceneCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveSceneCommandHandler::s_mappingRegistered = false;

void RemoveSceneCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Scene, Contracts::DTO::Scene::SceneDTO>(
        true, true);
}
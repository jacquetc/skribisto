#include "update_scene_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"
#include "scene/validators/update_scene_command_validator.h"

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Commands;
using namespace Contracts::CQRS::Scene::Validators;
using namespace Application::Features::Scene::Commands;

UpdateSceneCommandHandler::UpdateSceneCommandHandler(QSharedPointer<InterfaceSceneRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneDTO> UpdateSceneCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                   const UpdateSceneCommand &request)
{
    Result<SceneDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneCommand:" << ex.what();
    }
    return result;
}

Result<SceneDTO> UpdateSceneCommandHandler::restore()
{
    Result<SceneDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneDTO> UpdateSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const UpdateSceneCommand &request)
{
    qDebug() << "UpdateSceneCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateSceneCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<SceneDTO>(validatorResult.error());
    }

    // map
    auto scene = AutoMapper::AutoMapper::map<UpdateSceneDTO, Domain::Scene>(request.req);

    // set update timestamp only on first pass
    if (m_newState.isEmpty())
    {
        scene.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_newState.isEmpty())
    {
        Result<Domain::Scene> saveResult = m_repository->get(request.req.id());

        if (Q_UNLIKELY(saveResult.hasError()))
        {
            qDebug() << "Error getting scene from repository:" << saveResult.error().message();
            return Result<SceneDTO>(saveResult.error());
        }

        // map
        m_newState = Result<SceneDTO>(AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(saveResult.value()));
    }

    // do
    auto sceneResult = m_repository->update(std::move(scene));

    if (sceneResult.hasError())
    {
        return Result<SceneDTO>(sceneResult.error());
    }

    // map
    auto sceneDto = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(sceneResult.value());

    emit sceneUpdated(sceneDto);

    qDebug() << "UpdateSceneCommandHandler::handleImpl done";

    return Result<SceneDTO>(sceneDto);
}

Result<SceneDTO> UpdateSceneCommandHandler::restoreImpl()
{
    qDebug() << "UpdateSceneCommandHandler::restoreImpl called with id" << m_newState.value().uuid();

    // map
    auto scene = AutoMapper::AutoMapper::map<SceneDTO, Domain::Scene>(m_newState.value());

    // do
    auto sceneResult = m_repository->update(std::move(scene));

    if (Q_UNLIKELY(sceneResult.hasError()))
    {
        return Result<SceneDTO>(sceneResult.error());
    }

    // map
    auto sceneDto = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(sceneResult.value());

    emit sceneUpdated(sceneDto);

    qDebug() << "UpdateSceneCommandHandler::restoreImpl done";

    return Result<SceneDTO>(sceneDto);
}

bool UpdateSceneCommandHandler::s_mappingRegistered = false;

void UpdateSceneCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::UpdateSceneDTO, Domain::Scene>();
}
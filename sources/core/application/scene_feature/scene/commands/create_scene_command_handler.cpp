#include "create_scene_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_repository.h"
#include "scene/validators/create_scene_command_validator.h"

using namespace Contracts::DTO::Scene;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Scene::Validators;
using namespace Application::Features::Scene::Commands;

CreateSceneCommandHandler::CreateSceneCommandHandler(InterfaceSceneRepository *repository) : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneDTO> CreateSceneCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                   const CreateSceneCommand &request)
{
    Result<SceneDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneCommand:" << ex.what();
    }
    return result;
}

Result<SceneDTO> CreateSceneCommandHandler::restore()
{
    Result<SceneDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneDTO> CreateSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const CreateSceneCommand &request)
{
    qDebug() << "CreateSceneCommandHandler::handleImpl called";
    Domain::Scene scene;

    if (m_newState.isEmpty())
    {
        // Validate the create Scene command using the validator
        auto validator = CreateSceneCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<SceneDTO>(validatorResult.error());
        }

        // Map the create Scene command to a domain Scene object and
        // generate a UUID
        scene = AutoMapper::AutoMapper::map<CreateSceneDTO, Domain::Scene>(request.req);

        // allow for forcing the uuid
        if (scene.uuid().isNull())
        {
            scene.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        scene.setCreationDate(QDateTime::currentDateTime());
        scene.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        // Map the create scene command to a domain scene object and
        // generate a UUID
        scene = AutoMapper::AutoMapper::map<SceneDTO, Domain::Scene>(m_newState.value());
    }

    // Add the scene to the repository

    m_repository->beginChanges();
    auto sceneResult = m_repository->add(std::move(scene));

    if (Q_UNLIKELY(sceneResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<SceneDTO>(sceneResult.error());
    }
    m_repository->saveChanges();

    auto sceneDTO = AutoMapper::AutoMapper::map<Domain::Scene, SceneDTO>(sceneResult.value());

    m_newState = Result<SceneDTO>(sceneDTO);

    emit sceneCreated(sceneDTO);

    qDebug() << "Scene added:" << sceneDTO.id();

    // Return the DTO of the newly created Scene as a Result object
    return Result<SceneDTO>(sceneDTO);
}

Result<SceneDTO> CreateSceneCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newState.value().id());

    if (Q_UNLIKELY(deleteResult.hasError()))
    {
        qDebug() << "Error deleting Scene from repository:" << deleteResult.error().message();
        return Result<SceneDTO>(deleteResult.error());
    }

    emit sceneRemoved(deleteResult.value());

    qDebug() << "Scene removed:" << deleteResult.value();

    return Result<SceneDTO>(SceneDTO());
}

bool CreateSceneCommandHandler::s_mappingRegistered = false;

void CreateSceneCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::CreateSceneDTO, Domain::Scene>();
}
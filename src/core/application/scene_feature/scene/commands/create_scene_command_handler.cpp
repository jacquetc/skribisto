// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_scene_command_handler.h"
#include "scene/validators/create_scene_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

#include "chapter.h"

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Validators;
using namespace Skribisto::Application::Features::Scene::Commands;

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
        result = Result<SceneDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<SceneDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneDTO> CreateSceneCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                       const CreateSceneCommand &request)
{
    qDebug() << "CreateSceneCommandHandler::handleImpl called";
    Skribisto::Domain::Scene scene;
    CreateSceneDTO createDTO = request.req;

    QList<Skribisto::Domain::Scene> ownerEntityScenes;

    // Get the entities from owner
    int ownerId = createDTO.chapterId();
    m_ownerId = ownerId;

    if (m_firstPass)
    {
        // Validate the create Scene command using the validator
        auto validator = CreateSceneCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(SceneDTO, validatorResult);

        // Map the create Scene command to a domain Scene object and
        // generate a UUID
        scene = Qleany::Tools::AutoMapper::AutoMapper::map<CreateSceneDTO, Skribisto::Domain::Scene>(createDTO);

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
        scene = m_newEntity.value();
    }

    // Add the scene to the repository

    m_repository->beginChanges();
    auto sceneResult = m_repository->add(std::move(scene));

    QLN_RETURN_IF_ERROR_WITH_ACTION(SceneDTO, sceneResult, m_repository->cancelChanges();)

    // Get the newly created Scene entity
    scene = sceneResult.value();
    // Save the newly created entity
    m_newEntity = sceneResult;

    //  Manage relation to owner

    int position = -1;

    if (m_firstPass)
    {

        auto originalOwnerScenesResult = m_repository->getEntitiesInRelationOf(Chapter::schema, ownerId, "scenes");
        if (Q_UNLIKELY(originalOwnerScenesResult.hasError()))
        {
            return Result<SceneDTO>(originalOwnerScenesResult.error());
        }
        auto originalOwnerScenes = originalOwnerScenesResult.value();

        // save
        m_oldOwnerScenes = originalOwnerScenes;

        // Insert to the right position

        position = createDTO.position();
        if (position == -1)
        {
            position = originalOwnerScenes.count();
        }
        if (position > originalOwnerScenes.count())
        {
            position = originalOwnerScenes.count();
        }
        else if (position < 0)
        {
            position = 0;
        }

        m_position = position;

        originalOwnerScenes.insert(position, scene);

        m_ownerScenesNewState = originalOwnerScenes;
        ownerEntityScenes = originalOwnerScenes;
    }
    else
    {
        ownerEntityScenes = m_ownerScenesNewState;
        position = m_position;
    }

    // Add the scene to the owner entity
    Result<QList<Skribisto::Domain::Scene>> updateResult =
        m_repository->updateEntitiesInRelationOf(Chapter::schema, ownerId, "scenes", ownerEntityScenes);

    QLN_RETURN_IF_ERROR_WITH_ACTION(SceneDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = sceneResult;

    auto sceneDTO = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneDTO>(sceneResult.value());
    emit sceneCreated(sceneDTO);

    // send an insertion signal
    emit relationWithOwnerInserted(scene.id(), ownerId, position);

    qDebug() << "Scene added:" << sceneDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created Scene as a Result object
    return Result<SceneDTO>(sceneDTO);
}

Result<SceneDTO> CreateSceneCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(SceneDTO, deleteResult)

    emit sceneRemoved(deleteResult.value());

    qDebug() << "Scene removed:" << deleteResult.value();

    emit relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<SceneDTO>(SceneDTO());
}

bool CreateSceneCommandHandler::s_mappingRegistered = false;

void CreateSceneCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Scene, Contracts::DTO::Scene::SceneDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::CreateSceneDTO,
                                                           Skribisto::Domain::Scene>();
}
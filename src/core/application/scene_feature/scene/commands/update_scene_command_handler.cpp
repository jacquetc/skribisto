// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_scene_command_handler.h"
#include "repository/interface_scene_repository.h"
#include "scene/validators/update_scene_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Scene;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Scene::Commands;
using namespace Skribisto::Contracts::CQRS::Scene::Validators;
using namespace Skribisto::Application::Features::Scene::Commands;

UpdateSceneCommandHandler::UpdateSceneCommandHandler(InterfaceSceneRepository *repository) : m_repository(repository)
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
        result = Result<SceneDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<SceneDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
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

    QLN_RETURN_IF_ERROR(SceneDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::Scene> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(SceneDTO, currentResult)

        // map
        m_undoState = Result<SceneDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneDTO>(currentResult.value()));
    }
    auto updateDto = Qleany::Tools::AutoMapper::AutoMapper::map<SceneDTO, UpdateSceneDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto scene = Qleany::Tools::AutoMapper::AutoMapper::map<UpdateSceneDTO, Skribisto::Domain::Scene>(updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        scene.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto sceneResult = m_repository->update(std::move(scene));

    if (sceneResult.hasError())
    {
        return Result<SceneDTO>(sceneResult.error());
    }

    // map
    auto sceneDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneDTO>(sceneResult.value());

    emit sceneUpdated(sceneDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit sceneDetailsUpdated(sceneDto.id());
    }

    qDebug() << "UpdateSceneCommandHandler::handleImpl done";

    return Result<SceneDTO>(sceneDto);
}

Result<SceneDTO> UpdateSceneCommandHandler::restoreImpl()
{
    qDebug() << "UpdateSceneCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto scene = Qleany::Tools::AutoMapper::AutoMapper::map<SceneDTO, Skribisto::Domain::Scene>(m_undoState.value());

    // do
    auto sceneResult = m_repository->update(std::move(scene));

    QLN_RETURN_IF_ERROR(SceneDTO, sceneResult)

    // map
    auto sceneDto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Scene, SceneDTO>(sceneResult.value());

    emit sceneUpdated(sceneDto);

    qDebug() << "UpdateSceneCommandHandler::restoreImpl done";

    return Result<SceneDTO>(sceneDto);
}

bool UpdateSceneCommandHandler::s_mappingRegistered = false;

void UpdateSceneCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Scene, Contracts::DTO::Scene::SceneDTO>(
        true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::UpdateSceneDTO,
                                                           Contracts::DTO::Scene::SceneDTO>(true, true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::UpdateSceneDTO,
                                                           Skribisto::Domain::Scene>();
}
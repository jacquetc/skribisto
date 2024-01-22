// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "update_scene_paragraph_command_handler.h"
#include "repository/interface_scene_paragraph_repository.h"
#include "scene_paragraph/validators/update_scene_paragraph_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Validators;
using namespace Skribisto::Application::Features::SceneParagraph::Commands;

UpdateSceneParagraphCommandHandler::UpdateSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneParagraphDTO> UpdateSceneParagraphCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                     const UpdateSceneParagraphCommand &request)
{
    Result<SceneParagraphDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneParagraphCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<SceneParagraphDTO> UpdateSceneParagraphCommandHandler::restore()
{
    Result<SceneParagraphDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneParagraphDTO> UpdateSceneParagraphCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                         const UpdateSceneParagraphCommand &request)
{
    qDebug() << "UpdateSceneParagraphCommandHandler::handleImpl called with id" << request.req.id();

    // validate:
    auto validator = UpdateSceneParagraphCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(request.req);

    QLN_RETURN_IF_ERROR(SceneParagraphDTO, validatorResult)

    // save old state
    if (m_undoState.isEmpty())
    {
        Result<Skribisto::Domain::SceneParagraph> currentResult = m_repository->get(request.req.id());

        QLN_RETURN_IF_ERROR(SceneParagraphDTO, currentResult)

        // map
        m_undoState = Result<SceneParagraphDTO>(
            Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::SceneParagraph, SceneParagraphDTO>(
                currentResult.value()));
    }
    auto updateDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<SceneParagraphDTO, UpdateSceneParagraphDTO>(m_undoState.value());
    updateDto << request.req;

    // map
    auto sceneParagraph =
        Qleany::Tools::AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Skribisto::Domain::SceneParagraph>(
            updateDto);

    // set update timestamp only on first pass
    if (m_undoState.isEmpty())
    {
        sceneParagraph.setUpdateDate(QDateTime::currentDateTime());
    }

    // do
    auto sceneParagraphResult = m_repository->update(std::move(sceneParagraph));

    if (sceneParagraphResult.hasError())
    {
        return Result<SceneParagraphDTO>(sceneParagraphResult.error());
    }

    // map
    auto sceneParagraphDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::SceneParagraph, SceneParagraphDTO>(
            sceneParagraphResult.value());

    emit sceneParagraphUpdated(sceneParagraphDto);

    if (request.req.metaData().areDetailsSet())
    {
        emit sceneParagraphDetailsUpdated(sceneParagraphDto.id());
    }

    qDebug() << "UpdateSceneParagraphCommandHandler::handleImpl done";

    return Result<SceneParagraphDTO>(sceneParagraphDto);
}

Result<SceneParagraphDTO> UpdateSceneParagraphCommandHandler::restoreImpl()
{
    qDebug() << "UpdateSceneParagraphCommandHandler::restoreImpl called with id" << m_undoState.value().uuid();

    // map
    auto sceneParagraph =
        Qleany::Tools::AutoMapper::AutoMapper::map<SceneParagraphDTO, Skribisto::Domain::SceneParagraph>(
            m_undoState.value());

    // do
    auto sceneParagraphResult = m_repository->update(std::move(sceneParagraph));

    QLN_RETURN_IF_ERROR(SceneParagraphDTO, sceneParagraphResult)

    // map
    auto sceneParagraphDto =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::SceneParagraph, SceneParagraphDTO>(
            sceneParagraphResult.value());

    emit sceneParagraphUpdated(sceneParagraphDto);

    qDebug() << "UpdateSceneParagraphCommandHandler::restoreImpl done";

    return Result<SceneParagraphDTO>(sceneParagraphDto);
}

bool UpdateSceneParagraphCommandHandler::s_mappingRegistered = false;

void UpdateSceneParagraphCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::SceneParagraph,
                                                           Contracts::DTO::SceneParagraph::SceneParagraphDTO>(true,
                                                                                                              true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO,
                                                           Contracts::DTO::SceneParagraph::SceneParagraphDTO>(true,
                                                                                                              true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO,
                                                           Skribisto::Domain::SceneParagraph>();
}
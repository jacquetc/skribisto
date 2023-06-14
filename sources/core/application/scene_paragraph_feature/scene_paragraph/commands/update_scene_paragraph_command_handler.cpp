#include "update_scene_paragraph_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_paragraph_repository.h"
#include "scene_paragraph/validators/update_scene_paragraph_command_validator.h"

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Commands;
using namespace Contracts::CQRS::SceneParagraph::Validators;
using namespace Application::Features::SceneParagraph::Commands;

UpdateSceneParagraphCommandHandler::UpdateSceneParagraphCommandHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> repository)
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
        result = Result<SceneParagraphDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling UpdateSceneParagraphCommand:" << ex.what();
    }
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
        result = Result<SceneParagraphDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
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

    if (Q_UNLIKELY(!validatorResult.hasError()))
    {
        return Result<SceneParagraphDTO>(validatorResult.error());
    }

    // map
    auto sceneParagraph = AutoMapper::AutoMapper::map<UpdateSceneParagraphDTO, Domain::SceneParagraph>(request.req);

    // set update timestamp only on first pass
    if (m_newState.isEmpty())
    {
        sceneParagraph.setUpdateDate(QDateTime::currentDateTime());
    }

    // save old state
    if (m_newState.isEmpty())
    {
        Result<Domain::SceneParagraph> saveResult = m_repository->get(request.req.id());

        if (Q_UNLIKELY(!saveResult.hasError()))
        {
            qDebug() << "Error getting sceneParagraph from repository:" << saveResult.error().message();
            return Result<SceneParagraphDTO>(saveResult.error());
        }

        // map
        m_newState = Result<SceneParagraphDTO>(
            AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(saveResult.value()));
    }

    // do
    auto sceneParagraphResult = m_repository->update(std::move(sceneParagraph));

    if (sceneParagraphResult.hasError())
    {
        return Result<SceneParagraphDTO>(sceneParagraphResult.error());
    }

    // map
    auto sceneParagraphDto =
        AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraphResult.value());

    emit sceneParagraphUpdated(sceneParagraphDto);

    qDebug() << "UpdateSceneParagraphCommandHandler::handleImpl done";

    return Result<SceneParagraphDTO>(sceneParagraphDto);
}

Result<SceneParagraphDTO> UpdateSceneParagraphCommandHandler::restoreImpl()
{
    qDebug() << "UpdateSceneParagraphCommandHandler::restoreImpl called with id" << m_newState.value().uuid();

    // map
    auto sceneParagraph = AutoMapper::AutoMapper::map<SceneParagraphDTO, Domain::SceneParagraph>(m_newState.value());

    // do
    auto sceneParagraphResult = m_repository->update(std::move(sceneParagraph));

    if (Q_UNLIKELY(!sceneParagraphResult.hasError()))
    {
        return Result<SceneParagraphDTO>(sceneParagraphResult.error());
    }

    // map
    auto sceneParagraphDto =
        AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraphResult.value());

    emit sceneParagraphUpdated(sceneParagraphDto);

    qDebug() << "UpdateSceneParagraphCommandHandler::restoreImpl done";

    return Result<SceneParagraphDTO>(sceneParagraphDto);
}

bool UpdateSceneParagraphCommandHandler::s_mappingRegistered = false;

void UpdateSceneParagraphCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::SceneParagraph, Contracts::DTO::SceneParagraph::SceneParagraphDTO>(
        true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO,
                                            Domain::SceneParagraph>();
}
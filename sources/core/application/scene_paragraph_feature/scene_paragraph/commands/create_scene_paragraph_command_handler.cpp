#include "create_scene_paragraph_command_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_paragraph_repository.h"
#include "scene_paragraph/validators/create_scene_paragraph_command_validator.h"

using namespace Contracts::DTO::SceneParagraph;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::SceneParagraph::Validators;
using namespace Application::Features::SceneParagraph::Commands;

CreateSceneParagraphCommandHandler::CreateSceneParagraphCommandHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                     const CreateSceneParagraphCommand &request)
{
    Result<SceneParagraphDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneParagraphCommand:" << ex.what();
    }
    return result;
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::restore()
{
    Result<SceneParagraphDTO> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                         const CreateSceneParagraphCommand &request)
{
    qDebug() << "CreateSceneParagraphCommandHandler::handleImpl called";
    Domain::SceneParagraph sceneParagraph;

    if (m_newState.isEmpty())
    {
        // Validate the create SceneParagraph command using the validator
        auto validator = CreateSceneParagraphCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(request.req);

        if (Q_UNLIKELY(validatorResult.hasError()))
        {
            return Result<SceneParagraphDTO>(validatorResult.error());
        }

        // Map the create SceneParagraph command to a domain SceneParagraph object and
        // generate a UUID
        sceneParagraph = AutoMapper::AutoMapper::map<CreateSceneParagraphDTO, Domain::SceneParagraph>(request.req);

        // allow for forcing the uuid
        if (sceneParagraph.uuid().isNull())
        {
            sceneParagraph.setUuid(QUuid::createUuid());
        }

        // Set the creation and update timestamps to the current date and time
        sceneParagraph.setCreationDate(QDateTime::currentDateTime());
        sceneParagraph.setUpdateDate(QDateTime::currentDateTime());
    }
    else
    {
        // Map the create sceneParagraph command to a domain sceneParagraph object and
        // generate a UUID
        sceneParagraph = AutoMapper::AutoMapper::map<SceneParagraphDTO, Domain::SceneParagraph>(m_newState.value());
    }

    // Add the sceneParagraph to the repository

    m_repository->beginChanges();
    auto sceneParagraphResult = m_repository->add(std::move(sceneParagraph));

    if (Q_UNLIKELY(sceneParagraphResult.hasError()))
    {
        m_repository->cancelChanges();
        return Result<SceneParagraphDTO>(sceneParagraphResult.error());
    }
    m_repository->saveChanges();

    auto sceneParagraphDTO =
        AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraphResult.value());

    m_newState = Result<SceneParagraphDTO>(sceneParagraphDTO);

    emit sceneParagraphCreated(sceneParagraphDTO);

    qDebug() << "SceneParagraph added:" << sceneParagraphDTO.id();

    // Return the DTO of the newly created SceneParagraph as a Result object
    return Result<SceneParagraphDTO>(sceneParagraphDTO);
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::restoreImpl()
{

    auto deleteResult = m_repository->remove(m_newState.value().id());

    if (Q_UNLIKELY(!deleteResult.hasError()))
    {
        qDebug() << "Error deleting SceneParagraph from repository:" << deleteResult.error().message();
        return Result<SceneParagraphDTO>(deleteResult.error());
    }

    emit sceneParagraphRemoved(deleteResult.value());

    qDebug() << "SceneParagraph removed:" << deleteResult.value();

    return Result<SceneParagraphDTO>(SceneParagraphDTO());
}

bool CreateSceneParagraphCommandHandler::s_mappingRegistered = false;

void CreateSceneParagraphCommandHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::SceneParagraph, Contracts::DTO::SceneParagraph::SceneParagraphDTO>(
        true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO,
                                            Domain::SceneParagraph>();
}
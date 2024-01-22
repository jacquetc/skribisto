// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "create_scene_paragraph_command_handler.h"
#include "scene_paragraph/validators/create_scene_paragraph_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

#include "scene.h"

using namespace Qleany;
using namespace Skribisto::Domain;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Validators;
using namespace Skribisto::Application::Features::SceneParagraph::Commands;

CreateSceneParagraphCommandHandler::CreateSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository)
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
        result = Result<SceneParagraphDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneParagraphCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
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
        result = Result<SceneParagraphDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling CreateSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                         const CreateSceneParagraphCommand &request)
{
    qDebug() << "CreateSceneParagraphCommandHandler::handleImpl called";
    Skribisto::Domain::SceneParagraph sceneParagraph;
    CreateSceneParagraphDTO createDTO = request.req;

    QList<Skribisto::Domain::SceneParagraph> ownerEntityParagraphs;

    // Get the entities from owner
    int ownerId = createDTO.sceneId();
    m_ownerId = ownerId;

    if (m_firstPass)
    {
        // Validate the create SceneParagraph command using the validator
        auto validator = CreateSceneParagraphCommandValidator(m_repository);
        Result<void> validatorResult = validator.validate(createDTO);

        QLN_RETURN_IF_ERROR(SceneParagraphDTO, validatorResult);

        // Map the create SceneParagraph command to a domain SceneParagraph object and
        // generate a UUID
        sceneParagraph =
            Qleany::Tools::AutoMapper::AutoMapper::map<CreateSceneParagraphDTO, Skribisto::Domain::SceneParagraph>(
                createDTO);

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
        sceneParagraph = m_newEntity.value();
    }

    // Add the sceneParagraph to the repository

    m_repository->beginChanges();
    auto sceneParagraphResult = m_repository->add(std::move(sceneParagraph));

    QLN_RETURN_IF_ERROR_WITH_ACTION(SceneParagraphDTO, sceneParagraphResult, m_repository->cancelChanges();)

    // Get the newly created SceneParagraph entity
    sceneParagraph = sceneParagraphResult.value();
    // Save the newly created entity
    m_newEntity = sceneParagraphResult;

    //  Manage relation to owner

    int position = -1;

    if (m_firstPass)
    {

        auto originalOwnerParagraphsResult =
            m_repository->getEntitiesInRelationOf(Scene::schema, ownerId, "paragraphs");
        if (Q_UNLIKELY(originalOwnerParagraphsResult.hasError()))
        {
            return Result<SceneParagraphDTO>(originalOwnerParagraphsResult.error());
        }
        auto originalOwnerParagraphs = originalOwnerParagraphsResult.value();

        // save
        m_oldOwnerParagraphs = originalOwnerParagraphs;

        // Insert to the right position

        position = createDTO.position();
        if (position == -1)
        {
            position = originalOwnerParagraphs.count();
        }
        if (position > originalOwnerParagraphs.count())
        {
            position = originalOwnerParagraphs.count();
        }
        else if (position < 0)
        {
            position = 0;
        }

        m_position = position;

        originalOwnerParagraphs.insert(position, sceneParagraph);

        m_ownerParagraphsNewState = originalOwnerParagraphs;
        ownerEntityParagraphs = originalOwnerParagraphs;
    }
    else
    {
        ownerEntityParagraphs = m_ownerParagraphsNewState;
        position = m_position;
    }

    // Add the sceneParagraph to the owner entity
    Result<QList<Skribisto::Domain::SceneParagraph>> updateResult =
        m_repository->updateEntitiesInRelationOf(Scene::schema, ownerId, "paragraphs", ownerEntityParagraphs);

    QLN_RETURN_IF_ERROR_WITH_ACTION(SceneParagraphDTO, updateResult, m_repository->cancelChanges();)

    m_repository->saveChanges();

    m_newEntity = sceneParagraphResult;

    auto sceneParagraphDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::SceneParagraph, SceneParagraphDTO>(
            sceneParagraphResult.value());
    emit sceneParagraphCreated(sceneParagraphDTO);

    // send an insertion signal
    emit relationWithOwnerInserted(sceneParagraph.id(), ownerId, position);

    qDebug() << "SceneParagraph added:" << sceneParagraphDTO.id();

    m_firstPass = false;

    // Return the DTO of the newly created SceneParagraph as a Result object
    return Result<SceneParagraphDTO>(sceneParagraphDTO);
}

Result<SceneParagraphDTO> CreateSceneParagraphCommandHandler::restoreImpl()
{
    int entityId = m_newEntity.value().id();
    auto deleteResult = m_repository->remove(entityId);

    QLN_RETURN_IF_ERROR(SceneParagraphDTO, deleteResult)

    emit sceneParagraphRemoved(deleteResult.value());

    qDebug() << "SceneParagraph removed:" << deleteResult.value();

    emit relationWithOwnerRemoved(entityId, m_ownerId);

    return Result<SceneParagraphDTO>(SceneParagraphDTO());
}

bool CreateSceneParagraphCommandHandler::s_mappingRegistered = false;

void CreateSceneParagraphCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::SceneParagraph,
                                                           Contracts::DTO::SceneParagraph::SceneParagraphDTO>(true,
                                                                                                              true);
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO,
                                                           Skribisto::Domain::SceneParagraph>();
}
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "remove_scene_paragraph_command_handler.h"
#include "repository/interface_scene_paragraph_repository.h"
#include "scene_paragraph/validators/remove_scene_paragraph_command_validator.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::SceneParagraph;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands;
using namespace Skribisto::Application::Features::SceneParagraph::Commands;
using namespace Skribisto::Contracts::CQRS::SceneParagraph::Validators;

RemoveSceneParagraphCommandHandler::RemoveSceneParagraphCommandHandler(InterfaceSceneParagraphRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<int> RemoveSceneParagraphCommandHandler::handle(QPromise<Result<void>> &progressPromise,
                                                       const RemoveSceneParagraphCommand &request)
{
    Result<int> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneParagraphCommand:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<int> RemoveSceneParagraphCommandHandler::restore()
{
    Result<int> result;

    try
    {
        result = restoreImpl();
    }
    catch (const std::exception &ex)
    {
        result = Result<int>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling RemoveSceneParagraphCommand restore:" << ex.what();
    }
    return result;
}

Result<int> RemoveSceneParagraphCommandHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                           const RemoveSceneParagraphCommand &request)
{
    int sceneParagraphId = request.id;

    // Validate the command using the validator
    auto validator = RemoveSceneParagraphCommandValidator(m_repository);
    Result<void> validatorResult = validator.validate(sceneParagraphId);

    QLN_RETURN_IF_ERROR(int, validatorResult);

    Result<Skribisto::Domain::SceneParagraph> sceneParagraphResult = m_repository->get(sceneParagraphId);

    QLN_RETURN_IF_ERROR(int, sceneParagraphResult)

    // save old entity
    m_oldState = sceneParagraphResult.value();

    auto deleteResult = m_repository->removeInCascade(QList<int>() << sceneParagraphId);

    QLN_RETURN_IF_ERROR(int, deleteResult)

    // repositories handle remove signals
    // emit sceneParagraphRemoved(deleteResult.value());

    qDebug() << "SceneParagraph removed:" << sceneParagraphId;

    return Result<int>(sceneParagraphId);
}

Result<int> RemoveSceneParagraphCommandHandler::restoreImpl()
{
    // no restore possible
    return Result<int>(0);
}

bool RemoveSceneParagraphCommandHandler::s_mappingRegistered = false;

void RemoveSceneParagraphCommandHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::SceneParagraph,
                                                           Contracts::DTO::SceneParagraph::SceneParagraphDTO>(true,
                                                                                                              true);
}
#include "get_all_scene_paragraphs_query_handler.h"
#include "automapper/automapper.h"
#include "writing/get_all_scene_paragraphs_dto.h"
#include "writing/validators/get_all_scene_paragraphs_query_validator.h"
#include <QDebug>

using namespace Contracts::DTO::Writing;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Writing::Validators;
using namespace Application::Features::Writing::Queries;

GetAllSceneParagraphsQueryHandler::GetAllSceneParagraphsQueryHandler(
    InterfaceSceneRepository *sceneRepository, InterfaceSceneParagraphRepository *sceneParagraphRepository)
    : m_sceneRepository(sceneRepository), m_sceneParagraphRepository(sceneParagraphRepository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<GetAllSceneParagraphsReplyDTO> GetAllSceneParagraphsQueryHandler::handle(
    QPromise<Result<void>> &progressPromise, const GetAllSceneParagraphsQuery &request)
{
    Result<GetAllSceneParagraphsReplyDTO> result;

    try
    {
        result = handleImpl(progressPromise, request);
    }
    catch (const std::exception &ex)
    {
        result = Result<GetAllSceneParagraphsReplyDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllSceneParagraphsQuery:" << ex.what();
    }
    return result;
}

Result<GetAllSceneParagraphsReplyDTO> GetAllSceneParagraphsQueryHandler::handleImpl(
    QPromise<Result<void>> &progressPromise, const GetAllSceneParagraphsQuery &request)
{
    qDebug() << "GetAllSceneParagraphsQueryHandler::handleImpl called";

    // Domain::Writing writing;

    // Validate the query using the validator
    auto validator = GetAllSceneParagraphsQueryValidator(m_sceneRepository, m_sceneParagraphRepository);
    Result<void> validatorResult = validator.validate(request.req);

    if (Q_UNLIKELY(validatorResult.hasError()))
    {
        return Result<GetAllSceneParagraphsReplyDTO>(validatorResult.error());
    }

    // implement logic here
    // writing = AutoMapper::AutoMapper::map<GetAllSceneParagraphsReplyDTO, Domain::Writing>(request.req);

    // play here with the repositories
    Q_UNIMPLEMENTED();

    //    auto writingDTO =
    //        AutoMapper::AutoMapper::map<Domain::Writing, GetAllSceneParagraphsReplyDTO>(writingResult.value());
    GetAllSceneParagraphsReplyDTO getAllSceneParagraphsReplyDTO;
    // emit signal
    emit getAllSceneParagraphsChanged(getAllSceneParagraphsReplyDTO);

    // Return
    return Result<GetAllSceneParagraphsReplyDTO>(getAllSceneParagraphsReplyDTO);
}

bool GetAllSceneParagraphsQueryHandler::s_mappingRegistered = false;

void GetAllSceneParagraphsQueryHandler::registerMappings()
{
}

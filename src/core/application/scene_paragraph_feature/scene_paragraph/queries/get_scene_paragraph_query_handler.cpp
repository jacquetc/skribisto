// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_scene_paragraph_query_handler.h"
#include "repository/interface_scene_paragraph_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::SceneParagraph::Queries;

GetSceneParagraphQueryHandler::GetSceneParagraphQueryHandler(InterfaceSceneParagraphRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<SceneParagraphDTO> GetSceneParagraphQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                const GetSceneParagraphQuery &query)
{
    Result<SceneParagraphDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<SceneParagraphDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneParagraphQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<SceneParagraphDTO> GetSceneParagraphQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                    const GetSceneParagraphQuery &query)
{
    qDebug() << "GetSceneParagraphQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneParagraphResult = m_repository->get(query.id);

    QLN_RETURN_IF_ERROR(SceneParagraphDTO, sceneParagraphResult)

    // map
    auto dto = Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::SceneParagraph, SceneParagraphDTO>(
        sceneParagraphResult.value());

    qDebug() << "GetSceneParagraphQueryHandler::handleImpl done";

    return Result<SceneParagraphDTO>(dto);
}

bool GetSceneParagraphQueryHandler::s_mappingRegistered = false;

void GetSceneParagraphQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::SceneParagraph,
                                                           Contracts::DTO::SceneParagraph::SceneParagraphDTO>(true,
                                                                                                              true);
}
#include "get_scene_paragraph_query_handler.h"
#include "automapper/automapper.h"
#include "persistence/interface_scene_paragraph_repository.h"

using namespace Application::Features::SceneParagraph::Queries;

GetSceneParagraphQueryHandler::GetSceneParagraphQueryHandler(
    QSharedPointer<InterfaceSceneParagraphRepository> repository)
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
        result = Result<SceneParagraphDTO>(Error(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetSceneParagraphQuery:" << ex.what();
    }
    return result;
}

Result<SceneParagraphDTO> GetSceneParagraphQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                    const GetSceneParagraphQuery &query)
{
    qDebug() << "GetSceneParagraphQueryHandler::handleImpl called with id" << query.id;

    // do
    auto sceneParagraphResult = m_repository->get(query.id);

    if (Q_UNLIKELY(sceneParagraphResult.isError()))
    {
        return Result<SceneParagraphDTO>(sceneParagraphResult.error());
    }

    // map
    auto dto = AutoMapper::AutoMapper::map<Domain::SceneParagraph, SceneParagraphDTO>(sceneParagraphResult.value());

    qDebug() << "GetSceneParagraphQueryHandler::handleImpl done";

    return Result<SceneParagraphDTO>(dto);
}

bool GetSceneParagraphQueryHandler::s_mappingRegistered = false;

void GetSceneParagraphQueryHandler::registerMappings()
{
    AutoMapper::AutoMapper::registerMapping<Domain::SceneParagraph, Contracts::DTO::SceneParagraph::SceneParagraphDTO>(
        true);
}
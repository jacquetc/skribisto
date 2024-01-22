// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_atelier_with_details_query_handler.h"
#include "repository/interface_atelier_repository.h"
#include <qleany/tools/automapper/automapper.h>

using namespace Qleany;
using namespace Skribisto::Application::Features::Atelier::Queries;

GetAtelierWithDetailsQueryHandler::GetAtelierWithDetailsQueryHandler(InterfaceAtelierRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered)
    {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<AtelierWithDetailsDTO> GetAtelierWithDetailsQueryHandler::handle(QPromise<Result<void>> &progressPromise,
                                                                        const GetAtelierQuery &query)
{
    Result<AtelierWithDetailsDTO> result;

    try
    {
        result = handleImpl(progressPromise, query);
    }
    catch (const std::exception &ex)
    {
        result = Result<AtelierWithDetailsDTO>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAtelierQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<AtelierWithDetailsDTO> GetAtelierWithDetailsQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise,
                                                                            const GetAtelierQuery &query)
{
    qDebug() << "GetAtelierWithDetailsQueryHandler::handleImpl called with id" << query.id;

    // do
    auto atelierResult = m_repository->getWithDetails(query.id);

    QLN_RETURN_IF_ERROR(AtelierWithDetailsDTO, atelierResult)

    Skribisto::Domain::Atelier atelier = atelierResult.value();

    // map
    auto atelierWithDetailsDTO =
        Qleany::Tools::AutoMapper::AutoMapper::map<Skribisto::Domain::Atelier, AtelierWithDetailsDTO>(atelier);

    qDebug() << "GetAtelierWithDetailsQueryHandler::handleImpl done";

    return Result<AtelierWithDetailsDTO>(atelierWithDetailsDTO);
}

bool GetAtelierWithDetailsQueryHandler::s_mappingRegistered = false;

void GetAtelierWithDetailsQueryHandler::registerMappings()
{
    Qleany::Tools::AutoMapper::AutoMapper::registerMapping<Skribisto::Domain::Atelier,
                                                           Contracts::DTO::Atelier::AtelierWithDetailsDTO>();
}
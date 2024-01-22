// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_atelier_export.h"
#include "atelier/atelier_dto.h"
#include "atelier/queries/get_atelier_query.h"

#include "repository/interface_atelier_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Atelier;
using namespace Skribisto::Contracts::Repository;
using namespace Skribisto::Contracts::CQRS::Atelier::Queries;

namespace Skribisto::Application::Features::Atelier::Queries
{
class SKRIBISTO_APPLICATION_ATELIER_EXPORT GetAtelierQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAtelierQueryHandler(InterfaceAtelierRepository *repository);
    Result<AtelierDTO> handle(QPromise<Result<void>> &progressPromise, const GetAtelierQuery &query);

  private:
    InterfaceAtelierRepository *m_repository;
    Result<AtelierDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetAtelierQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Atelier::Queries
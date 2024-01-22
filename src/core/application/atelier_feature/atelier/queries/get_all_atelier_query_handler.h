// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_atelier_export.h"
#include "atelier/atelier_dto.h"

#include "repository/interface_atelier_repository.h"
#include <QPromise>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::Atelier;
using namespace Skribisto::Contracts::Repository;

namespace Skribisto::Application::Features::Atelier::Queries
{
class SKRIBISTO_APPLICATION_ATELIER_EXPORT GetAllAtelierQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllAtelierQueryHandler(InterfaceAtelierRepository *repository);
    Result<QList<AtelierDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    InterfaceAtelierRepository *m_repository;
    Result<QList<AtelierDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::Atelier::Queries
#pragma once

#include "application_author_export.h"
#include "author/author_dto.h"
#include "author/queries/get_author_query.h"

#include "persistence/interface_author_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Queries;

namespace Application::Features::Author::Queries
{
class SKRIBISTO_APPLICATION_AUTHOR_EXPORT GetAuthorQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAuthorQueryHandler(InterfaceAuthorRepository *repository);
    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const GetAuthorQuery &query);

  private:
    InterfaceAuthorRepository *m_repository;
    Result<AuthorDTO> handleImpl(QPromise<Result<void>> &progressPromise, const GetAuthorQuery &query);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Author::Queries
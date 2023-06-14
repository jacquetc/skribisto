#pragma once

#include "application_author_export.h"
#include "author/author_dto.h"

#include "persistence/interface_author_repository.h"
#include <QPromise>

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;

namespace Application::Features::Author::Queries
{
class SKRIBISTO_APPLICATION_AUTHOR_EXPORT GetAllAuthorQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetAllAuthorQueryHandler(QSharedPointer<InterfaceAuthorRepository> repository);
    Result<QList<AuthorDTO>> handle(QPromise<Result<void>> &progressPromise);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
    Result<QList<AuthorDTO>> handleImpl(QPromise<Result<void>> &progressPromise);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Author::Queries
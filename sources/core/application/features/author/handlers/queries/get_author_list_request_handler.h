#pragma once

#include "application_global.h"
#include "dto/author/author_dto.h"
#include "handler.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;

namespace Application::Features::Author::Queries
{
class SKR_APPLICATION_EXPORT GetAuthorListRequestHandler : public Handler
{
    Q_OBJECT
  public:
    GetAuthorListRequestHandler(QSharedPointer<InterfaceAuthorRepository> repository);
    Result<QList<AuthorDTO>> handle(QPromise<Result<void> > &progressPromise);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
    Result<QList<AuthorDTO>> handleImpl();
};
} // namespace Application::Features::Author::Queries

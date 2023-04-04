#pragma once

#include "application_global.h"
#include "cqrs/author/requests/get_author_request.h"
#include "dto/author/author_dto.h"
#include "handler.h"
#include "persistence/interface_author_repository.h"

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Requests;

namespace Application::Features::Author::Queries
{
class SKR_APPLICATION_EXPORT GetAuthorRequestHandler : public Handler
{
    Q_OBJECT
  public:
    GetAuthorRequestHandler(QSharedPointer<InterfaceAuthorRepository> repository);
    Result<AuthorDTO> handle(QPromise<Result<void>> &progressPromise, const GetAuthorRequest &request);

  private:
    QSharedPointer<InterfaceAuthorRepository> m_repository;
    Result<AuthorDTO> handleImpl(const GetAuthorRequest &request);
};
} // namespace Application::Features::Author::Queries

#pragma once

#include "application_author_export.h"
#include "author/author_dto.h"
#include "author/commands/remove_author_command.h"

#include "persistence/interface_author_repository.h"
#include "result.h"
#include <QPromise>

using namespace Contracts::DTO::Author;
using namespace Contracts::Persistence;
using namespace Contracts::CQRS::Author::Commands;

namespace Application::Features::Author::Commands
{
class SKRIBISTO_APPLICATION_AUTHOR_EXPORT RemoveAuthorCommandHandler : public QObject
{
    Q_OBJECT
  public:
    RemoveAuthorCommandHandler(InterfaceAuthorRepository *repository);
    Result<int> handle(QPromise<Result<void>> &progressPromise, const RemoveAuthorCommand &request);
    Result<int> restore();

  signals:
    void authorCreated(Contracts::DTO::Author::AuthorDTO authorDto);
    void authorRemoved(int id);

  private:
    InterfaceAuthorRepository *m_repository;
    Result<int> handleImpl(QPromise<Result<void>> &progressPromise, const RemoveAuthorCommand &request);
    Result<int> restoreImpl();
    Domain::Author m_oldState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Application::Features::Author::Commands
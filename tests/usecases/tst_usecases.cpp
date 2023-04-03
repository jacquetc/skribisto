#include "automapper/automapper.h"
#include "dto/author/author_dto.h"
#include "dummy_author_repository.h"
#include "features/author/handlers/commands/create_author_command_handler.h"
#include "features/author/handlers/commands/remove_author_command_handler.h"
#include "features/author/handlers/queries/get_author_request_handler.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Application::Features::Author::Queries;
using namespace Application::Features::Author::Commands;
using namespace Contracts::DTO::Author;

class UseCases : public QObject
{
    Q_OBJECT

  public:
    UseCases();
    ~UseCases();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // void createTag();
    void getAuthor();
    void addAuthor();
    void removeAuthor();

  private:
};

UseCases::UseCases()
{
}

UseCases::~UseCases()
{
}

void UseCases::initTestCase()
{
}

void UseCases::cleanupTestCase()
{
}

void UseCases::init()
{
}

void UseCases::cleanup()
{
}

// ----------------------------------------------------------

void UseCases::getAuthor()
{

    QSharedPointer<DummyAuthorRepository> repository(new DummyAuthorRepository(this));

    QUuid uuid = QUuid::createUuid();
    QUuid relative = QUuid::createUuid();
    Domain::Author author(1, uuid, "test", relative);
    repository->fillGet(author);

    GetAuthorRequestHandler handler(repository);
    GetAuthorRequest request;
    request.id = uuid;

    Result<AuthorDTO> dtoResult = handler.handle(request);
    if (!dtoResult)
    {
        qDebug() << dtoResult.error().message() << dtoResult.error().data();
    }
    QVERIFY(dtoResult.isSuccess());

    QCOMPARE(dtoResult.value().id(), 1);
    QCOMPARE(dtoResult.value().getName(), "test");
    QCOMPARE(dtoResult.value().getRelative(), relative);
}

// ----------------------------------------------------------

// ----------------------------------------------------------

void UseCases::addAuthor()
{
    QSharedPointer<DummyAuthorRepository> repository(new DummyAuthorRepository(this));

    // Create an AuthorDTO to add
    CreateAuthorDTO dto;
    dto.setName("new author");
    dto.setRelative(QUuid::createUuid());

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    author.setUuid(QUuid::createUuid());
    repository->fillAdd(author);
    repository->fillGetAll(QList<Domain::Author>() << author);

    // Invoke the CreateAuthorCommandHandler with the DTO
    CreateAuthorCommandHandler handler(repository);
    CreateAuthorCommand command;
    command.req = dto;

    Result<AuthorDTO> result = handler.handle(command);

    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }
    QVERIFY(result.isSuccess());
}
// ----------------------------------------------------------

void UseCases::removeAuthor()
{
    QSharedPointer<DummyAuthorRepository> repository(new DummyAuthorRepository(this));

    // Add an author to the repository
    AuthorDTO dto;
    dto.setUuid(QUuid::createUuid());
    dto.setName("test");
    dto.setRelative(QUuid::createUuid());
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    repository->fillRemove(author);
    repository->fillGet(author);

    // Remove the author
    RemoveAuthorCommandHandler handler(repository);
    RemoveAuthorCommand command;
    command.id = author.uuid();

    Result<AuthorDTO> result = handler.handle(command);
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }
    QVERIFY(result.isSuccess());
}

QTEST_MAIN(UseCases)

#include "tst_usecases.moc"

#include "dto/author/author_dto.h"
#include "dummy_database.h"
#include "repositories/author_repository.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

class RepositoryTest : public QObject
{
    Q_OBJECT

  public:
    RepositoryTest();
    ~RepositoryTest();

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
    void updateAuthor();
    void authorExists();
    void getAllAuthors();

  private:
};

RepositoryTest::RepositoryTest()
{
}

RepositoryTest::~RepositoryTest()
{
}

void RepositoryTest::initTestCase()
{
}

void RepositoryTest::cleanupTestCase()
{
}

void RepositoryTest::init()
{
}

void RepositoryTest::cleanup()
{
}

// ----------------------------------------------------------

void RepositoryTest::getAuthor()
{

    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    QUuid uuid = QUuid::createUuid();
    QUuid relative = QUuid::createUuid();
    Domain::Author author(uuid, "test", relative);
    database->fillGet(author);

    Result<Domain::Author> result = repository.get(uuid);

    QCOMPARE(result.value().name(), "test");
    QCOMPARE(result.value().relative(), relative);
}

// ----------------------------------------------------------

void RepositoryTest::addAuthor()
{
    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    QUuid uuid = QUuid::createUuid();
    QUuid relative = QUuid::createUuid();
    Domain::Author author(uuid, "test", relative);
    database->fillAdd(author);

    Result<Domain::Author> result = repository.add(std::move(author));

    QVERIFY(result.isSuccess());
    QCOMPARE(result.value().name(), "test");
    QCOMPARE(result.value().relative(), relative);
}

void RepositoryTest::removeAuthor()
{
    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    QUuid uuid = QUuid::createUuid();
    Domain::Author author(uuid, "test", QUuid::createUuid());
    database->fillRemove(author);

    Result<Domain::Author> result = repository.remove(std::move(author));
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }

    QVERIFY(result);
    QCOMPARE(result.value().name(), "test");
}

void RepositoryTest::updateAuthor()
{
    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    QUuid uuid = QUuid::createUuid();
    QUuid relative = QUuid::createUuid();
    Domain::Author author(uuid, "test", relative);
    database->fillUpdate(author);

    Result<Domain::Author> result = repository.update(std::move(author));
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }
    QVERIFY(result.isSuccess());
    QCOMPARE(result.value().name(), "test");
    QCOMPARE(result.value().relative(), relative);
}

void RepositoryTest::authorExists()
{
    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    QUuid uuid = QUuid::createUuid();
    database->fillExists(true);

    Result<bool> result = repository.exists(uuid);
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }
    QVERIFY(result.isSuccess());
    QVERIFY(result.value());
}

void RepositoryTest::getAllAuthors()
{
    auto database = new DummyDatabase;
    Repository::AuthorRepository repository(database);

    Domain::Author author1(QUuid::createUuid(), "test1", QUuid::createUuid());
    Domain::Author author2(QUuid::createUuid(), "test2", QUuid::createUuid());
    QList<Domain::Author> expected = {author1, author2};
    database->fillGetAll(expected);

    Result<QList<Domain::Author>> result = repository.getAll();
    if (!result)
    {
        qDebug() << result.error().message() << result.error().data();
    }
    QVERIFY(result.isSuccess());
    QCOMPARE(result.value().size(), 2);
    QCOMPARE(result.value()[0].name(), "test1");
    QCOMPARE(result.value()[1].name(), "test2");
}
QTEST_MAIN(RepositoryTest)

#include "tst_repository.moc"

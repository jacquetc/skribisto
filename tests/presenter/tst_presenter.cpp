#include "QtConcurrent/qtconcurrentrun.h"
#include "author/author_controller.h"
#include "author_dto.h"
#include "automapper/automapper.h"
#include "dummy_author_repository.h"
#include "dummy_repository_provider.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Presenter::Author;
using namespace Contracts::DTO::Author;

class PresenterTest : public QObject {
    Q_OBJECT

public:

    PresenterTest();
    ~PresenterTest();

public slots:

private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void getAuthorAsync();
    void getAuthorAsync_aLot();
    void addAuthorAsync();
    void addAuthorAsync_TwoRapid();

private:

    DummyRepositoryProvider *m_repositoryProvider;
    QSharedPointer<DummyAuthorRepository>m_repository;
    AuthorController *m_authorController;
};

PresenterTest::PresenterTest()
{
    m_repositoryProvider = new DummyRepositoryProvider(this);
}

PresenterTest::~PresenterTest()
{}

void PresenterTest::initTestCase()
{
    m_repository.reset(new DummyAuthorRepository(this));
    m_repositoryProvider->registerRepository(DummyRepositoryProvider::Author, m_repository);

    Scopes scopes(QStringList() << "author");
    new UndoRedo::ThreadedUndoRedoSystem(this, scopes);
    m_authorController = new AuthorController(m_repositoryProvider);

    AutoMapper::AutoMapper::registerMapping<Domain::Author, AuthorDTO>(true);
    AutoMapper::AutoMapper::registerMapping<CreateAuthorDTO, Domain::Author>();
}

void PresenterTest::cleanupTestCase()
{
}

void PresenterTest::init()
{
}

void PresenterTest::cleanup()
{
}

// ----------------------------------------------------------

void PresenterTest::getAuthorAsync()
{
    // Create an AuthorDTO to add
    AuthorDTO dto;

    dto.setId(1);
    dto.setUuid(QUuid::createUuid());
    dto.setName("new author");

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<AuthorDTO, Domain::Author>(dto);
    m_repository->fillGet(author);

    // invoke
    QSignalSpy spy(m_authorController, &AuthorController::getReplied);
    QVERIFY(spy.isValid());

    AuthorController::get(dto.id());

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(),         1);
    QList<QVariant> arguments = spy.takeFirst();
    auto signalResult         = arguments.at(0).value<AuthorDTO>();
    QCOMPARE(signalResult.uuid(), dto.uuid());
}

// ----------------------------------------------------------

void PresenterTest::getAuthorAsync_aLot()
{
    // Create an AuthorDTO to add
    AuthorDTO dto;

    dto.setId(1);
    dto.setUuid(QUuid::createUuid());
    dto.setName("new author");

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<AuthorDTO, Domain::Author>(dto);
    m_repository->fillGet(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::getReplied);
    QVERIFY(spy.isValid());
    QBENCHMARK_ONCE
    {
        for (int i = 1; i <= 100; i++)
        {
            AuthorController::get(dto.id());
        }
        QTest::qWait(500);
    }
    QCOMPARE(spy.count(), 100);

    QList<QVariant> arguments = spy.takeFirst();
    auto signalResult         = arguments.at(0).value<AuthorDTO>();
    QVERIFY(signalResult.uuid() == dto.uuid());
}

// ----------------------------------------------------------

void PresenterTest::addAuthorAsync()
{
    // Create an AuthorDTO to add
    CreateAuthorDTO dto;

    dto.setName("new author");

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<CreateAuthorDTO, Domain::Author>(dto);
    author.setUuid(QUuid::createUuid());
    m_repository->fillAdd(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::authorCreated);
    QVERIFY(spy.isValid());

    AuthorController::create(dto);

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments    = spy.takeFirst();
    Result<QUuid>   signalResult = arguments.at(0).value<Result<QUuid> >();

    if (!signalResult)
    {
        qDebug() << signalResult.error().message() << signalResult.error().data();
    }

    QVERIFY(signalResult.isSuccess());
}

// ----------------------------------------------------------

void PresenterTest::addAuthorAsync_TwoRapid()
{
    // Create an AuthorDTO to add
    CreateAuthorDTO dto;

    dto.setName("new author");

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<CreateAuthorDTO, Domain::Author>(dto);
    author.setUuid(QUuid::createUuid());
    m_repository->fillAdd(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::authorCreated);
    QVERIFY(spy.isValid());

    m_authorController->create(dto);
    m_authorController->create(dto);

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 1);
    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 2);
    QList<QVariant> arguments    = spy.takeFirst();
    Result<QUuid>   signalResult = arguments.at(0).value<Result<QUuid> >();

    if (!signalResult)
    {
        qDebug() << signalResult.error().message() << signalResult.error().data();
    }

    QVERIFY(signalResult.isSuccess());
}

QTEST_GUILESS_MAIN(PresenterTest)

#include "tst_presenter.moc"

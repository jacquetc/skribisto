#include "QtConcurrent/qtconcurrentrun.h"
#include "author/author_controller.h"
#include "automapper/automapper.h"
#include "dto/author/author_dto.h"
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

class PresenterTest : public QObject
{
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
    QSharedPointer<DummyAuthorRepository> m_repository;
    AuthorController *m_authorController;
};

PresenterTest::PresenterTest()
{
    m_repositoryProvider = new DummyRepositoryProvider(this);
}

PresenterTest::~PresenterTest()
{
}

void PresenterTest::initTestCase()
{
    m_repository.reset(new DummyAuthorRepository(this));
    m_repositoryProvider->registerRepository(DummyRepositoryProvider::Author, m_repository);
    new UndoRedo::ThreadedUndoRedoSystem(this);
    m_authorController = new AuthorController(m_repositoryProvider);
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
    dto.setUuid(QUuid::createUuid());
    dto.setName("new author");
    dto.setRelative(QUuid::createUuid());

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    m_repository->fillGet(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::getAuthorReplied);
    QVERIFY(spy.isValid());

    AuthorController::getAsync(dto.uuid());

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments = spy.takeFirst();
    Result<AuthorDTO> signalResult = arguments.at(0).value<Result<AuthorDTO>>();
    if (!signalResult)
    {
        qDebug() << signalResult.error().message() << signalResult.error().data();
    }

    QVERIFY(signalResult.isSuccess());
    QVERIFY(signalResult.value().uuid() == dto.uuid());
}
// ----------------------------------------------------------

void PresenterTest::getAuthorAsync_aLot()
{
    // Create an AuthorDTO to add
    AuthorDTO dto;
    dto.setUuid(QUuid::createUuid());
    dto.setName("new author");
    dto.setRelative(QUuid::createUuid());

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    m_repository->fillGet(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::getAuthorReplied);
    QVERIFY(spy.isValid());
    QBENCHMARK_ONCE
    {
        for (int i = 0; i < 30; i++)
            AuthorController::getAsync(dto.uuid());

        for (int i = 0; i < 30; i++)
        {
            QVERIFY(spy.wait(5000));
        }
    }
    QCOMPARE(spy.count(), 30);

    QList<QVariant> arguments = spy.takeFirst();
    Result<AuthorDTO> signalResult = arguments.at(0).value<Result<AuthorDTO>>();
    if (!signalResult)
    {
        qDebug() << signalResult.error().message() << signalResult.error().data();
    }

    QVERIFY(signalResult.isSuccess());
    QVERIFY(signalResult.value().uuid() == dto.uuid());
}
// ----------------------------------------------------------

void PresenterTest::addAuthorAsync()
{
    // Create an AuthorDTO to add
    CreateAuthorDTO dto;
    dto.setName("new author");
    dto.setRelative(QUuid::createUuid());

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    author.setUuid(QUuid::createUuid());
    m_repository->fillAdd(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::authorCreated);
    QVERIFY(spy.isValid());

    AuthorController::createAsync(dto);

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 1);
    QList<QVariant> arguments = spy.takeFirst();
    Result<QUuid> signalResult = arguments.at(0).value<Result<QUuid>>();
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
    dto.setRelative(QUuid::createUuid());

    // prefill the dummy repo:
    auto author = AutoMapper::AutoMapper::map<Domain::Author>(dto);
    author.setUuid(QUuid::createUuid());
    m_repository->fillAdd(author);

    // invoke

    QSignalSpy spy(m_authorController, &AuthorController::authorCreated);
    QVERIFY(spy.isValid());

    m_authorController->createAsync(dto);
    m_authorController->createAsync(dto);

    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 1);
    QVERIFY(spy.wait(5000));
    QCOMPARE(spy.count(), 2);
    QList<QVariant> arguments = spy.takeFirst();
    Result<QUuid> signalResult = arguments.at(0).value<Result<QUuid>>();
    if (!signalResult)
    {
        qDebug() << signalResult.error().message() << signalResult.error().data();
    }

    QVERIFY(signalResult.isSuccess());
}

QTEST_GUILESS_MAIN(PresenterTest)

#include "tst_presenter.moc"

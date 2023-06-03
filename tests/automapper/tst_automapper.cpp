#include "author.h"
#include "author_dto.h"
#include "automapper/automapper.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Contracts::DTO::Author;

class AutoMapperTest : public QObject
{
    Q_OBJECT

  public:
    AutoMapperTest();
    ~AutoMapperTest();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void basicMap();
    void invertedMap();

  private:
};

AutoMapperTest::AutoMapperTest()
{
}

AutoMapperTest::~AutoMapperTest()
{
}

void AutoMapperTest::initTestCase()
{
}

void AutoMapperTest::cleanupTestCase()
{
}

void AutoMapperTest::init()
{
}

void AutoMapperTest::cleanup()
{
}

// ----------------------------------------------------------

void AutoMapperTest::basicMap()
{

    QUuid uuid = QUuid::createUuid();
    Domain::Author author(1, uuid, QDateTime(), QDateTime(), "e");

    AuthorDTO dto = AutoMapper::AutoMapper::map<AuthorDTO>(author);

    QCOMPARE(dto.getName(), "e");
    QCOMPARE(dto.uuid(), uuid);
}
// ----------------------------------------------------------

void AutoMapperTest::invertedMap()
{

    QUuid uuid = QUuid::createUuid();
    AuthorDTO dto(1, uuid, "e");

    Domain::Author author = AutoMapper::AutoMapper::map<Domain::Author>(dto);

    QCOMPARE(author.id(), 1);
    QCOMPARE(author.uuid(), uuid);
    QCOMPARE(author.name(), "e");
}

QTEST_MAIN(AutoMapperTest)

#include "tst_automapper.moc"


#include "automapper/automapper.h"
#include "dummy_basic_entity.h"
#include "dummy_basic_entity_dto.h"
#include "dummy_entity_with_details.h"
#include "dummy_entity_with_details_dto.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

using namespace Contracts::DTO::DummyBasicEntity;
using namespace Contracts::DTO::DummyEntityWithDetails;

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
    Domain::DummyBasicEntity author(1, uuid, QDateTime(), "e");

    DummyBasicEntityDTO dto = AutoMapper::AutoMapper::map<DummyBasicEntityDTO, Domain::DummyBasicEntity>(author);

    QCOMPARE(dto.name(), "e");
    QCOMPARE(dto.uuid(), uuid);
}
// ----------------------------------------------------------

void AutoMapperTest::invertedMap()
{

    QUuid uuid = QUuid::createUuid();
    DummyBasicEntityDTO dto(1, uuid, QDateTime(), "e");

    Domain::DummyBasicEntity author = AutoMapper::AutoMapper::map<Domain::DummyBasicEntity, DummyBasicEntityDTO>(dto);

    QCOMPARE(author.id(), 1);
    QCOMPARE(author.uuid(), uuid);
    QCOMPARE(author.name(), "e");
}

QTEST_MAIN(AutoMapperTest)

#include "tst_automapper.moc"

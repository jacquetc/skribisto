
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

class AutoMapperTest : public QObject {
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
    void mapWithForeignEntity();

    void mapWithForeignEntityInverted();

    void mapWithUniqueForeignEntity();

private:
};

AutoMapperTest::AutoMapperTest()
{}

AutoMapperTest::~AutoMapperTest()
{}

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

    AutoMapper::AutoMapper::registerMapping<Domain::DummyBasicEntity, DummyBasicEntityDTO>();

    DummyBasicEntityDTO dto = AutoMapper::AutoMapper::map<Domain::DummyBasicEntity, DummyBasicEntityDTO>(author);

    QCOMPARE(dto.name(), "e");
    QCOMPARE(dto.uuid(), uuid);
}

// ----------------------------------------------------------

void AutoMapperTest::invertedMap()
{
    QUuid uuid = QUuid::createUuid();
    DummyBasicEntityDTO dto(1, uuid, QDateTime(), "e");

    // reversed registering:
    AutoMapper::AutoMapper::registerMapping<Domain::DummyBasicEntity, DummyBasicEntityDTO>(true);

    Domain::DummyBasicEntity author = AutoMapper::AutoMapper::map<DummyBasicEntityDTO, Domain::DummyBasicEntity>(dto);

    QCOMPARE(author.id(),   1);
    QCOMPARE(author.uuid(), uuid);
    QCOMPARE(author.name(), "e");
}

// ----------------------------------------------------------


void AutoMapperTest::mapWithForeignEntity()
{
    QUuid uuid = QUuid::createUuid();
    Domain::DummyBasicEntity author(1, uuid, QDateTime(), "detail");
    QUuid uuid2 = QUuid::createUuid();
    Domain::DummyEntityWithDetails entity(1, uuid2, QDateTime(), "entity_with_detail",
                                          QList<Domain::DummyBasicEntity>() << author, Domain::DummyBasicEntity());

    AutoMapper::AutoMapper::registerMapping<Domain::DummyBasicEntity, DummyBasicEntityDTO>();
    AutoMapper::AutoMapper::registerMapping<Domain::DummyEntityWithDetails, DummyEntityWithDetailsDTO>();

    DummyEntityWithDetailsDTO dto = AutoMapper::AutoMapper::map<Domain::DummyEntityWithDetails,
                                                                DummyEntityWithDetailsDTO>(entity);

    QCOMPARE(dto.name(),                   "entity_with_detail");
    QCOMPARE(dto.uuid(),                   uuid2);
    QCOMPARE(dto.details().size(),         1);
    QCOMPARE(dto.details().first().name(), "detail");
    QCOMPARE(dto.details().first().uuid(), uuid);
}

void AutoMapperTest::mapWithForeignEntityInverted()
{
    QUuid uuid = QUuid::createUuid();
    DummyBasicEntityDTO author(1, uuid, QDateTime(), "detail");
    QUuid uuid2 = QUuid::createUuid();
    DummyEntityWithDetailsDTO dto(1, uuid2, QDateTime(), "entity_with_detail",
                                  QList<DummyBasicEntityDTO>() << author, DummyBasicEntityDTO());

    AutoMapper::AutoMapper::registerMapping<Domain::DummyBasicEntity, DummyBasicEntityDTO>(            true);
    AutoMapper::AutoMapper::registerMapping<Domain::DummyEntityWithDetails, DummyEntityWithDetailsDTO>(true);

    Domain::DummyEntityWithDetails entity = AutoMapper::AutoMapper::map<DummyEntityWithDetailsDTO,
                                                                        Domain::DummyEntityWithDetails>(dto);

    QCOMPARE(entity.name(),                   "entity_with_detail");
    QCOMPARE(entity.uuid(),                   uuid2);
    QCOMPARE(entity.details().size(),         1);
    QCOMPARE(entity.details().first().name(), "detail");
    QCOMPARE(entity.details().first().uuid(), uuid);
}

void AutoMapperTest::mapWithUniqueForeignEntity() {
    QUuid uuid = QUuid::createUuid();
    Domain::DummyBasicEntity author(1, uuid, QDateTime(), "detail");
    QUuid uuid2 = QUuid::createUuid();
    Domain::DummyEntityWithDetails entity(1, uuid2, QDateTime(), "entity_with_detail",
                                          QList<Domain::DummyBasicEntity>(),
                                          author);

    AutoMapper::AutoMapper::registerMapping<Domain::DummyBasicEntity, DummyBasicEntityDTO>();
    AutoMapper::AutoMapper::registerMapping<Domain::DummyEntityWithDetails, DummyEntityWithDetailsDTO>();

    DummyEntityWithDetailsDTO dto = AutoMapper::AutoMapper::map<Domain::DummyEntityWithDetails,
                                                                DummyEntityWithDetailsDTO>(entity);

    QCOMPARE(dto.name(),                "entity_with_detail");
    QCOMPARE(dto.uuid(),                uuid2);
    QCOMPARE(dto.details().size(),      0);
    QCOMPARE(dto.uniqueDetail().name(), "detail");
    QCOMPARE(dto.uniqueDetail().uuid(), uuid);
}

QTEST_MAIN(AutoMapperTest)

#include "tst_automapper.moc"

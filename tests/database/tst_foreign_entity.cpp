#include "database/database_table.h"
#include "database/tools.h"
#include "dummy_database_context.h"
#include "dummy_entity_with_foreign.h"
#include "dummy_other_entity.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

class ForeignEntityTest : public QObject
{
    Q_OBJECT

  public:
    ForeignEntityTest();
    ~ForeignEntityTest();

  public slots:

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testListAddRelationship();

  private:
    Database::DatabaseTable<Domain::DummyEntityWithForeign> *m_entityTable;
    Database::DatabaseTable<Domain::DummyOtherEntity> *m_otherEntityTable;
};

ForeignEntityTest::ForeignEntityTest()
{
}

ForeignEntityTest::~ForeignEntityTest()
{
}

void ForeignEntityTest::initTestCase()
{
    DummyDatabaseContext<Domain::DummyEntityWithForeign, Domain::DummyOtherEntity> *context =
        new DummyDatabaseContext<Domain::DummyEntityWithForeign, Domain::DummyOtherEntity>();
    context->setEntityClassNames(QStringList() << "DummyEntityWithForeign"
                                               << "DummyOtherEntity");
    context->init();

    m_entityTable = new Database::DatabaseTable<Domain::DummyEntityWithForeign>(context);
    m_otherEntityTable = new Database::DatabaseTable<Domain::DummyOtherEntity>(context);
}

void ForeignEntityTest::cleanupTestCase()
{
}

void ForeignEntityTest::init()
{
}

void ForeignEntityTest::cleanup()
{
    m_entityTable->clear();
}

void ForeignEntityTest::testListAddRelationship()
{
    // setup table

    Domain::DummyEntityWithForeign entity;
    entity.setName("Sample DummyEntityWithForeign");
    entity.setUuid(QUuid::createUuid());
    entity.setCreationDate(QDateTime::currentDateTime());
    entity.setUpdateDate(QDateTime::currentDateTime());

    //    // setup the lazy loading :
    //    entity.setListsLoader([this, &entity]() {
    //        QList<int> otherIds =
    //            m_entityTable
    //                ->getRelatedEntityIds(entity.id(),
    //                Database::Tools<Domain::DummyOtherEntity>::getEntityClassName()) .value();

    //        QList<Domain::DummyOtherEntity> otherEntities;
    //        for (int otherId : otherIds)
    //        {
    //            otherEntities.append(m_otherEntityTable->get(otherId).value());
    //        }

    //        return otherEntities;
    //    });

    auto addResult = m_entityTable->add(std::move(entity));
    if (addResult.isError())
    {
        qDebug() << addResult.error().code() << addResult.error().message() << addResult.error().data();
    }
    QVERIFY(addResult.isSuccess());

    int entityId = addResult.value().id();

    // setup other table

    Domain::DummyOtherEntity otherEntity;
    otherEntity.setName("Sample DummyOtherEntity");
    otherEntity.setUuid(QUuid::createUuid());
    otherEntity.setCreationDate(QDateTime::currentDateTime());
    otherEntity.setUpdateDate(QDateTime::currentDateTime());

    auto addOtherResult = m_otherEntityTable->add(std::move(otherEntity));
    if (addOtherResult.isError())
    {
        qDebug() << addOtherResult.error().code() << addOtherResult.error().message() << addOtherResult.error().data();
    }
    QVERIFY(addOtherResult.isSuccess());

    int otherEntityId = addOtherResult.value().id();

    // add relationship :

    auto addRelationshipResult = m_entityTable->addEntityRelationship(
        entityId, otherEntityId, Database::Tools<Domain::DummyOtherEntity>::getEntityClassName());
    QVERIFY(addRelationshipResult.isSuccess());
}

QTEST_APPLESS_MAIN(ForeignEntityTest)

#include "tst_foreign_entity.moc"

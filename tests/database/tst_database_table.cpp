#include "database/database_table.h"
#include "dummy_basic_entity.h"
#include "dummy_database_context.h"
#include <QtTest/QtTest>

class TestDatabaseTable : public QObject
{
    Q_OBJECT

  private slots:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testAdd();
    void testRemove();

  private:
    Database::DatabaseTable<Domain::DummyBasicEntity> *m_entityTable;
};

void TestDatabaseTable::initTestCase()
{
    DummyDatabaseContext<Domain::DummyBasicEntity, Domain::DummyBasicEntity> *context =
        new DummyDatabaseContext<Domain::DummyBasicEntity, Domain::DummyBasicEntity>();
    context->setEntityClassNames(QStringList() << "DummyEntity");
    context->init();
    m_entityTable = new Database::DatabaseTable<Domain::DummyBasicEntity>(context);
}

void TestDatabaseTable::cleanupTestCase()
{
}

void TestDatabaseTable::init()
{
}

void TestDatabaseTable::cleanup()
{
    m_entityTable->clear();
}
void TestDatabaseTable::testAdd()
{

    Domain::DummyBasicEntity entity;
    entity.setName("Sample DummyEntity");
    entity.setUuid(QUuid::createUuid());
    entity.setCreationDate(QDateTime::currentDateTime());
    auto addResult = m_entityTable->add(std::move(entity));
    if (addResult.isError())
    {
        qDebug() << addResult.error().code() << addResult.error().message() << addResult.error().data();
    }
    QVERIFY(addResult.isSuccess());

    // Verify the entity is added
    auto entitiesResult = m_entityTable->getAll();
    if (entitiesResult.isError())
    {
        qDebug() << entitiesResult.error().code() << entitiesResult.error().message() << entitiesResult.error().data();
    }
    QVERIFY(entitiesResult.isSuccess());

    auto entities = entitiesResult.value();
    QCOMPARE(entities.size(), 1);
    QCOMPARE(entities.first().name(), QString("Sample DummyEntity"));
    QVERIFY(entities.first().creationDate().isValid());
}

void TestDatabaseTable::testRemove()
{
    Domain::DummyBasicEntity entity;
    entity.setName("Sample DummyEntity");
    entity.setUuid(QUuid::createUuid());
    auto addResult = m_entityTable->add(std::move(entity));
    QVERIFY(addResult.isSuccess());

    // Verify the entity is added
    auto entities = m_entityTable->getAll().value();
    QCOMPARE(entities.size(), 1);
    QCOMPARE(entities.first().name(), QString("Sample DummyEntity"));

    // remove the entity

    auto removeResult = m_entityTable->remove(std::move(addResult.value()));
    QVERIFY(removeResult.isSuccess());

    // Verify the entity is removed
    auto entities2 = m_entityTable->getAll().value();
    QCOMPARE(entities2.size(), 0);
}
QTEST_APPLESS_MAIN(TestDatabaseTable)
#include "tst_database_table.moc"

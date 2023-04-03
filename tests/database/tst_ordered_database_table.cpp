#include "database/ordered_database_table.h"
#include "dummy_database_context.h"
#include "dummy_entity.h"
#include "dummy_ordered_database_table.h"
#include <QDate>
#include <QDateTime>
#include <QDebug>
#include <QHash>
#include <QList>
#include <QTime>
#include <QVariant>
#include <QtTest>

class OrderedDatabaseTableTest : public QObject
{
    Q_OBJECT

  public:
    OrderedDatabaseTableTest();
    ~OrderedDatabaseTableTest();

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testGetAll();
    void testGetAllWithFilter();
    void testGetAllWithFilter_disordoned();
    void testGetFuturePreviousAndNextIds();
    void testGetPreviousAndNextIds();
    void testInsert();
    void testFilteredInsert();
    void testGetAllOrderedUuid();
    void testGetAllOrderedUuidWithFilter();
    void testGetAllOrderedUuidWithFilter_disordoned();
    void testRemove();

  private:
    Database::DummyOrderedDatabaseTable *m_entityTable;
    void debugOrderingTable();
    void printTableContents();
};

OrderedDatabaseTableTest::OrderedDatabaseTableTest()
{
}

OrderedDatabaseTableTest::~OrderedDatabaseTableTest()
{
}

void OrderedDatabaseTableTest::initTestCase()
{
    DummyDatabaseContext *context = new DummyDatabaseContext();
    m_entityTable = new Database::DummyOrderedDatabaseTable(context);
}

void OrderedDatabaseTableTest::cleanupTestCase()
{
}

void OrderedDatabaseTableTest::init()
{
}

void OrderedDatabaseTableTest::cleanup()
{
    m_entityTable->clear();
}

// ----------------------------------------------------------
void OrderedDatabaseTableTest::testGetAll()
{
    // Test add when the table is empty
    Domain::DummyEntity entity1;
    entity1.setUuid(QUuid::createUuid());
    entity1.setName("Sample DummyEntity 1");
    entity1.setCreationDate(QDateTime::currentDateTime());
    entity1.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult1 = m_entityTable->add(std::move(entity1));
    if (insertResult1.isError())
    {
        qDebug() << insertResult1.error().code() << insertResult1.error().message();
    }
    QVERIFY(insertResult1.isSuccess());

    // Verify the entity is inserted at position 0
    auto entities1 = m_entityTable->getAll();
    if (entities1.isError())
    {
        qDebug() << entities1.error().code() << entities1.error().message() << entities1.error().data();
    }
    QVERIFY(entities1.isSuccess());
    QCOMPARE(entities1.value().size(), 1);
    QCOMPARE(entities1.value().first().name(), QString("Sample DummyEntity 1"));
}
// ----------------------------------------------------------
void OrderedDatabaseTableTest::testGetAllOrderedUuid()
{
    // Set up the table with some initial data
    for (int i = 1; i <= 3; ++i)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    // Call getAllOrderedUuid
    auto uuidResult = m_entityTable->getAllOrderedUuid();
    if (uuidResult.isError())
    {
        qDebug() << uuidResult.error().code() << uuidResult.error().message() << uuidResult.error().data();
    }
    QVERIFY(uuidResult.isSuccess());
    QList<QUuid> uuidList = uuidResult.value();

    // Verify the number of UUIDs
    QCOMPARE(uuidList.size(), 3);

    // Get all entities using getAll
    auto entitiesResult = m_entityTable->getAll();
    QVERIFY(entitiesResult.isSuccess());
    QList<Domain::DummyEntity> entities = entitiesResult.value();

    // Verify the order of UUIDs
    for (int i = 0; i < entities.size(); ++i)
    {
        QCOMPARE(uuidList.at(i), entities.at(i).uuid());
    }

    // clear
    m_entityTable->clear();

    // Set up the table with some initial data in a disordered way
    QVector<int> order = {3, 1, 2};
    for (int i : order)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    // Call getAllOrderedUuid
    uuidResult = m_entityTable->getAllOrderedUuid();
    QVERIFY(uuidResult.isSuccess());
    QList<QUuid> uuidList2 = uuidResult.value();

    // Verify the number of UUIDs
    QCOMPARE(uuidList2.size(), 3);

    // Get all entities using getAll
    entitiesResult = m_entityTable->getAll();
    QVERIFY(entitiesResult.isSuccess());
    QList<Domain::DummyEntity> entities2 = entitiesResult.value();

    // Verify the order of UUIDs
    for (int i = 0; i < entities2.size(); ++i)
    {
        QCOMPARE(uuidList2.at(i), entities2.at(i).uuid());
    }
}

void OrderedDatabaseTableTest::testGetAllOrderedUuidWithFilter()
{
    // Set up the table with some initial data
    for (int i = 1; i <= 6; ++i)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    // Test getAllOrderedUuid with a filter for "Author A"
    QHash<QString, QVariant> filter1;
    filter1.insert("author", "Author A");
    auto filteredUuidsResult1 = m_entityTable->getAllOrderedUuid(filter1);
    if (filteredUuidsResult1.isError())
    {
        qDebug() << filteredUuidsResult1.error().code() << filteredUuidsResult1.error().message();
    }
    QVERIFY(!filteredUuidsResult1.isError());
    auto filteredUuids1 = filteredUuidsResult1.value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids1.size(), 3);

    // Test getAllOrderedUuid with a filter for "Author B"
    QHash<QString, QVariant> filter2;
    filter2.insert("author", "Author B");
    auto filteredUuids2 = m_entityTable->getAllOrderedUuid(filter2).value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids2.size(), 3);

    // Test getAllOrderedUuid with a filter for a specific name
    QHash<QString, QVariant> filter3;
    filter3.insert("name", "Sample DummyEntity 3");
    auto filteredUuids3 = m_entityTable->getAllOrderedUuid(filter3).value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids3.size(), 1);

    // Test getAllOrderedUuid with a filter that has no matches
    QHash<QString, QVariant> filter4;
    filter4.insert("author", "Author C");
    auto filteredUuids4 = m_entityTable->getAllOrderedUuid(filter4).value();

    // Verify there are no UUIDs returned
    QCOMPARE(filteredUuids4.size(), 0);
}

void OrderedDatabaseTableTest::testGetAllOrderedUuidWithFilter_disordoned()
{
    // Set up the table with some initial data
    QVector<int> order = {4, 1, 6, 2, 5, 3};
    for (int i : order)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    // Test getAllOrderedUuid with a filter for "Author A"
    QHash<QString, QVariant> filter1;
    filter1.insert("author", "Author A");
    auto filteredUuidsResult1 = m_entityTable->getAllOrderedUuid(filter1);
    if (filteredUuidsResult1.isError())
    {
        qDebug() << filteredUuidsResult1.error().code() << filteredUuidsResult1.error().message();
    }
    QVERIFY(!filteredUuidsResult1.isError());
    auto filteredUuids1 = filteredUuidsResult1.value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids1.size(), 3);

    // Test getAllOrderedUuid with a filter for "Author B"
    QHash<QString, QVariant> filter2;
    filter2.insert("author", "Author B");
    auto filteredUuids2 = m_entityTable->getAllOrderedUuid(filter2).value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids2.size(), 3);

    // Test getAllOrderedUuid with a filter for a specific name
    QHash<QString, QVariant> filter3;
    filter3.insert("name", "Sample DummyEntity 3");
    auto filteredUuids3 = m_entityTable->getAllOrderedUuid(filter3).value();

    // Verify the number of filtered UUIDs is correct
    QCOMPARE(filteredUuids3.size(), 1);

    // Test getAllOrderedUuid with a filter that has no matches
    QHash<QString, QVariant> filter4;
    filter4.insert("author", "Author C");
    auto filteredUuids4 = m_entityTable->getAllOrderedUuid(filter4).value();

    // Verify there are no UUIDs returned
    QCOMPARE(filteredUuids4.size(), 0);
}

void OrderedDatabaseTableTest::testGetAllWithFilter()
{
    // Set up the table with some initial data
    for (int i = 1; i <= 6; ++i)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    auto entities1 = m_entityTable->getAll().value();
    qDebug() << "unordered list at entities1";
    for (const Domain::DummyEntity &entity : entities1)
    {
        qDebug() << entity.name() << entity.author();
    }

    qDebug() << ""; // new line
    // Test getAll with a filter for "Author A"
    QHash<QString, QVariant> filter1;
    filter1.insert("author", "Author A");
    auto filteredEntitiesResult1 = m_entityTable->getAll(filter1);
    if (filteredEntitiesResult1.isError())
    {
        qDebug() << filteredEntitiesResult1.error().code() << filteredEntitiesResult1.error().message();
    }

    auto filteredEntities1 = filteredEntitiesResult1.value();

    qDebug() << "ordered list at filteredEntities1";
    for (const Domain::DummyEntity &entity : filteredEntities1)
    {
        qDebug() << entity.name() << entity.author();
    }
    // Verify the filtered entities are correct
    QCOMPARE(filteredEntities1.size(), 3);
    for (const Domain::DummyEntity &entity : filteredEntities1)
    {
        QCOMPARE(entity.author(), QString("Author A"));
    }

    // Test getAll with a filter for "Author B"
    QHash<QString, QVariant> filter2;
    filter2.insert("author", "Author B");
    auto filteredEntities2 = m_entityTable->getAll(filter2).value();

    // Verify the filtered entities are correct
    QCOMPARE(filteredEntities2.size(), 3);
    for (const Domain::DummyEntity &entity : filteredEntities2)
    {
        QCOMPARE(entity.author(), QString("Author B"));
    }

    // Test getAll with a filter for a specific name
    QHash<QString, QVariant> filter3;
    filter3.insert("name", "Sample DummyEntity 3");
    auto filteredEntities3 = m_entityTable->getAll(filter3).value();

    // Verify the filtered entity is correct
    QCOMPARE(filteredEntities3.size(), 1);
    QCOMPARE(filteredEntities3.at(0).name(), QString("Sample DummyEntity 3"));

    // Test getAll with a filter that has no matches
    QHash<QString, QVariant> filter4;
    filter4.insert("author", "Author C");
    auto filteredEntities4 = m_entityTable->getAll(filter4).value();

    // Verify there are no entities returned
    QCOMPARE(filteredEntities4.size(), 0);
}

// ----------------------------------------------------------
void OrderedDatabaseTableTest::testGetAllWithFilter_disordoned()
{
    // Set up the table with some initial data in a disordered way
    QVector<int> order = {4, 1, 6, 2, 5, 3};
    for (int i : order)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }

    auto entities1 = m_entityTable->getAll().value();
    qDebug() << "unordered list at entities1";
    for (const Domain::DummyEntity &entity : entities1)
    {
        qDebug() << entity.name() << entity.author();
    }

    qDebug() << ""; // new line
    // Test getAll with a filter for "Author A"
    QHash<QString, QVariant> filter1;
    filter1.insert("author", "Author A");
    auto filteredEntitiesResult1 = m_entityTable->getAll(filter1);
    if (filteredEntitiesResult1.isError())
    {
        qDebug() << filteredEntitiesResult1.error().code() << filteredEntitiesResult1.error().message();
    }

    auto filteredEntities1 = filteredEntitiesResult1.value();

    qDebug() << "ordered list at filteredEntities1";
    for (const Domain::DummyEntity &entity : filteredEntities1)
    {
        qDebug() << entity.name() << entity.author();
    }
    // Verify the filtered entities are correct
    QCOMPARE(filteredEntities1.size(), 3);
    for (const Domain::DummyEntity &entity : filteredEntities1)
    {
        QCOMPARE(entity.author(), QString("Author A"));
    }

    // Test getAll with a filter for "Author B"
    QHash<QString, QVariant> filter2;
    filter2.insert("author", "Author B");
    auto filteredEntities2 = m_entityTable->getAll(filter2).value();

    // Verify the filtered entities are correct
    QCOMPARE(filteredEntities2.size(), 3);
    for (const Domain::DummyEntity &entity : filteredEntities2)
    {
        QCOMPARE(entity.author(), QString("Author B"));
    }

    // Test getAll with a filter for a specific name
    QHash<QString, QVariant> filter3;
    filter3.insert("name", "Sample DummyEntity 3");
    auto filteredEntities3 = m_entityTable->getAll(filter3).value();

    // Verify the filtered entity is correct
    QCOMPARE(filteredEntities3.size(), 1);
    QCOMPARE(filteredEntities3.at(0).name(), QString("Sample DummyEntity 3"));

    // Test getAll with a filter that has no matches
    QHash<QString, QVariant> filter4;
    filter4.insert("author", "Author C");
    auto filteredEntities4 = m_entityTable->getAll(filter4).value();

    // Verify there are no entities returned
    QCOMPARE(filteredEntities4.size(), 0);
}
// ----------------------------------------------------------

void OrderedDatabaseTableTest::testGetFuturePreviousAndNextIds()
{
    // test if empty:

    auto futurePreviousAndNextRowIds = m_entityTable->getFuturePreviousAndNextOrderedIds(0);
    if (futurePreviousAndNextRowIds.isError())
    {
        qDebug() << futurePreviousAndNextRowIds.error().code() << futurePreviousAndNextRowIds.error().message();
    }
    int previous = futurePreviousAndNextRowIds.value().first;
    int next = futurePreviousAndNextRowIds.value().second;
    QCOMPARE(previous, -1);
    QCOMPARE(next, -1);

    // add one
    Domain::DummyEntity entity1;
    entity1.setUuid(QUuid::createUuid());
    entity1.setName("Sample DummyEntity 1");
    entity1.setCreationDate(QDateTime::currentDateTime());
    entity1.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult1 = m_entityTable->add(std::move(entity1));
    if (insertResult1.isError())
    {
        qDebug() << insertResult1.error().code() << insertResult1.error().message();
    }
    QVERIFY(insertResult1.isSuccess());

    // test position 0 for size of 1

    futurePreviousAndNextRowIds = m_entityTable->getFuturePreviousAndNextOrderedIds(0);
    if (futurePreviousAndNextRowIds.isError())
    {
        qDebug() << futurePreviousAndNextRowIds.error().code() << futurePreviousAndNextRowIds.error().message();
    }
    previous = futurePreviousAndNextRowIds.value().first;
    next = futurePreviousAndNextRowIds.value().second;
    QCOMPARE(previous, -1);
    QCOMPARE_NE(next, -1);

    // test position 1 for size of 1

    futurePreviousAndNextRowIds = m_entityTable->getFuturePreviousAndNextOrderedIds(1);
    if (futurePreviousAndNextRowIds.isError())
    {
        qDebug() << futurePreviousAndNextRowIds.error().code() << futurePreviousAndNextRowIds.error().message();
    }
    previous = futurePreviousAndNextRowIds.value().first;
    next = futurePreviousAndNextRowIds.value().second;
    QCOMPARE_NE(previous, -1);
    QCOMPARE(next, -1);

    // test position -1 for size of 1

    futurePreviousAndNextRowIds = m_entityTable->getFuturePreviousAndNextOrderedIds(-1);
    if (futurePreviousAndNextRowIds.isError())
    {
        qDebug() << futurePreviousAndNextRowIds.error().code() << futurePreviousAndNextRowIds.error().message();
    }
    previous = futurePreviousAndNextRowIds.value().first;
    next = futurePreviousAndNextRowIds.value().second;
    QCOMPARE_NE(previous, -1);
    QCOMPARE(next, -1);

    // add one, making it size of 2
    Domain::DummyEntity entity2;
    entity2.setUuid(QUuid::createUuid());
    entity2.setName("Sample DummyEntity 2");
    entity2.setCreationDate(QDateTime::currentDateTime());
    entity2.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult2 = m_entityTable->add(std::move(entity2));
    if (insertResult2.isError())
    {
        qDebug() << insertResult2.error().code() << insertResult2.error().message();
    }
    QVERIFY(insertResult2.isSuccess());

    // test position 1 for size of 2

    futurePreviousAndNextRowIds = m_entityTable->getFuturePreviousAndNextOrderedIds(1);
    if (futurePreviousAndNextRowIds.isError())
    {
        qDebug() << futurePreviousAndNextRowIds.error().code() << futurePreviousAndNextRowIds.error().message();
    }
    previous = futurePreviousAndNextRowIds.value().first;
    next = futurePreviousAndNextRowIds.value().second;
    QCOMPARE_NE(previous, -1);
    QCOMPARE_NE(next, -1);
    QCOMPARE_NE(previous, next);
}
// ----------------------------------------------------------
void OrderedDatabaseTableTest::testGetPreviousAndNextIds()
{
    // Test empty table
    auto previousAndNextRowIds = m_entityTable->getPreviousAndNextOrderedIds(1);
    QVERIFY(previousAndNextRowIds.isError());

    // Add one entity
    Domain::DummyEntity entity1;
    entity1.setUuid(QUuid::createUuid());
    entity1.setName("Sample DummyEntity 1");
    entity1.setCreationDate(QDateTime::currentDateTime());
    entity1.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult1 = m_entityTable->add(std::move(entity1));
    QVERIFY(insertResult1.isSuccess());
    int entityId1 = insertResult1.value().id();

    // Test for added entity
    previousAndNextRowIds = m_entityTable->getPreviousAndNextOrderedIds(entityId1);
    QVERIFY(!previousAndNextRowIds.isError());
    int previous = previousAndNextRowIds.value().first;
    int next = previousAndNextRowIds.value().second;
    QCOMPARE(previous, -1);
    QCOMPARE(next, -1);

    // Add a second entity
    Domain::DummyEntity entity2;
    entity2.setUuid(QUuid::createUuid());
    entity2.setName("Sample DummyEntity 2");
    entity2.setCreationDate(QDateTime::currentDateTime());
    entity2.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult2 = m_entityTable->add(std::move(entity2));
    QVERIFY(insertResult2.isSuccess());
    int entityId2 = insertResult2.value().id();

    // Test for first entity
    previousAndNextRowIds = m_entityTable->getPreviousAndNextOrderedIds(entityId1);
    QVERIFY(!previousAndNextRowIds.isError());
    previous = previousAndNextRowIds.value().first;
    next = previousAndNextRowIds.value().second;
    QCOMPARE(previous, -1);
    QCOMPARE_NE(next, -1);

    // Test for second entity
    previousAndNextRowIds = m_entityTable->getPreviousAndNextOrderedIds(entityId2);
    QVERIFY(!previousAndNextRowIds.isError());
    previous = previousAndNextRowIds.value().first;
    next = previousAndNextRowIds.value().second;
    QCOMPARE_NE(previous, -1);
    QCOMPARE(next, -1);
}

// ----------------------------------------------------------
void OrderedDatabaseTableTest::testInsert()
{

    // Test insert when the table is empty and position is 0
    Domain::DummyEntity entity1;
    entity1.setUuid(QUuid::createUuid());
    entity1.setName("Sample DummyEntity 1");
    entity1.setCreationDate(QDateTime::currentDateTime());
    entity1.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult1 = m_entityTable->insert(std::move(entity1), 0);
    if (insertResult1.isError())
    {
        qDebug() << insertResult1.error().code() << insertResult1.error().message();
    }
    QVERIFY(insertResult1.isSuccess());

    // Verify the entity is inserted at position 0
    auto entitiesResult1 = m_entityTable->getAll();

    if (entitiesResult1.isError())
    {
        qDebug() << entitiesResult1.error().code() << entitiesResult1.error().message();
    }
    QCOMPARE(entitiesResult1.value().size(), 1);
    QCOMPARE(entitiesResult1.value().first().name(), QString("Sample DummyEntity 1"));

    // Test insert when the table is not empty and position is -1 (end of the list)
    Domain::DummyEntity entity2;
    entity2.setUuid(QUuid::createUuid());
    entity2.setName("Sample DummyEntity 2");
    entity2.setCreationDate(QDateTime::currentDateTime());
    entity2.setUpdateDate(QDateTime::currentDateTime());
    debugOrderingTable();
    printTableContents();
    auto insertResult2 = m_entityTable->insert(std::move(entity2), -1);
    debugOrderingTable();
    printTableContents();
    if (insertResult2.isError())
    {
        qDebug() << insertResult2.error().code() << insertResult2.error().message();
    }
    QVERIFY(insertResult2.isSuccess());

    // Verify the entity is inserted at the end of the list
    auto entities2 = m_entityTable->getAll().value();
    QCOMPARE(entities2.size(), 2);
    QCOMPARE(entities2.last().name(), QString("Sample DummyEntity 2"));

    // Test insert when the table is not empty and position is a valid position (e.g., 1)
    Domain::DummyEntity entity3;
    entity3.setUuid(QUuid::createUuid());
    entity3.setName("Sample DummyEntity 3");
    entity3.setCreationDate(QDateTime::currentDateTime());
    entity3.setUpdateDate(QDateTime::currentDateTime());

    debugOrderingTable();
    printTableContents();
    auto insertResult3 = m_entityTable->insert(std::move(entity3), 1);
    QVERIFY(insertResult3.isSuccess());
    debugOrderingTable();
    printTableContents();
    // Verify the entity is inserted at position 1
    auto entities3 = m_entityTable->getAll().value();
    QCOMPARE(entities3.size(), 3);
    QCOMPARE(entities3.at(1).name(), QString("Sample DummyEntity 3"));

    // Test insert when the table is not empty and position is a position superior of row number (e.g., 20)
    Domain::DummyEntity entity4;
    entity4.setUuid(QUuid::createUuid());
    entity4.setName("Sample DummyEntity 4");
    entity4.setCreationDate(QDateTime::currentDateTime());
    entity4.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult4 = m_entityTable->insert(std::move(entity4), 20);
    if (insertResult4.isError())
    {
        qDebug() << insertResult4.error().code() << insertResult4.error().message();
    }
    QVERIFY(insertResult4.isSuccess());

    // Verify the entity is inserted at position 3
    auto entities4 = m_entityTable->getAll().value();
    QCOMPARE(entities4.size(), 4);
    QCOMPARE(entities4.at(3).name(), QString("Sample DummyEntity 4"));
}

void OrderedDatabaseTableTest::testFilteredInsert()
{
    // Set up the table with some initial data
    for (int i = 1; i <= 3; ++i)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        QVERIFY(m_entityTable->insert(std::move(entity), -1).isSuccess());
    }
    auto initialEntities = m_entityTable->getAll().value();

    // Test insert with a filter when position is 0
    Domain::DummyEntity entity1;
    entity1.setUuid(QUuid::createUuid());
    entity1.setName("Filtered DummyEntity 1");
    entity1.setAuthor("Author A");
    entity1.setCreationDate(QDateTime::currentDateTime());
    entity1.setUpdateDate(QDateTime::currentDateTime());
    QHash<QString, QVariant> filter1;
    filter1.insert("author", "Author A");
    debugOrderingTable();
    printTableContents();
    auto insertResult1 = m_entityTable->insert(std::move(entity1), 0, filter1);
    debugOrderingTable();
    printTableContents();
    QVERIFY(insertResult1.isSuccess());

    // Verify the entity is inserted at position 0 among filtered entities
    auto entities1 = m_entityTable->getAll().value();
    QCOMPARE(entities1.at(1).name(), QString("Filtered DummyEntity 1"));

    qDebug() << ""; // new line

    // Test insert with a filter when position is -1 (end of the filtered list)
    Domain::DummyEntity entity2;
    entity2.setUuid(QUuid::createUuid());
    entity2.setName("Filtered DummyEntity 2");
    entity2.setAuthor("Author A");
    entity2.setCreationDate(QDateTime::currentDateTime());
    entity2.setUpdateDate(QDateTime::currentDateTime());
    debugOrderingTable();
    printTableContents();
    auto insertResult2 = m_entityTable->insert(std::move(entity2), -1, filter1);
    debugOrderingTable();
    printTableContents();
    QVERIFY(insertResult2.isSuccess());

    // Verify the entity is inserted at the end of the filtered list
    auto entities2 = m_entityTable->getAll(filter1).value();

    QCOMPARE(entities2.size(), 3);
    QCOMPARE(entities2.last().name(), QString("Filtered DummyEntity 2"));

    // Test insert with a filter when position is a valid position (e.g., 1)
    Domain::DummyEntity entity3;
    entity3.setUuid(QUuid::createUuid());
    entity3.setName("Filtered DummyEntity 3");
    entity3.setAuthor("Author A");
    entity3.setCreationDate(QDateTime::currentDateTime());
    entity3.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult3 = m_entityTable->insert(std::move(entity3), 1, filter1);
    QVERIFY(insertResult3.isSuccess());

    // Verify the entity is inserted at position 1 among filtered entities
    auto entities3 = m_entityTable->getAll(filter1).value();

    qDebug() << "ordered list at entities3";
    for (const Domain::DummyEntity &entity : entities3)
    {
        qDebug() << entity.name() << entity.author();
    }
    qDebug() << "";
    QCOMPARE(entities3.size(), 4);
    QCOMPARE(entities3.at(1).name(), QString("Filtered DummyEntity 3"));

    // Test insert with a filter when position is a position superior to the row number (e.g., 20)
    Domain::DummyEntity entity4;
    entity4.setUuid(QUuid::createUuid());
    entity4.setName("Filtered DummyEntity 4");
    entity4.setAuthor("Author A");
    entity4.setCreationDate(QDateTime::currentDateTime());
    entity4.setUpdateDate(QDateTime::currentDateTime());
    auto insertResult4 = m_entityTable->insert(std::move(entity4), 20, filter1);
    QVERIFY(insertResult4.isSuccess());

    // Verify the entity is inserted at the end of the filtered list
    auto entities4 = m_entityTable->getAll(filter1).value();

    qDebug() << "ordered list at entities4";
    for (const Domain::DummyEntity &entity : entities4)
    {
        qDebug() << entity.name() << entity.author();
    }
    qDebug() << "";

    QCOMPARE(entities4.size(), 5);
    QCOMPARE(entities4.at(4).name(), QString("Filtered DummyEntity 4"));
}

void OrderedDatabaseTableTest::testRemove()
{
    // Set up the table with some initial data
    QVector<Domain::DummyEntity> initialEntities;
    for (int i = 1; i <= 3; ++i)
    {
        Domain::DummyEntity entity;
        entity.setUuid(QUuid::createUuid());
        entity.setName(QString("Sample DummyEntity %1").arg(i));
        entity.setAuthor(i % 2 == 0 ? "Author A" : "Author B");
        entity.setCreationDate(QDateTime::currentDateTime());
        entity.setUpdateDate(QDateTime::currentDateTime());
        auto result = m_entityTable->insert(std::move(entity), -1);
        QVERIFY(result.isSuccess());
        initialEntities.append(result.value());
    }

    // Verify the initial count
    auto entitiesResult = m_entityTable->getAll();
    QVERIFY(entitiesResult.isSuccess());
    QCOMPARE(entitiesResult.value().size(), 3);

    // Remove the second entity
    auto removeResult = m_entityTable->remove(Domain::DummyEntity(initialEntities.at(1)));
    QVERIFY(removeResult.isSuccess());

    // Verify the count after removal
    entitiesResult = m_entityTable->getAll();
    QVERIFY(entitiesResult.isSuccess());
    QCOMPARE(entitiesResult.value().size(), 2);

    // Verify the order of the remaining entities
    auto uuidResult = m_entityTable->getAllOrderedUuid();
    QVERIFY(uuidResult.isSuccess());
    QList<QUuid> uuidList = uuidResult.value();
    QCOMPARE(uuidList.size(), 2);
    QCOMPARE(uuidList.at(0), initialEntities.at(0).uuid());
    QCOMPARE(uuidList.at(1), initialEntities.at(2).uuid());

    // Remove the first entity
    removeResult = m_entityTable->remove(Domain::DummyEntity(initialEntities.at(0)));
    QVERIFY(removeResult.isSuccess());

    // Verify the count after removal
    entitiesResult = m_entityTable->getAll();
    QVERIFY(entitiesResult.isSuccess());
    QCOMPARE(entitiesResult.value().size(), 1);

    // Verify the remaining entity
    uuidResult = m_entityTable->getAllOrderedUuid();
    QVERIFY(uuidResult.isSuccess());
    uuidList = uuidResult.value();
    QCOMPARE(uuidList.size(), 1);
    QCOMPARE(uuidList.at(0), initialEntities.at(2).uuid());
}

void OrderedDatabaseTableTest::debugOrderingTable()
{
    auto tableResult = m_entityTable->getOrderingTableData();

    if (tableResult.hasError())
    {
        qCritical() << "Error getting ordering table data:" << tableResult.error().code();
        return;
    }

    const QList<QVariantMap> &orderedTableData = tableResult.value();

    qDebug() << "Ordering Table Data:";

    for (const QVariantMap &row : orderedTableData)
    {
        QStringList rowData;

        for (auto it = row.constBegin(); it != row.constEnd(); ++it)
        {
            rowData.append(QString("%1: %2").arg(it.key(), it.value().toString()));
        }

        qDebug() << rowData.join(", ");
    }
}

void OrderedDatabaseTableTest::printTableContents()
{
    auto entities = m_entityTable->getAll().value();
    for (int i = 0; i < entities.size(); ++i)
    {
        qDebug() << "Position:" << i << ", Name:" << entities.at(i).name();
    }
}

QTEST_APPLESS_MAIN(OrderedDatabaseTableTest)

#include "tst_ordered_database_table.moc"

#include "database/database_table.h"
#include "database/tools.h"
#include "dummy_database_context.h"
#include "dummy_entity_with_foreign.h"
#include "dummy_other_entity.h"
#include "foreign_entity_wrapper.h"
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

  private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // lower level tests
    void testGetRelatedEntityIds();
    void testAddEntityRelationship();
    void testAddEntityRelationship_TwoDummyOtherEntity();
    void testAddEntityRelationship_TwoDummyOtherEntityAtPosition();
    void testGetRelatedEntityIds_TwoInList();
    void testMoveRelatedEntityIds();
    void testMoveRelatedEntityToFirstPosition();
    void testMoveRelatedEntityToSamePosition();
    void testMoveRelatedEntityNotFound();

    // higher level tests:

    void testListAddRelationship();
    void testListRemoveRelationship();
    void testListRemoveRelationship_OneOfTwo();
    void testListUpdateRelationship();

  private:
    Domain::DummyEntityWithForeign addToTable(
        const QString &name, const QList<Domain::DummyOtherEntity> &lists = QList<Domain::DummyOtherEntity>(),
        const Domain::DummyOtherEntity &unique = Domain::DummyOtherEntity());
    Domain::DummyOtherEntity addToOtherTable(const QString &name);
    void debugListsRelationshipTable();

  private:
    DummyDatabaseContext<Domain::DummyEntityWithForeign, Domain::DummyOtherEntity> *m_context;

    ForeignEntityWrapper<Domain::DummyEntityWithForeign> *m_foreignEntityTable;
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
    m_context = new DummyDatabaseContext<Domain::DummyEntityWithForeign, Domain::DummyOtherEntity>();
    m_context->setEntityClassNames(QStringList() << "DummyEntityWithForeign"
                                                 << "DummyOtherEntity");
    m_context->init();
    m_foreignEntityTable = new ForeignEntityWrapper<Domain::DummyEntityWithForeign>(m_context);
    m_entityTable = new Database::DatabaseTable<Domain::DummyEntityWithForeign>(m_context);
    m_otherEntityTable = new Database::DatabaseTable<Domain::DummyOtherEntity>(m_context);
}

void ForeignEntityTest::cleanupTestCase()
{
}

void ForeignEntityTest::init()
{
    m_context->getConnection().transaction();
}

void ForeignEntityTest::cleanup()
{
    m_context->getConnection().rollback();
}
//-----------------------------------------------------------------------------

void ForeignEntityTest::testGetRelatedEntityIds()
{
    // Add an entity to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity = addToOtherTable("Sample DummyOtherEntity");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (NULL, NULL, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    // Insert a relationship into dummy_entity_with_foreign_unique_relationship
    query.prepare("INSERT INTO dummy_entity_with_foreign_unique_relationship (dummy_entity_with_foreign_id, "
                  "dummy_other_entity_id) "
                  "VALUES (:dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity.id());
    execStatus = query.exec();
    if (!execStatus)
    {
        qDebug() << query.lastError();
    }
    QVERIFY(execStatus);

    // Use the ForeignEntityWrapper's testGetRelatedEntityIds function to get the list of related entity ids from
    // 'lists'
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    if (resultList.hasError())
    {
        qDebug() << resultList.errorString();
    }
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    QVERIFY(relatedIdsList.contains(otherEntity.id()));

    // Use the ForeignEntityWrapper's testGetRelatedEntityIds function to get the list of related entity ids from
    // 'unique'
    Result<QList<int>> resultUnique = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "unique");
    QVERIFY(resultUnique.isOk());
    QList<int> relatedIdsUnique = resultUnique.value();
    QVERIFY(relatedIdsUnique.contains(otherEntity.id()));
}
//-----------------------------------------------------------------------------

void ForeignEntityTest::testGetRelatedEntityIds_TwoInList()
{
    // Add an entity to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity1 = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (NULL, :next, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":next", 0);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity2.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (:previous, NULL, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":previous", 1);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity1.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    // Use the ForeignEntityWrapper's testGetRelatedEntityIds function to get the list of related entity ids from
    // 'lists'
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    qDebug() << relatedIdsList;
    QCOMPARE(relatedIdsList, QList<int>() << 2 << 1);
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testMoveRelatedEntityIds()
{

    // Add three entities to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity1 = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");
    Domain::DummyOtherEntity otherEntity3 = addToOtherTable("Sample DummyOtherEntity 3");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (:previous, :next, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":previous", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":next", 2);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity1.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 1);
    query.bindValue(":next", 3);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity2.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 2);
    query.bindValue(":next", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity3.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    qDebug() << "before";
    void debugListsRelationshipTable();

    // Call the moveEntityRelationship function to switch the position of otherEntity2 and otherEntity3
    Result<void> result = m_foreignEntityTable->moveEntityRelationship(entity.id(), otherEntity2.id(), "lists", 3);
    QVERIFY(result.isOk());

    qDebug() << "after";
    void debugListsRelationshipTable();

    // Check the new order of the related entity ids
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    qDebug() << relatedIdsList;
    QCOMPARE(relatedIdsList, QList<int>() << otherEntity1.id() << otherEntity3.id() << otherEntity2.id());
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testMoveRelatedEntityToFirstPosition()
{

    // Add three entities to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity1 = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");
    Domain::DummyOtherEntity otherEntity3 = addToOtherTable("Sample DummyOtherEntity 3");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (:previous, :next, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":previous", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":next", 2);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity1.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 1);
    query.bindValue(":next", 3);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity2.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 2);
    query.bindValue(":next", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity3.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    // Move the third entity to the first position
    Result<void> result = m_foreignEntityTable->moveEntityRelationship(entity.id(), otherEntity3.id(), "lists", 0);
    QVERIFY(result.isOk());

    // Check the new order of the related entity ids
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    QCOMPARE(relatedIdsList, QList<int>() << otherEntity3.id() << otherEntity1.id() << otherEntity2.id());
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testMoveRelatedEntityToSamePosition()
{

    // Add three entities to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity1 = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");
    Domain::DummyOtherEntity otherEntity3 = addToOtherTable("Sample DummyOtherEntity 3");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (:previous, :next, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":previous", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":next", 2);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity1.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 1);
    query.bindValue(":next", 3);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity2.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 2);
    query.bindValue(":next", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity3.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    // Move the second entity to the second position (i.e., don't move it at all)
    Result<void> result = m_foreignEntityTable->moveEntityRelationship(entity.id(), otherEntity2.id(), "lists", 1);
    QVERIFY(result.isOk());

    // Check the order of the related entity ids (it should be unchanged)
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    QCOMPARE(relatedIdsList, QList<int>() << otherEntity1.id() << otherEntity2.id() << otherEntity3.id());
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testMoveRelatedEntityNotFound()
{
    // Add three entities to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity1 = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");
    Domain::DummyOtherEntity otherEntity3 = addToOtherTable("Sample DummyOtherEntity 3");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    bool execStatus;

    // Insert a relationship into dummy_entity_with_foreign_lists_relationship
    query.prepare(
        "INSERT INTO dummy_entity_with_foreign_lists_relationship (previous, next, dummy_entity_with_foreign_id, "
        "dummy_other_entity_id) VALUES (:previous, :next, :dummy_entity_id, :dummy_other_entity_id)");
    query.bindValue(":previous", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":next", 2);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity1.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 1);
    query.bindValue(":next", 3);
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity2.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    query.bindValue(":previous", 2);
    query.bindValue(":next", QVariant(QMetaType::fromType<QString>()));
    query.bindValue(":dummy_entity_id", entity.id());
    query.bindValue(":dummy_other_entity_id", otherEntity3.id());
    execStatus = query.exec();
    QVERIFY(execStatus);

    // Try to move an entity that isn't related to the main entity
    Domain::DummyOtherEntity otherEntity4 = addToOtherTable("Sample DummyOtherEntity 4");
    Result<void> result = m_foreignEntityTable->moveEntityRelationship(entity.id(), otherEntity4.id(), "lists", 2);
    QVERIFY(result.hasError()); // It should return an error

    // Check the order of the related entity ids (it should be unchanged)
    Result<QList<int>> resultList = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    QVERIFY(resultList.isOk());
    QList<int> relatedIdsList = resultList.value();
    QCOMPARE(relatedIdsList, QList<int>() << otherEntity1.id() << otherEntity2.id() << otherEntity3.id());
}
//-----------------------------------------------------------------------------

void ForeignEntityTest::testAddEntityRelationship()
{
    // Add an entity to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity = addToOtherTable("Sample DummyOtherEntity");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    // Use the ForeignEntityWrapper's testAddEntityRelationship function to add a relationship
    Result<void> result = m_foreignEntityTable->testAddEntityRelationship(entity.id(), otherEntity.id(), "lists");
    if (result.hasError())
    {
        qDebug() << result.errorString();
    }
    // Check if the relationship was added without errors
    QVERIFY(result.isOk());

    // Check if the relationship was created correctly.
    // For this, get the list of related foreign ids for the entity and check if it contains the otherEntity's id.
    Result<QList<int>> relatedIdsResult = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    if (relatedIdsResult.hasError())
    {
        qDebug() << relatedIdsResult.errorString();
    }
    QVERIFY(relatedIdsResult.isOk());
    QList<int> relatedIds = relatedIdsResult.value();
    qDebug() << relatedIdsResult.value();
    QVERIFY(relatedIds.contains(otherEntity.id()));
}
//-----------------------------------------------------------------------------

void ForeignEntityTest::testAddEntityRelationship_TwoDummyOtherEntity()
{
    // Add an entity to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    // Use the ForeignEntityWrapper's testAddEntityRelationship function to add a relationship
    Result<void> result = m_foreignEntityTable->testAddEntityRelationship(entity.id(), otherEntity.id(), "lists");
    if (result.hasError())
    {
        qDebug() << result.errorString();
    }
    // Check if the relationship was added without errors
    QVERIFY(result.isOk());

    Result<void> result2 = m_foreignEntityTable->testAddEntityRelationship(entity.id(), otherEntity2.id(), "lists");
    if (result2.hasError())
    {
        qDebug() << result2.errorString();
    }
    // Check if the relationship was added without errors
    QVERIFY(result2.isOk());

    // Check if the relationship was created correctly.
    // For this, get the list of related foreign ids for the entity and check if it contains the otherEntity's id.
    Result<QList<int>> relatedIdsResult = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    if (relatedIdsResult.hasError())
    {
        qDebug() << relatedIdsResult.errorString();
    }
    QVERIFY(relatedIdsResult.isOk());
    QList<int> relatedIds = relatedIdsResult.value();
    qDebug() << relatedIdsResult.value();
    QCOMPARE(relatedIds, QList<int>() << 1 << 2);
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testAddEntityRelationship_TwoDummyOtherEntityAtPosition()
{
    // Add an entity to the DummyOtherEntity table
    Domain::DummyOtherEntity otherEntity = addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity otherEntity2 = addToOtherTable("Sample DummyOtherEntity 2");

    // Add an entity to the DummyEntityWithForeign table
    Domain::DummyEntityWithForeign entity = addToTable("Sample DummyEntityWithForeign");

    // Use the ForeignEntityWrapper's testAddEntityRelationship function to add a relationship
    Result<void> result = m_foreignEntityTable->testAddEntityRelationship(entity.id(), otherEntity.id(), "lists");
    if (result.hasError())
    {
        qDebug() << result.errorString();
    }
    // Check if the relationship was added without errors
    QVERIFY(result.isOk());

    Result<void> result2 = m_foreignEntityTable->testAddEntityRelationship(entity.id(), otherEntity2.id(), "lists", 0);
    if (result2.hasError())
    {
        qDebug() << result2.errorString();
    }
    // Check if the relationship was added without errors
    QVERIFY(result2.isOk());

    // Check if the relationship was created correctly.
    // For this, get the list of related foreign ids for the entity and check if it contains the otherEntity's id.
    Result<QList<int>> relatedIdsResult = m_foreignEntityTable->testGetRelatedEntityIds(entity.id(), "lists");
    if (relatedIdsResult.hasError())
    {
        qDebug() << relatedIdsResult.errorString();
    }
    QVERIFY(relatedIdsResult.isOk());
    QList<int> relatedIds = relatedIdsResult.value();
    qDebug() << relatedIdsResult.value();
    QCOMPARE(relatedIds, QList<int>() << 2 << 1);
}
//-----------------------------------------------------------------------------

void ForeignEntityTest::testListAddRelationship()
{
    // setup other table

    Domain::DummyOtherEntity createdOtherEntity = this->addToOtherTable("Sample DummyOtherEntity");

    // setup table

    Domain::DummyEntityWithForeign createdEntity =
        this->addToTable("Sample DummyEntityWithForeign", QList<Domain::DummyOtherEntity>() << createdOtherEntity);

    // setup the lazy loading :
    createdEntity.setListsLoader([this, &createdEntity]() {
        QList<int> otherIds = m_entityTable->getRelatedForeignIds(createdEntity, "lists").value();

        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }

        return otherEntities;
    });

    // test the presence of the relationship

    QCOMPARE(createdEntity.lists().size(), 1);

    QCOMPARE(createdEntity.lists().first().name(), "Sample DummyOtherEntity");
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testListRemoveRelationship()
{
    // setup other table

    Domain::DummyOtherEntity createdOtherEntity = this->addToOtherTable("Sample DummyOtherEntity");

    // setup table

    Domain::DummyEntityWithForeign createdEntity =
        this->addToTable("Sample DummyEntityWithForeign", QList<Domain::DummyOtherEntity>() << createdOtherEntity);

    // setup the lazy loading :
    createdEntity.setListsLoader([this, &createdEntity]() {
        QList<int> otherIds = m_entityTable->getRelatedForeignIds(createdEntity, "lists").value();

        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }

        return otherEntities;
    });

    // test the presence of the relationship

    QCOMPARE(createdEntity.lists().size(), 1);

    QCOMPARE(createdEntity.lists().first().name(), "Sample DummyOtherEntity");

    // remove the relationship

    createdEntity.setLists(QList<Domain::DummyOtherEntity>());

    auto updateResult = m_entityTable->update(std::move(createdEntity));
    if (updateResult.isError())
    {
        qDebug() << updateResult.error().code() << updateResult.error().message() << updateResult.error().data();
    }
    QVERIFY(updateResult.isSuccess());

    // prepare the update result

    Domain::DummyEntityWithForeign updateEntity = updateResult.value();
    int updateEntityId = updateEntity.id();

    // setup the lazy loading :
    updateEntity.setListsLoader([this, &updateEntity]() {
        auto result = m_entityTable->getRelatedForeignIds(updateEntity, "lists");
        if (result.isError())
        {
            qDebug() << result.error().code() << result.error().message() << result.error().data();
            qFatal("");
        }

        QList<int> otherIds = result.value();

        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }

        return otherEntities;
    });

    // test the absence of list

    QCOMPARE(updateEntity.lists().size(), 0);
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::testListRemoveRelationship_OneOfTwo()
{
    // setup other table

    Domain::DummyOtherEntity createdOtherEntity = this->addToOtherTable("Sample DummyOtherEntity");

    // setup table

    Domain::DummyEntityWithForeign createdEntity =
        this->addToTable("Sample DummyEntityWithForeign", QList<Domain::DummyOtherEntity>() << createdOtherEntity);

    // setup the lazy loading :
    createdEntity.setListsLoader([this, &createdEntity]() {
        QList<int> otherIds = m_entityTable->getRelatedForeignIds(createdEntity, "lists").value();

        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }

        return otherEntities;
    });

    // test the presence of the relationship

    QCOMPARE(createdEntity.lists().size(), 1);

    QCOMPARE(createdEntity.lists().first().name(), "Sample DummyOtherEntity");

    // remove the relationship

    createdEntity.setLists(QList<Domain::DummyOtherEntity>());

    auto updateResult = m_entityTable->update(std::move(createdEntity));
    if (updateResult.isError())
    {
        qDebug() << updateResult.error().code() << updateResult.error().message() << updateResult.error().data();
    }
    QVERIFY(updateResult.isSuccess());

    // prepare the update result

    Domain::DummyEntityWithForeign updateEntity = updateResult.value();
    int updateEntityId = updateEntity.id();

    // setup the lazy loading :
    updateEntity.setListsLoader([this, &updateEntity]() {
        auto result = m_entityTable->getRelatedForeignIds(updateEntity, "lists");
        if (result.isError())
        {
            qDebug() << result.error().code() << result.error().message() << result.error().data();
            qFatal("");
        }

        QList<int> otherIds = result.value();

        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }

        return otherEntities;
    });

    // test the absence of list

    QCOMPARE(updateEntity.lists().size(), 0);
}

//-----------------------------------------------------------------------------
void ForeignEntityTest::testListUpdateRelationship()
{
    // setup other table
    Domain::DummyOtherEntity createdOtherEntity1 = this->addToOtherTable("Sample DummyOtherEntity 1");
    Domain::DummyOtherEntity createdOtherEntity2 = this->addToOtherTable("Sample DummyOtherEntity 2");
    Domain::DummyOtherEntity createdOtherEntity3 = this->addToOtherTable("Sample DummyOtherEntity 3");

    // setup table
    Domain::DummyEntityWithForeign createdEntity =
        this->addToTable("Sample DummyEntityWithForeign", QList<Domain::DummyOtherEntity>()
                                                              << createdOtherEntity1 << createdOtherEntity2);

    // setup the lazy loading
    createdEntity.setListsLoader([this, &createdEntity]() {
        QList<int> otherIds = m_entityTable->getRelatedForeignIds(createdEntity, "lists").value();
        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }
        return otherEntities;
    });

    // test the presence of the relationships
    QCOMPARE(createdEntity.lists().size(), 2);

    // remove one entity, add a new one, and change the order
    createdEntity.setLists(QList<Domain::DummyOtherEntity>() << createdOtherEntity3 << createdOtherEntity1);

    auto updateResult = m_entityTable->update(std::move(createdEntity));
    QVERIFY(updateResult.isSuccess());

    // prepare the update result
    Domain::DummyEntityWithForeign updateEntity = updateResult.value();

    // setup the lazy loading
    updateEntity.setListsLoader([this, &updateEntity]() {
        QList<int> otherIds = m_entityTable->getRelatedForeignIds(updateEntity, "lists").value();
        QList<Domain::DummyOtherEntity> otherEntities;
        for (int otherId : otherIds)
        {
            otherEntities.append(m_otherEntityTable->get(otherId).value());
        }
        return otherEntities;
    });

    // test the updated list
    QCOMPARE(updateEntity.lists().size(), 2);
    QCOMPARE(updateEntity.lists().at(0).name(), "Sample DummyOtherEntity 3");
    QCOMPARE(updateEntity.lists().at(1).name(), "Sample DummyOtherEntity 1");
}

//-----------------------------------------------------------------------------

Domain::DummyEntityWithForeign ForeignEntityTest::addToTable(const QString &name,
                                                             const QList<Domain::DummyOtherEntity> &lists,
                                                             const Domain::DummyOtherEntity &unique)
{

    Domain::DummyEntityWithForeign entity;
    entity.setName(name);
    entity.setUuid(QUuid::createUuid());
    entity.setCreationDate(QDateTime::currentDateTime());

    entity.setLists(lists);
    entity.setUnique(unique);

    auto addResult = m_entityTable->add(std::move(entity));
    if (addResult.isError())
    {
        qDebug() << addResult.error().code() << addResult.error().message() << addResult.error().data();
    }

    return addResult.value();
}

//-----------------------------------------------------------------------------

Domain::DummyOtherEntity ForeignEntityTest::addToOtherTable(const QString &name)
{
    Domain::DummyOtherEntity otherEntity;
    otherEntity.setName(name);
    otherEntity.setUuid(QUuid::createUuid());
    otherEntity.setCreationDate(QDateTime::currentDateTime());

    auto addOtherResult = m_otherEntityTable->add(std::move(otherEntity));
    if (addOtherResult.isError())
    {
        qDebug() << addOtherResult.error().code() << addOtherResult.error().message() << addOtherResult.error().data();
    }

    return addOtherResult.value();
}

//-----------------------------------------------------------------------------

void ForeignEntityTest::debugListsRelationshipTable()
{

    QSqlDatabase db = m_context->getConnection();
    QSqlQuery query(db);
    // Add a SQL query to print all rows in the dummy_entity_with_foreign_lists_relationship table
    query.prepare("SELECT * FROM dummy_entity_with_foreign_lists_relationship");
    if (query.exec())
    {
        while (query.next())
        {
            qDebug() << "Id: " << query.value("id").toInt() << " Previous: " << query.value("previous").toInt()
                     << " Next: " << query.value("next").toInt()
                     << " Dummy Entity ID: " << query.value("dummy_entity_with_foreign_id").toInt()
                     << " Dummy Other Entity ID: " << query.value("dummy_other_entity_id").toInt();
        }
    }
    else
    {
        qWarning() << "Query execution error: " << query.lastError().text();
    }
}

QTEST_APPLESS_MAIN(ForeignEntityTest)

#include "tst_foreign_entity.moc"

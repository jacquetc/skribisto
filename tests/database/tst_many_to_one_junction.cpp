/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#include "database/junction_table_ops/junction_cache.h"
#include "database/junction_table_ops/many_to_one.h"
#include "service_locator.h"
#include <QObject>
#include <QSignalSpy>
#include <QSqlDatabase>
#include <QSqlError>
#include <QSqlQuery>
#include <QString>
#include <QTest>
#include <QThread>
#include <memory>
#include <optional>

using namespace Skribisto::Common::Database::JunctionTableOps;

class TestManyToOneJunction : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // getRightId tests
    void testGetRightId();
    void testGetRightIdMany();
    void testGetRightIdEmpty();
    void testGetRightIdNonExistent();

    // removeWithLeftId tests
    void testRemoveWithLeftId();
    void testRemoveWithLeftIdMany();
    void testRemoveWithLeftIdEmpty();
    void testRemoveWithLeftIdNonExistent();

    // removeWithRightIds tests
    void testRemoveWithRightIds();
    void testRemoveWithRightIdsMany();
    void testRemoveWithRightIdsEmpty();
    void testRemoveWithRightIdsNonExistent();

    // upsertRightId tests
    void testUpsertRightId();
    void testUpsertRightIdMany();
    void testUpsertRightIdOptional();
    void testUpsertRightIdEmpty();
    void testUpsertRightIdOverwrite();
    void testUpsertRightIdNullOptional();

    // getLeftIds tests (many-to-one specific)
    void testGetLeftIds();
    void testGetLeftIdsMany();
    void testGetLeftIdsEmpty();
    void testGetLeftIdsNonExistent();
    void testGetLeftIdsMultipleLefts();

    // getRightIdCount tests
    void testGetRightIdCount();
    void testGetRightIdCountZero();
    void testGetRightIdCountNonExistent();

    // getRightIdInRange tests
    void testGetRightIdInRange();
    void testGetRightIdInRangeEmpty();
    void testGetRightIdInRangeNonExistent();

    // many-to-one constraint tests
    void testManyToOneConstraintEnforcement();
    void testCacheInvalidation();

  private:
    QSqlDatabase m_db;
    QString m_junctionTableDefinition = QStringLiteral("CREATE TABLE IF NOT EXISTS test_junction ("
                                                       "left_id INTEGER NOT NULL UNIQUE, "
                                                       "right_id INTEGER NOT NULL"
                                                       ")");
    QString m_junctionTableName = QStringLiteral("test_junction");

    void setupDatabase();
    void insertTestData(const QList<QPair<int, int>> &data);
    void clearJunctionTable();
};

void TestManyToOneJunction::initTestCase()
{
    std::string duration("20000"); // 20 seconds
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
}

void TestManyToOneJunction::cleanupTestCase()
{
}

void TestManyToOneJunction::init()
{
    setupDatabase();
}

void TestManyToOneJunction::cleanup()
{
    if (m_db.isOpen())
    {
        QString connectionName = m_db.connectionName();
        clearJunctionTable();
        {
            QSqlDatabase db = m_db;
            m_db = QSqlDatabase(); // Reset member to avoid dangling reference

            // Clear junction cache
            Skribisto::Common::Database::JunctionTableOps::JunctionCache::instance().clear();

            // Close and remove the database connection
            db.close();
        }
        QSqlDatabase::removeDatabase(connectionName);
    }
}

void TestManyToOneJunction::setupDatabase()
{
    // Create unique in-memory database for each test
    static int counter = 0;
    QString connectionName = QStringLiteral("test_connection_%1").arg(++counter);

    m_db = QSqlDatabase::addDatabase(QStringLiteral("QSQLITE"), connectionName);
    m_db.setDatabaseName(QStringLiteral(":memory:"));

    QVERIFY(m_db.open());

    // Create junction table
    QSqlQuery query(m_db);
    QVERIFY(query.exec(m_junctionTableDefinition));
}

void TestManyToOneJunction::insertTestData(const QList<QPair<int, int>> &data)
{
    QSqlQuery query(m_db);
    query.prepare(QStringLiteral("INSERT INTO %1 (left_id, right_id) VALUES (?, ?)").arg(m_junctionTableName));

    for (const auto &pair : data)
    {
        query.addBindValue(pair.first);
        query.addBindValue(pair.second);
        QVERIFY2(query.exec(), query.lastError().text().toUtf8().data());
    }
}

void TestManyToOneJunction::clearJunctionTable()
{
    QSqlQuery query(m_db);
    QVERIFY(query.exec(QStringLiteral("DELETE FROM %1").arg(m_junctionTableName)));
}

void TestManyToOneJunction::testGetRightId()
{
    insertTestData({{1, 101}, {2, 102}});

    auto result = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(result.has_value());
    QCOMPARE(result.value(), 101);
}

void TestManyToOneJunction::testGetRightIdMany()
{
    insertTestData({{1, 101}, {2, 102}, {3, 103}});

    QList<int> leftIds = {1, 2, 4}; // 4 doesn't exist
    auto result = ManyToOne::getRightIdMany(m_db, leftIds, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QVERIFY(result[1].has_value());
    QCOMPARE(result[1].value(), 101);
    QVERIFY(result[2].has_value());
    QCOMPARE(result[2].value(), 102);
    QVERIFY(!result[4].has_value()); // Should be nullopt
}

void TestManyToOneJunction::testGetRightIdEmpty()
{
    auto result = ManyToOne::getRightIdMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testGetRightIdNonExistent()
{
    insertTestData({{1, 101}});

    auto result = ManyToOne::getRightId(m_db, 999, m_junctionTableName);
    QVERIFY(!result.has_value());
}

void TestManyToOneJunction::testRemoveWithLeftId()
{
    insertTestData({{1, 101}, {2, 102}});

    bool success = ManyToOne::removeWithLeftId(m_db, 1, m_junctionTableName);
    QVERIFY(success);

    // Verify removal
    auto result = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(!result.has_value());

    // Verify other record still exists
    result = ManyToOne::getRightId(m_db, 2, m_junctionTableName);
    QVERIFY(result.has_value());
    QCOMPARE(result.value(), 102);
}

void TestManyToOneJunction::testRemoveWithLeftIdMany()
{
    insertTestData({{1, 101}, {2, 102}, {3, 103}});

    QList<int> leftIds = {1, 3};
    auto results = ManyToOne::removeWithLeftIdMany(m_db, leftIds, m_junctionTableName);

    QCOMPARE(results.size(), 2);
    QVERIFY(results[1]);
    QVERIFY(results[3]);

    // Verify removals
    auto remaining = ManyToOne::getRightIdMany(m_db, {1, 2, 3}, m_junctionTableName);
    QVERIFY(!remaining[1].has_value());
    QVERIFY(remaining[2].has_value());
    QCOMPARE(remaining[2].value(), 102);
    QVERIFY(!remaining[3].has_value());
}

void TestManyToOneJunction::testRemoveWithLeftIdEmpty()
{
    auto result = ManyToOne::removeWithLeftIdMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testRemoveWithLeftIdNonExistent()
{
    bool success = ManyToOne::removeWithLeftId(m_db, 999, m_junctionTableName);
    QVERIFY(success); // Should succeed even if nothing to remove
}

void TestManyToOneJunction::testRemoveWithRightIds()
{
    insertTestData({{1, 101}, {2, 101}, {3, 102}});

    bool success = ManyToOne::removeWithRightIds(m_db, 101, m_junctionTableName);
    QVERIFY(success);

    // Verify removal - both left_ids 1 and 2 should be removed
    auto result1 = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(!result1.has_value());
    auto result2 = ManyToOne::getRightId(m_db, 2, m_junctionTableName);
    QVERIFY(!result2.has_value());

    // Verify left_id 3 still exists
    auto result3 = ManyToOne::getRightId(m_db, 3, m_junctionTableName);
    QVERIFY(result3.has_value());
    QCOMPARE(result3.value(), 102);
}

void TestManyToOneJunction::testRemoveWithRightIdsMany()
{
    insertTestData({{1, 101}, {2, 101}, {3, 102}, {4, 103}});

    QList<int> rightIds = {101, 103};
    auto results = ManyToOne::removeWithRightIdsMany(m_db, rightIds, m_junctionTableName);

    QCOMPARE(results.size(), 2);
    QVERIFY(results[101]);
    QVERIFY(results[103]);

    // Verify removals
    auto remaining = ManyToOne::getRightIdMany(m_db, {1, 2, 3, 4}, m_junctionTableName);
    QVERIFY(!remaining[1].has_value()); // removed with 101
    QVERIFY(!remaining[2].has_value()); // removed with 101
    QVERIFY(remaining[3].has_value());
    QCOMPARE(remaining[3].value(), 102);
    QVERIFY(!remaining[4].has_value()); // removed with 103
}

void TestManyToOneJunction::testRemoveWithRightIdsEmpty()
{
    auto result = ManyToOne::removeWithRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testRemoveWithRightIdsNonExistent()
{
    bool success = ManyToOne::removeWithRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(success); // Should succeed even if nothing to remove
}

void TestManyToOneJunction::testUpsertRightId()
{
    // Test insert
    auto result = ManyToOne::upsertRightId(m_db, 1, m_junctionTableName, 101);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);

    // Verify insert
    auto retrieved = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 101);
}

void TestManyToOneJunction::testUpsertRightIdMany()
{
    QHash<int, int> data;
    data[1] = 101;
    data[2] = 101; // Multiple lefts can point to same right
    data[3] = 102;

    auto results = ManyToOne::upsertRightIdMany(m_db, data, m_junctionTableName);

    QCOMPARE(results.size(), 3);
    QCOMPARE(results[1].size(), 1);
    QCOMPARE(results[1][0], 101);
    QCOMPARE(results[2].size(), 1);
    QCOMPARE(results[2][0], 101);
    QCOMPARE(results[3].size(), 1);
    QCOMPARE(results[3][0], 102);

    // Verify inserts
    auto retrieved = ManyToOne::getRightIdMany(m_db, {1, 2, 3}, m_junctionTableName);
    QVERIFY(retrieved[1].has_value());
    QCOMPARE(retrieved[1].value(), 101);
    QVERIFY(retrieved[2].has_value());
    QCOMPARE(retrieved[2].value(), 101);
    QVERIFY(retrieved[3].has_value());
    QCOMPARE(retrieved[3].value(), 102);
}

void TestManyToOneJunction::testUpsertRightIdOptional()
{
    // Test with valid optional
    std::optional<int> rightId = 101;
    auto result = ManyToOne::upsertRightId(m_db, 1, m_junctionTableName, rightId);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);

    // Verify
    auto retrieved = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 101);
}

void TestManyToOneJunction::testUpsertRightIdEmpty()
{
    QHash<int, int> data;
    auto results = ManyToOne::upsertRightIdMany(m_db, data, m_junctionTableName);
    QVERIFY(results.isEmpty());
}

void TestManyToOneJunction::testUpsertRightIdOverwrite()
{
    insertTestData({{1, 101}});

    // Update existing record
    auto result = ManyToOne::upsertRightId(m_db, 1, m_junctionTableName, 201);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 201);

    // Verify update
    auto retrieved = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 201);
}

void TestManyToOneJunction::testUpsertRightIdNullOptional()
{
    insertTestData({{1, 101}});

    // Test with nullopt - should remove
    QHash<int, std::optional<int>> data;
    data[1] = std::nullopt;

    auto results = ManyToOne::upsertRightIdMany(m_db, data, m_junctionTableName);
    QCOMPARE(results.size(), 1);
    QVERIFY(results[1].isEmpty());

    // Verify removal
    auto retrieved = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(!retrieved.has_value());
}

void TestManyToOneJunction::testGetLeftIds()
{
    insertTestData({{1, 101}, {2, 101}, {3, 102}});

    auto result = ManyToOne::getLeftIds(m_db, m_junctionTableName, 101);
    QCOMPARE(result.size(), 2);
    QVERIFY(result.contains(1));
    QVERIFY(result.contains(2));
}

void TestManyToOneJunction::testGetLeftIdsMany()
{
    insertTestData({{1, 101}, {2, 101}, {3, 102}, {4, 103}});

    QList<int> rightIds = {101, 102, 404}; // 404 doesn't exist
    auto result = ManyToOne::getLeftIdsMany(m_db, m_junctionTableName, rightIds);

    QCOMPARE(result.size(), 3);
    QVERIFY(result.contains(101));
    QCOMPARE(result[101].size(), 2);
    QVERIFY(result[101].contains(1));
    QVERIFY(result[101].contains(2));
    QVERIFY(result.contains(102));
    QCOMPARE(result[102].size(), 1);
    QVERIFY(result[102].contains(3));
    QVERIFY(result.contains(404));
    QVERIFY(result[404].isEmpty());
}

void TestManyToOneJunction::testGetLeftIdsEmpty()
{
    auto result = ManyToOne::getLeftIdsMany(m_db, m_junctionTableName, {});
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testGetLeftIdsNonExistent()
{
    insertTestData({{1, 101}});

    auto result = ManyToOne::getLeftIds(m_db, m_junctionTableName, 999);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testGetLeftIdsMultipleLefts()
{
    // Test that multiple left_ids can map to the same right_id
    insertTestData({{1, 101}, {2, 101}, {3, 101}});

    auto result = ManyToOne::getLeftIds(m_db, m_junctionTableName, 101);
    QCOMPARE(result.size(), 3);
    QVERIFY(result.contains(1));
    QVERIFY(result.contains(2));
    QVERIFY(result.contains(3));
}

void TestManyToOneJunction::testGetRightIdCount()
{
    insertTestData({{1, 101}});

    int count = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 1);
}

void TestManyToOneJunction::testGetRightIdCountZero()
{
    int count = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestManyToOneJunction::testGetRightIdCountNonExistent()
{
    insertTestData({{1, 101}});

    int count = ManyToOne::getRightIdCount(m_db, 999, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestManyToOneJunction::testGetRightIdInRange()
{
    insertTestData({{1, 101}});

    auto result = ManyToOne::getRightIdInRange(m_db, 1, m_junctionTableName);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);
}

void TestManyToOneJunction::testGetRightIdInRangeEmpty()
{
    auto result = ManyToOne::getRightIdInRange(m_db, 1, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testGetRightIdInRangeNonExistent()
{
    insertTestData({{1, 101}});

    auto result = ManyToOne::getRightIdInRange(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestManyToOneJunction::testManyToOneConstraintEnforcement()
{
    insertTestData({{1, 101}});

    // Test that updating to a different right_id works
    auto result = ManyToOne::upsertRightId(m_db, 1, m_junctionTableName, 201);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 201);

    // Verify only one right_id per left_id
    int count = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 1);

    auto retrieved = ManyToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 201);

    // Test that multiple left_ids can point to the same right_id
    ManyToOne::upsertRightId(m_db, 2, m_junctionTableName, 201);
    ManyToOne::upsertRightId(m_db, 3, m_junctionTableName, 201);

    auto leftIds = ManyToOne::getLeftIds(m_db, m_junctionTableName, 201);
    QCOMPARE(leftIds.size(), 3);
    QVERIFY(leftIds.contains(1));
    QVERIFY(leftIds.contains(2));
    QVERIFY(leftIds.contains(3));
}

void TestManyToOneJunction::testCacheInvalidation()
{
    insertTestData({{1, 101}});

    // Get count to populate cache
    int initialCount = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(initialCount, 1);

    // Remove the record - this should invalidate cache
    bool success = ManyToOne::removeWithLeftId(m_db, 1, m_junctionTableName);
    QVERIFY(success);

    // Count should now be 0 (cache should be invalidated)
    int newCount = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(newCount, 0);

    // Add back and test upsert cache invalidation
    ManyToOne::upsertRightId(m_db, 1, m_junctionTableName, 102);
    int finalCount = ManyToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(finalCount, 1);
}

QTEST_MAIN(TestManyToOneJunction)

#include "tst_many_to_one_junction.moc"

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
#include "database/junction_table_ops/one_to_one.h"
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

class TestOneToOneJunction : public QObject
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

    // upsertRightId tests
    void testUpsertRightId();
    void testUpsertRightIdMany();
    void testUpsertRightIdOptional();
    void testUpsertRightIdEmpty();
    void testUpsertRightIdOverwrite();
    void testUpsertRightIdNullOptional();

    // getLeftId tests
    void testGetLeftId();
    void testGetLeftIdMany();
    void testGetLeftIdEmpty();
    void testGetLeftIdNonExistent();

    // getRightIdCount tests
    void testGetRightIdCount();
    void testGetRightIdCountZero();
    void testGetRightIdCountNonExistent();

    // getRightIdInRange tests
    void testGetRightIdInRange();
    void testGetRightIdInRangeEmpty();
    void testGetRightIdInRangeNonExistent();

    // one-to-one constraint tests
    void testOneToOneConstraintEnforcement();
    void testCacheInvalidation();

  private:
    QSqlDatabase m_db;
    QString m_junctionTableName = QStringLiteral("test_junction");
    QString m_connectionName;

    void setupDatabase();
    void insertTestData(const QList<QPair<int, int>> &data);
    void clearJunctionTable();
};

void TestOneToOneJunction::initTestCase()
{
    m_connectionName = QStringLiteral("test_connection_%1").arg(reinterpret_cast<quintptr>(QThread::currentThread()));
    m_db = QSqlDatabase::addDatabase(QStringLiteral("QSQLITE"), m_connectionName);
    m_db.setDatabaseName(QStringLiteral(":memory:"));
    QVERIFY(m_db.open());
}

void TestOneToOneJunction::cleanupTestCase()
{
    m_db.close();
    QSqlDatabase::removeDatabase(m_connectionName);
}

void TestOneToOneJunction::init()
{
    setupDatabase();
}

void TestOneToOneJunction::cleanup()
{
    clearJunctionTable();
    // Drop the table to ensure clean state
    QSqlQuery query(m_db);
    query.exec(QStringLiteral("DROP TABLE IF EXISTS %1").arg(m_junctionTableName));
    // Clear cache to ensure test isolation
    Skribisto::Common::Database::JunctionTableOps::JunctionCache::instance().clear();
}

void TestOneToOneJunction::setupDatabase()
{
    QSqlQuery query(m_db);
    QString createTableSql = QStringLiteral("CREATE TABLE IF NOT EXISTS %1 ("
                                            "left_id INTEGER NOT NULL, "
                                            "right_id INTEGER NOT NULL, "
                                            "UNIQUE(left_id), "
                                            "UNIQUE(right_id)"
                                            ")")
                                 .arg(m_junctionTableName);

    QVERIFY2(query.exec(createTableSql), query.lastError().text().toUtf8().data());
}

void TestOneToOneJunction::insertTestData(const QList<QPair<int, int>> &data)
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

void TestOneToOneJunction::clearJunctionTable()
{
    QSqlQuery query(m_db);
    QVERIFY(query.exec(QStringLiteral("DELETE FROM %1").arg(m_junctionTableName)));
}

void TestOneToOneJunction::testGetRightId()
{
    insertTestData({{1, 101}, {2, 102}});

    auto result = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(result.has_value());
    QCOMPARE(result.value(), 101);
}

void TestOneToOneJunction::testGetRightIdMany()
{
    insertTestData({{1, 101}, {2, 102}, {3, 103}});

    QList<int> leftIds = {1, 2, 4}; // 4 doesn't exist
    auto result = OneToOne::getRightIdMany(m_db, leftIds, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QVERIFY(result[1].has_value());
    QCOMPARE(result[1].value(), 101);
    QVERIFY(result[2].has_value());
    QCOMPARE(result[2].value(), 102);
    QVERIFY(!result[4].has_value()); // Should be nullopt
}

void TestOneToOneJunction::testGetRightIdEmpty()
{
    auto result = OneToOne::getRightIdMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOneToOneJunction::testGetRightIdNonExistent()
{
    insertTestData({{1, 101}});

    auto result = OneToOne::getRightId(m_db, 999, m_junctionTableName);
    QVERIFY(!result.has_value());
}

void TestOneToOneJunction::testRemoveWithLeftId()
{
    insertTestData({{1, 101}, {2, 102}});

    bool success = OneToOne::removeWithLeftId(m_db, 1, m_junctionTableName);
    QVERIFY(success);

    // Verify removal
    auto result = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(!result.has_value());

    // Verify other record still exists
    result = OneToOne::getRightId(m_db, 2, m_junctionTableName);
    QVERIFY(result.has_value());
    QCOMPARE(result.value(), 102);
}

void TestOneToOneJunction::testRemoveWithLeftIdMany()
{
    insertTestData({{1, 101}, {2, 102}, {3, 103}});

    QList<int> leftIds = {1, 3};
    auto results = OneToOne::removeWithLeftIdMany(m_db, leftIds, m_junctionTableName);

    QCOMPARE(results.size(), 2);
    QVERIFY(results[1]);
    QVERIFY(results[3]);

    // Verify removals
    auto remaining = OneToOne::getRightIdMany(m_db, {1, 2, 3}, m_junctionTableName);
    QVERIFY(!remaining[1].has_value());
    QVERIFY(remaining[2].has_value());
    QCOMPARE(remaining[2].value(), 102);
    QVERIFY(!remaining[3].has_value());
}

void TestOneToOneJunction::testRemoveWithLeftIdEmpty()
{
    auto result = OneToOne::removeWithLeftIdMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOneToOneJunction::testRemoveWithLeftIdNonExistent()
{
    bool success = OneToOne::removeWithLeftId(m_db, 999, m_junctionTableName);
    QVERIFY(success); // Should succeed even if nothing to remove
}

void TestOneToOneJunction::testUpsertRightId()
{
    // Test insert
    auto result = OneToOne::upsertRightId(m_db, 1, m_junctionTableName, 101);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);

    // Verify insert
    auto retrieved = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 101);
}

void TestOneToOneJunction::testUpsertRightIdMany()
{
    QHash<int, int> data;
    data[1] = 101;
    data[2] = 102;

    auto results = OneToOne::upsertRightIdMany(m_db, data, m_junctionTableName);

    QCOMPARE(results.size(), 2);
    QCOMPARE(results[1].size(), 1);
    QCOMPARE(results[1][0], 101);
    QCOMPARE(results[2].size(), 1);
    QCOMPARE(results[2][0], 102);

    // Verify inserts
    auto retrieved = OneToOne::getRightIdMany(m_db, {1, 2}, m_junctionTableName);
    QVERIFY(retrieved[1].has_value());
    QCOMPARE(retrieved[1].value(), 101);
    QVERIFY(retrieved[2].has_value());
    QCOMPARE(retrieved[2].value(), 102);
}

void TestOneToOneJunction::testUpsertRightIdOptional()
{
    // Test with valid optional
    std::optional<int> rightId = 101;
    auto result = OneToOne::upsertRightId(m_db, 1, m_junctionTableName, rightId);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);

    // Verify
    auto retrieved = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 101);
}

void TestOneToOneJunction::testUpsertRightIdEmpty()
{
    QHash<int, int> data;
    auto results = OneToOne::upsertRightIdMany(m_db, data, m_junctionTableName);
    QVERIFY(results.isEmpty());
}

void TestOneToOneJunction::testUpsertRightIdOverwrite()
{
    insertTestData({{1, 101}});

    // Update existing record
    auto result = OneToOne::upsertRightId(m_db, 1, m_junctionTableName, 201);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 201);

    // Verify update
    auto retrieved = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 201);
}

void TestOneToOneJunction::testUpsertRightIdNullOptional()
{
    insertTestData({{1, 101}});

    // Test with nullopt - should remove
    QHash<int, std::optional<int>> data;
    data[1] = std::nullopt;

    auto results = OneToOne::upsertRightIdMany(m_db, data, m_junctionTableName);
    QCOMPARE(results.size(), 1);
    QVERIFY(results[1].isEmpty());

    // Verify removal
    auto retrieved = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(!retrieved.has_value());
}

void TestOneToOneJunction::testGetLeftId()
{
    insertTestData({{1, 101}, {2, 102}});

    int result = OneToOne::getLeftId(m_db, m_junctionTableName, 101);
    QCOMPARE(result, 1);
}

void TestOneToOneJunction::testGetLeftIdMany()
{
    insertTestData({{1, 101}, {2, 102}, {3, 103}});

    QList<int> rightIds = {101, 102, 404}; // 404 doesn't exist
    auto result = OneToOne::getLeftIdMany(m_db, m_junctionTableName, rightIds);

    QCOMPARE(result.size(), 2); // Only found records
    QVERIFY(result.contains(101));
    QCOMPARE(result[101], 1);
    QVERIFY(result.contains(102));
    QCOMPARE(result[102], 2);
    QVERIFY(!result.contains(404));
}

void TestOneToOneJunction::testGetLeftIdEmpty()
{
    auto result = OneToOne::getLeftIdMany(m_db, m_junctionTableName, {});
    QVERIFY(result.isEmpty());
}

void TestOneToOneJunction::testGetLeftIdNonExistent()
{
    insertTestData({{1, 101}});

    int result = OneToOne::getLeftId(m_db, m_junctionTableName, 999);
    QCOMPARE(result, -1);
}

void TestOneToOneJunction::testGetRightIdCount()
{
    insertTestData({{1, 101}});

    int count = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 1);
}

void TestOneToOneJunction::testGetRightIdCountZero()
{
    int count = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestOneToOneJunction::testGetRightIdCountNonExistent()
{
    insertTestData({{1, 101}});

    int count = OneToOne::getRightIdCount(m_db, 999, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestOneToOneJunction::testGetRightIdInRange()
{
    insertTestData({{1, 101}});

    auto result = OneToOne::getRightIdInRange(m_db, 1, m_junctionTableName);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 101);
}

void TestOneToOneJunction::testGetRightIdInRangeEmpty()
{
    auto result = OneToOne::getRightIdInRange(m_db, 1, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOneToOneJunction::testGetRightIdInRangeNonExistent()
{
    insertTestData({{1, 101}});

    auto result = OneToOne::getRightIdInRange(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOneToOneJunction::testOneToOneConstraintEnforcement()
{
    insertTestData({{1, 101}});

    // Test that updating to a different right_id works
    auto result = OneToOne::upsertRightId(m_db, 1, m_junctionTableName, 201);
    QCOMPARE(result.size(), 1);
    QCOMPARE(result[0], 201);

    // Verify only one right_id per left_id
    int count = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(count, 1);

    auto retrieved = OneToOne::getRightId(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.has_value());
    QCOMPARE(retrieved.value(), 201);
}

void TestOneToOneJunction::testCacheInvalidation()
{
    insertTestData({{1, 101}});

    // Get count to populate cache
    int initialCount = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(initialCount, 1);

    // Remove the record - this should invalidate cache
    bool success = OneToOne::removeWithLeftId(m_db, 1, m_junctionTableName);
    QVERIFY(success);

    // Count should now be 0 (cache should be invalidated)
    int newCount = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(newCount, 0);

    // Add back and test upsert cache invalidation
    OneToOne::upsertRightId(m_db, 1, m_junctionTableName, 102);
    int finalCount = OneToOne::getRightIdCount(m_db, 1, m_junctionTableName);
    QCOMPARE(finalCount, 1);
}

QTEST_MAIN(TestOneToOneJunction)

#include "tst_one_to_one_junction.moc"
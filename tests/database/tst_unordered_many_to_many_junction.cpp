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

#include "database/junction_table_ops/unordered_many_to_many.h"
#include "service_locator.h"
#include <QObject>
#include <QSignalSpy>
#include <QSqlDatabase>
#include <QSqlError>
#include <QSqlQuery>
#include <QString>
#include <QTest>
#include <memory>

using namespace Qt::StringLiterals;

namespace SCU = Skribisto::Common::Database;
namespace JUNCTIONOPS = Skribisto::Common::Database::JunctionTableOps;

class TestUnorderedManyToManyJunction : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    // Test cases for getRightIds functions
    void testGetRightIds();
    void testGetRightIdsMany();
    void testGetRightIdsEmpty();
    void testGetRightIdsNonExistent();

    // Test cases for removeWithLeftIds functions
    void testRemoveWithLeftIds();
    void testRemoveWithLeftIdsMany();
    void testRemoveWithLeftIdsEmpty();
    void testRemoveWithLeftIdsNonExistent();

    // Test cases for removeWithRightIds functions
    void testRemoveWithRightIds();
    void testRemoveWithRightIdsMany();
    void testRemoveWithRightIdsEmpty();
    void testRemoveWithRightIdsNonExistent();

    // Test cases for upsertRightIds functions
    void testUpsertRightIds();
    void testUpsertRightIdsMany();
    void testUpsertRightIdsOptional();
    void testUpsertRightIdsEmpty();
    void testUpsertRightIdsOverwrite();

    // Test cases for getLeftIds functions
    void testGetLeftIds();
    void testGetLeftIdsMany();
    void testGetLeftIdsEmpty();
    void testGetLeftIdsNonExistent();

    // Test cases for getRightIdsCount
    void testGetRightIdsCount();
    void testGetRightIdsCountZero();
    void testGetRightIdsCountNonExistent();

    // Test cases for getRightIdsInRange
    void testGetRightIdsInRange();
    void testGetRightIdsInRangeOffset();
    void testGetRightIdsInRangeLimit();
    void testGetRightIdsInRangeEmpty();

  private:
    QString m_junctionTableName = "test_junction"_L1;
    QString m_junctionTableDefinition = "CREATE TABLE IF NOT EXISTS test_junction ("
                                        "    left_id INTEGER NOT NULL,"
                                        "    right_id INTEGER NOT NULL,"
                                        "    PRIMARY KEY (left_id, right_id)"
                                        ");"_L1;
    QSqlDatabase m_db;

    void setupDatabase();
    void insertTestData(const QList<QPair<int, int>> &data);
    void clearJunctionTable();
};

void TestUnorderedManyToManyJunction::initTestCase()
{
    std::string duration("20000"); // 20 secondes
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
}

void TestUnorderedManyToManyJunction::cleanupTestCase()
{
}

void TestUnorderedManyToManyJunction::init()
{
    setupDatabase();
}

void TestUnorderedManyToManyJunction::cleanup()
{
    // Clear the junction cache to ensure test isolation
    SCU::JunctionTableOps::JunctionCache::instance().clear();

    if (m_db.isOpen())
    {
        m_db.close();
    }
    QSqlDatabase::removeDatabase(m_db.connectionName());
}

void TestUnorderedManyToManyJunction::setupDatabase()
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

void TestUnorderedManyToManyJunction::insertTestData(const QList<QPair<int, int>> &data)
{
    QSqlQuery query(m_db);
    query.prepare(QStringLiteral("INSERT INTO %1 (left_id, right_id) VALUES (?, ?)").arg(m_junctionTableName));

    for (const auto &pair : data)
    {
        query.addBindValue(pair.first);
        query.addBindValue(pair.second);
        QVERIFY(query.exec());
    }
}

void TestUnorderedManyToManyJunction::clearJunctionTable()
{
    QSqlQuery query(m_db);
    QVERIFY(query.exec(QStringLiteral("DELETE FROM %1").arg(m_junctionTableName)));
}

// Test cases for getRightIds functions
void TestUnorderedManyToManyJunction::testGetRightIds()
{
    // Insert test data: left_id 1 -> right_ids [10, 20, 30]
    insertTestData({{1, 10}, {1, 20}, {1, 30}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QVERIFY(result.contains(10));
    QVERIFY(result.contains(20));
    QVERIFY(result.contains(30));
}

void TestUnorderedManyToManyJunction::testGetRightIdsMany()
{
    // Insert test data:
    // left_id 1 -> right_ids [10, 20]
    // left_id 2 -> right_ids [30]
    // left_id 3 -> right_ids [] (no relationships)
    insertTestData({{1, 10}, {1, 20}, {2, 30}});

    QHash<int, QList<int>> result =
        JUNCTIONOPS::UnorderedManyToMany::getRightIdsMany(m_db, {1, 2, 3}, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[1].size(), 2);
    QVERIFY(result[1].contains(10));
    QVERIFY(result[1].contains(20));
    QCOMPARE(result[2].size(), 1);
    QVERIFY(result[2].contains(30));
    QCOMPARE(result[3].size(), 0);
}

void TestUnorderedManyToManyJunction::testGetRightIdsEmpty()
{
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());

    QHash<int, QList<int>> resultMany =
        JUNCTIONOPS::UnorderedManyToMany::getRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(resultMany.isEmpty());
}

void TestUnorderedManyToManyJunction::testGetRightIdsNonExistent()
{
    insertTestData({{1, 10}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

// Test cases for removeWithLeftIds functions
void TestUnorderedManyToManyJunction::testRemoveWithLeftIds()
{
    insertTestData({{1, 10}, {1, 20}, {2, 30}});

    bool result = JUNCTIONOPS::UnorderedManyToMany::removeWithLeftIds(m_db, 1, m_junctionTableName);
    QVERIFY(result);

    // Verify removal
    QList<int> remaining = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(remaining.isEmpty());

    // Verify other data is untouched
    QList<int> untouched = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);
    QCOMPARE(untouched.size(), 1);
    QVERIFY(untouched.contains(30));
}

void TestUnorderedManyToManyJunction::testRemoveWithLeftIdsMany()
{
    insertTestData({{1, 10}, {1, 20}, {2, 30}, {3, 40}});

    QHash<int, bool> result =
        JUNCTIONOPS::UnorderedManyToMany::removeWithLeftIdsMany(m_db, {1, 2}, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QVERIFY(result[1]);
    QVERIFY(result[2]);

    // Verify removals
    QVERIFY(JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName).isEmpty());
    QVERIFY(JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 2, m_junctionTableName).isEmpty());

    // Verify untouched data
    QList<int> untouched = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 3, m_junctionTableName);
    QCOMPARE(untouched.size(), 1);
    QVERIFY(untouched.contains(40));
}

void TestUnorderedManyToManyJunction::testRemoveWithLeftIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::UnorderedManyToMany::removeWithLeftIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestUnorderedManyToManyJunction::testRemoveWithLeftIdsNonExistent()
{
    insertTestData({{1, 10}});

    bool result = JUNCTIONOPS::UnorderedManyToMany::removeWithLeftIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should still return true even if nothing was removed
}

// Test cases for removeWithRightIds functions
void TestUnorderedManyToManyJunction::testRemoveWithRightIds()
{
    insertTestData({{1, 10}, {2, 10}, {1, 20}});

    bool result = JUNCTIONOPS::UnorderedManyToMany::removeWithRightIds(m_db, 10, m_junctionTableName);
    QVERIFY(result);

    // Verify removal - both (1,10) and (2,10) should be gone
    QList<int> remaining1 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> remaining2 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QVERIFY(!remaining1.contains(10));
    QVERIFY(!remaining2.contains(10));
    QVERIFY(remaining1.contains(20)); // (1,20) should remain
}

void TestUnorderedManyToManyJunction::testRemoveWithRightIdsMany()
{
    insertTestData({{1, 10}, {1, 20}, {2, 10}, {2, 30}});

    QHash<int, bool> result =
        JUNCTIONOPS::UnorderedManyToMany::removeWithRightIdsMany(m_db, {10, 20}, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QVERIFY(result[10]);
    QVERIFY(result[20]);

    // Verify removals
    QList<int> remaining1 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> remaining2 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QVERIFY(remaining1.isEmpty()); // All relationships for left_id 1 removed
    QCOMPARE(remaining2.size(), 1);
    QVERIFY(remaining2.contains(30)); // Only (2,30) remains
}

void TestUnorderedManyToManyJunction::testRemoveWithRightIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::UnorderedManyToMany::removeWithRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestUnorderedManyToManyJunction::testRemoveWithRightIdsNonExistent()
{
    insertTestData({{1, 10}});

    bool result = JUNCTIONOPS::UnorderedManyToMany::removeWithRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should still return true even if nothing was removed
}

// Test cases for upsertRightIds functions
void TestUnorderedManyToManyJunction::testUpsertRightIds()
{
    // Test initial insert
    QList<int> rightIds = {10, 20, 30};
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);

    QCOMPARE(result, rightIds);

    // Verify insertion
    QList<int> retrieved = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved.size(), 3);
    for (int id : rightIds)
    {
        QVERIFY(retrieved.contains(id));
    }
}

void TestUnorderedManyToManyJunction::testUpsertRightIdsMany()
{
    QHash<int, QList<int>> input;
    input[1] = {10, 20};
    input[2] = {30, 40};

    QHash<int, QList<int>> result =
        JUNCTIONOPS::UnorderedManyToMany::upsertRightIdsMany(m_db, input, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QCOMPARE(result[1], QList<int>({10, 20}));
    QCOMPARE(result[2], QList<int>({30, 40}));

    // Verify insertion
    QList<int> retrieved1 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> retrieved2 = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QCOMPARE(retrieved1.size(), 2);
    QCOMPARE(retrieved2.size(), 2);
    QVERIFY(retrieved1.contains(10) && retrieved1.contains(20));
    QVERIFY(retrieved2.contains(30) && retrieved2.contains(40));
}

void TestUnorderedManyToManyJunction::testUpsertRightIdsOptional()
{
    // Test with valid optional
    std::optional<QList<int>> rightIds = QList<int>({10, 20});
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);

    QCOMPARE(result.size(), 2);
    QVERIFY(result.contains(10) && result.contains(20));

    // Test with empty optional - should remove all relationships
    std::optional<QList<int>> emptyOptional;
    QList<int> emptyResult =
        JUNCTIONOPS::UnorderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, emptyOptional);

    QVERIFY(emptyResult.isEmpty());
    QList<int> retrieved = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.isEmpty());
}

void TestUnorderedManyToManyJunction::testUpsertRightIdsEmpty()
{
    QList<int> emptyList;
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, emptyList);

    QVERIFY(result.isEmpty());
    QList<int> retrieved = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.isEmpty());
}

void TestUnorderedManyToManyJunction::testUpsertRightIdsOverwrite()
{
    // Insert initial data
    insertTestData({{1, 10}, {1, 20}});

    // Overwrite with new data
    QList<int> newRightIds = {30, 40, 50};
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, newRightIds);

    QCOMPARE(result, newRightIds);

    // Verify old data is gone and new data is present
    QList<int> retrieved = JUNCTIONOPS::UnorderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved.size(), 3);
    QVERIFY(!retrieved.contains(10) && !retrieved.contains(20));
    QVERIFY(retrieved.contains(30) && retrieved.contains(40) && retrieved.contains(50));
}

// Test cases for getLeftIds functions
void TestUnorderedManyToManyJunction::testGetLeftIds()
{
    // Insert test data: right_id 10 <- left_ids [1, 2, 3]
    insertTestData({{1, 10}, {2, 10}, {3, 10}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getLeftIds(m_db, m_junctionTableName, 10);

    QCOMPARE(result.size(), 3);
    QVERIFY(result.contains(1));
    QVERIFY(result.contains(2));
    QVERIFY(result.contains(3));
}

void TestUnorderedManyToManyJunction::testGetLeftIdsMany()
{
    // Insert test data:
    // right_id 10 <- left_ids [1, 2]
    // right_id 20 <- left_ids [3]
    // right_id 30 <- left_ids [] (no relationships)
    insertTestData({{1, 10}, {2, 10}, {3, 20}});

    QMap<int, QList<int>> result =
        JUNCTIONOPS::UnorderedManyToMany::getLeftIdsMany(m_db, m_junctionTableName, {10, 20, 30});

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[10].size(), 2);
    QVERIFY(result[10].contains(1));
    QVERIFY(result[10].contains(2));
    QCOMPARE(result[20].size(), 1);
    QVERIFY(result[20].contains(3));
    QCOMPARE(result[30].size(), 0);
}

void TestUnorderedManyToManyJunction::testGetLeftIdsEmpty()
{
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getLeftIds(m_db, m_junctionTableName, 999);
    QVERIFY(result.isEmpty());

    QMap<int, QList<int>> resultMany = JUNCTIONOPS::UnorderedManyToMany::getLeftIdsMany(m_db, m_junctionTableName, {});
    QVERIFY(resultMany.isEmpty());
}

void TestUnorderedManyToManyJunction::testGetLeftIdsNonExistent()
{
    insertTestData({{1, 10}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getLeftIds(m_db, m_junctionTableName, 999);
    QVERIFY(result.isEmpty());
}

// Test cases for getRightIdsCount
void TestUnorderedManyToManyJunction::testGetRightIdsCount()
{
    insertTestData({{1, 10}, {1, 20}, {1, 30}, {2, 40}});

    int count1 = JUNCTIONOPS::UnorderedManyToMany::getRightIdsCount(m_db, 1, m_junctionTableName);
    int count2 = JUNCTIONOPS::UnorderedManyToMany::getRightIdsCount(m_db, 2, m_junctionTableName);

    QCOMPARE(count1, 3);
    QCOMPARE(count2, 1);
}

void TestUnorderedManyToManyJunction::testGetRightIdsCountZero()
{
    insertTestData({{1, 10}});

    int count = JUNCTIONOPS::UnorderedManyToMany::getRightIdsCount(m_db, 2, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestUnorderedManyToManyJunction::testGetRightIdsCountNonExistent()
{
    int count = JUNCTIONOPS::UnorderedManyToMany::getRightIdsCount(m_db, 999, m_junctionTableName);
    QCOMPARE(count, 0);
}

// Test cases for getRightIdsInRange
void TestUnorderedManyToManyJunction::testGetRightIdsInRange()
{
    insertTestData({{1, 10}, {1, 20}, {1, 30}, {1, 40}, {1, 50}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 3);

    QCOMPARE(result.size(), 3);
    // Note: Order is not guaranteed in unordered many-to-many, so we just check count
}

void TestUnorderedManyToManyJunction::testGetRightIdsInRangeOffset()
{
    insertTestData({{1, 10}, {1, 20}, {1, 30}, {1, 40}, {1, 50}});

    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 2, 2);

    QCOMPARE(result.size(), 2);
}

void TestUnorderedManyToManyJunction::testGetRightIdsInRangeLimit()
{
    insertTestData({{1, 10}, {1, 20}});

    // Request more than available
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 5);

    QCOMPARE(result.size(), 2);
}

void TestUnorderedManyToManyJunction::testGetRightIdsInRangeEmpty()
{
    QList<int> result = JUNCTIONOPS::UnorderedManyToMany::getRightIdsInRange(m_db, 999, m_junctionTableName, 0, 10);

    QVERIFY(result.isEmpty());
}

QTEST_MAIN(TestUnorderedManyToManyJunction)
#include "tst_unordered_many_to_many_junction.moc"

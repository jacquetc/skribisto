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

#include "database/junction_table_ops/ordered_many_to_many.h"
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

class TestOrderedManyToManyJunction : public QObject
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
    void testGetRightIdsOrdering();

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
    void testUpsertRightIdsPreservesOrder();

    // Test cases for getLeftIds functions
    void testGetLeftIds();
    void testGetLeftIdsMany();
    void testGetLeftIdsEmpty();
    void testGetLeftIdsNonExistent();
    void testGetLeftIdsOrdering();

    // Test cases for getRightIdsCount
    void testGetRightIdsCount();
    void testGetRightIdsCountZero();
    void testGetRightIdsCountNonExistent();

    // Test cases for getRightIdsInRange
    void testGetRightIdsInRange();
    void testGetRightIdsInRangeOffset();
    void testGetRightIdsInRangeLimit();
    void testGetRightIdsInRangeEmpty();
    void testGetRightIdsInRangeOrdering();

  private:
    QString m_junctionTableName = "test_junction"_L1;
    QString m_junctionTableDefinition = "CREATE TABLE IF NOT EXISTS test_junction ("
                                        "    left_id INTEGER NOT NULL,"
                                        "    right_id INTEGER NOT NULL,"
                                        "    order_ INTEGER NOT NULL,"
                                        "    PRIMARY KEY (left_id, right_id)"
                                        ");"_L1;
    QSqlDatabase m_db;

    void setupDatabase();
    void insertTestData(const QList<QPair<int, QPair<int, int>>> &data); // {left_id, {right_id, order_}}
    void clearJunctionTable();
};

void TestOrderedManyToManyJunction::initTestCase()
{
    std::string duration("20000"); // 20 secondes
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
}

void TestOrderedManyToManyJunction::cleanupTestCase()
{
}

void TestOrderedManyToManyJunction::init()
{
    setupDatabase();
}

void TestOrderedManyToManyJunction::cleanup()
{
    if (m_db.isOpen())
    {
        QString connectionName = m_db.connectionName();
        clearJunctionTable();
        {
            QSqlDatabase db = m_db;
            m_db = QSqlDatabase(); // Reset member to avoid dangling reference

            // Clear junction cache
            SCU::JunctionTableOps::JunctionCache::instance().clear();

            // Close and remove the database connection
            db.close();
        }
        QSqlDatabase::removeDatabase(connectionName);
    }
}

void TestOrderedManyToManyJunction::setupDatabase()
{
    // Create unique in-memory database for each test
    static int counter = 0;
    QString connectionName = QStringLiteral("test_connection_%1").arg(++counter);

    m_db = QSqlDatabase::addDatabase(QStringLiteral("QSQLITE"), connectionName);
    m_db.setDatabaseName(QStringLiteral(":memory:"));

    QVERIFY(m_db.open());

    // Create junction table with order_ column
    QSqlQuery query(m_db);
    QVERIFY(query.exec(m_junctionTableDefinition));
}

void TestOrderedManyToManyJunction::insertTestData(const QList<QPair<int, QPair<int, int>>> &data)
{
    QSqlQuery query(m_db);
    query.prepare(
        QStringLiteral("INSERT INTO %1 (left_id, right_id, order_) VALUES (?, ?, ?)").arg(m_junctionTableName));

    for (const auto &item : data)
    {
        query.addBindValue(item.first);           // left_id
        query.addBindValue(item.second.first);    // right_id
        query.addBindValue(item.second.second);   // order_
        QVERIFY(query.exec());
    }
}

void TestOrderedManyToManyJunction::clearJunctionTable()
{
    QSqlQuery query(m_db);
    QVERIFY(query.exec(QStringLiteral("DELETE FROM %1").arg(m_junctionTableName)));
}

// Test cases for getRightIds functions
void TestOrderedManyToManyJunction::testGetRightIds()
{
    // Insert test data: left_id 1 -> right_ids [10, 20, 30] with ordering
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[0], 10);
    QCOMPARE(result[1], 20);
    QCOMPARE(result[2], 30);
}

void TestOrderedManyToManyJunction::testGetRightIdsMany()
{
    // Insert test data:
    // left_id 1 -> right_ids [10, 20]
    // left_id 2 -> right_ids [30]
    // left_id 3 -> right_ids [] (no relationships)
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {30, 0}}});

    QHash<int, QList<int>> result =
        JUNCTIONOPS::OrderedManyToMany::getRightIdsMany(m_db, {1, 2, 3}, m_junctionTableName);

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[1].size(), 2);
    QCOMPARE(result[1][0], 10);
    QCOMPARE(result[1][1], 20);
    QCOMPARE(result[2].size(), 1);
    QCOMPARE(result[2][0], 30);
    QCOMPARE(result[3].size(), 0);
}

void TestOrderedManyToManyJunction::testGetRightIdsEmpty()
{
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());

    QHash<int, QList<int>> resultMany = JUNCTIONOPS::OrderedManyToMany::getRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(resultMany.isEmpty());
}

void TestOrderedManyToManyJunction::testGetRightIdsNonExistent()
{
    insertTestData({{1, {10, 0}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedManyToManyJunction::testGetRightIdsOrdering()
{
    // Insert data in non-sequential order_ values
    insertTestData({{1, {50, 4000}}, {1, {10, 0}}, {1, {30, 2000}}, {1, {20, 1000}}, {1, {40, 3000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);

    // Should be ordered by order_ column: 10, 20, 30, 40, 50
    QCOMPARE(result.size(), 5);
    QCOMPARE(result[0], 10);
    QCOMPARE(result[1], 20);
    QCOMPARE(result[2], 30);
    QCOMPARE(result[3], 40);
    QCOMPARE(result[4], 50);
}

// Test cases for removeWithLeftIds functions
void TestOrderedManyToManyJunction::testRemoveWithLeftIds()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {30, 0}}});

    bool result = JUNCTIONOPS::OrderedManyToMany::removeWithLeftIds(m_db, 1, m_junctionTableName);
    QVERIFY(result);

    // Verify removal
    QList<int> remaining = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(remaining.isEmpty());

    // Verify other data is untouched
    QList<int> untouched = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);
    QCOMPARE(untouched.size(), 1);
    QCOMPARE(untouched[0], 30);
}

void TestOrderedManyToManyJunction::testRemoveWithLeftIdsMany()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {30, 0}}, {3, {40, 0}}});

    QHash<int, bool> result = JUNCTIONOPS::OrderedManyToMany::removeWithLeftIdsMany(m_db, {1, 2}, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QVERIFY(result[1]);
    QVERIFY(result[2]);

    // Verify removals
    QVERIFY(JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName).isEmpty());
    QVERIFY(JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 2, m_junctionTableName).isEmpty());

    // Verify untouched data
    QList<int> untouched = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 3, m_junctionTableName);
    QCOMPARE(untouched.size(), 1);
    QCOMPARE(untouched[0], 40);
}

void TestOrderedManyToManyJunction::testRemoveWithLeftIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::OrderedManyToMany::removeWithLeftIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedManyToManyJunction::testRemoveWithLeftIdsNonExistent()
{
    insertTestData({{1, {10, 0}}});

    bool result = JUNCTIONOPS::OrderedManyToMany::removeWithLeftIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should still return true even if nothing was removed
}

// Test cases for removeWithRightIds functions
void TestOrderedManyToManyJunction::testRemoveWithRightIds()
{
    insertTestData({{1, {10, 0}}, {2, {10, 0}}, {1, {20, 1000}}});

    bool result = JUNCTIONOPS::OrderedManyToMany::removeWithRightIds(m_db, 10, m_junctionTableName);
    QVERIFY(result);

    // Verify removal - both (1,10) and (2,10) should be gone
    QList<int> remaining1 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> remaining2 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QVERIFY(!remaining1.contains(10));
    QVERIFY(!remaining2.contains(10));
    QCOMPARE(remaining1.size(), 1);
    QCOMPARE(remaining1[0], 20); // (1,20) should remain
}

void TestOrderedManyToManyJunction::testRemoveWithRightIdsMany()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {10, 0}}, {2, {30, 1000}}});

    QHash<int, bool> result = JUNCTIONOPS::OrderedManyToMany::removeWithRightIdsMany(m_db, {10, 20}, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QVERIFY(result[10]);
    QVERIFY(result[20]);

    // Verify removals
    QList<int> remaining1 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> remaining2 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QVERIFY(remaining1.isEmpty()); // All relationships for left_id 1 removed
    QCOMPARE(remaining2.size(), 1);
    QCOMPARE(remaining2[0], 30); // Only (2,30) remains
}

void TestOrderedManyToManyJunction::testRemoveWithRightIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::OrderedManyToMany::removeWithRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedManyToManyJunction::testRemoveWithRightIdsNonExistent()
{
    insertTestData({{1, {10, 0}}});

    bool result = JUNCTIONOPS::OrderedManyToMany::removeWithRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should still return true even if nothing was removed
}

// Test cases for upsertRightIds functions
void TestOrderedManyToManyJunction::testUpsertRightIds()
{
    // Test initial insert
    QList<int> rightIds = {10, 20, 30};
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);

    QCOMPARE(result, rightIds);

    // Verify insertion and ordering
    QList<int> retrieved = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, rightIds);
}

void TestOrderedManyToManyJunction::testUpsertRightIdsMany()
{
    QHash<int, QList<int>> input;
    input[1] = {10, 20};
    input[2] = {30, 40};

    QHash<int, QList<int>> result = JUNCTIONOPS::OrderedManyToMany::upsertRightIdsMany(m_db, input, m_junctionTableName);

    QCOMPARE(result.size(), 2);
    QCOMPARE(result[1], QList<int>({10, 20}));
    QCOMPARE(result[2], QList<int>({30, 40}));

    // Verify insertion and ordering
    QList<int> retrieved1 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> retrieved2 = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 2, m_junctionTableName);

    QCOMPARE(retrieved1, QList<int>({10, 20}));
    QCOMPARE(retrieved2, QList<int>({30, 40}));
}

void TestOrderedManyToManyJunction::testUpsertRightIdsOptional()
{
    // Test with valid optional
    std::optional<QList<int>> rightIds = QList<int>({10, 20});
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);

    QCOMPARE(result, QList<int>({10, 20}));

    // Test with empty optional - should remove all relationships
    std::optional<QList<int>> emptyOptional;
    QList<int> emptyResult = JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, emptyOptional);

    QVERIFY(emptyResult.isEmpty());
    QList<int> retrieved = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.isEmpty());
}

void TestOrderedManyToManyJunction::testUpsertRightIdsEmpty()
{
    QList<int> emptyList;
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, emptyList);

    QVERIFY(result.isEmpty());
    QList<int> retrieved = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(retrieved.isEmpty());
}

void TestOrderedManyToManyJunction::testUpsertRightIdsOverwrite()
{
    // Insert initial data
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}});

    // Overwrite with new data
    QList<int> newRightIds = {30, 40, 50};
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, newRightIds);

    QCOMPARE(result, newRightIds);

    // Verify old data is gone and new data is present with correct order
    QList<int> retrieved = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, newRightIds);
}

void TestOrderedManyToManyJunction::testUpsertRightIdsPreservesOrder()
{
    // Test that upsert preserves the order of input list
    QList<int> rightIds = {50, 10, 30, 20, 40};
    JUNCTIONOPS::OrderedManyToMany::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);

    QList<int> retrieved = JUNCTIONOPS::OrderedManyToMany::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, rightIds);
}

// Test cases for getLeftIds functions
void TestOrderedManyToManyJunction::testGetLeftIds()
{
    // Insert test data: right_id 10 <- left_ids [1, 2, 3]
    insertTestData({{1, {10, 0}}, {2, {10, 0}}, {3, {10, 0}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getLeftIds(m_db, m_junctionTableName, 10);

    QCOMPARE(result.size(), 3);
    QVERIFY(result.contains(1));
    QVERIFY(result.contains(2));
    QVERIFY(result.contains(3));
}

void TestOrderedManyToManyJunction::testGetLeftIdsMany()
{
    // Insert test data:
    // right_id 10 <- left_ids [1, 2]
    // right_id 20 <- left_ids [3]
    // right_id 30 <- left_ids [] (no relationships)
    insertTestData({{1, {10, 0}}, {2, {10, 1000}}, {3, {20, 0}}});

    QMap<int, QList<int>> result = JUNCTIONOPS::OrderedManyToMany::getLeftIdsMany(m_db, m_junctionTableName, {10, 20, 30});

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[10].size(), 2);
    QCOMPARE(result[10][0], 1);
    QCOMPARE(result[10][1], 2);
    QCOMPARE(result[20].size(), 1);
    QCOMPARE(result[20][0], 3);
    QCOMPARE(result[30].size(), 0);
}

void TestOrderedManyToManyJunction::testGetLeftIdsEmpty()
{
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getLeftIds(m_db, m_junctionTableName, 999);
    QVERIFY(result.isEmpty());

    QMap<int, QList<int>> resultMany = JUNCTIONOPS::OrderedManyToMany::getLeftIdsMany(m_db, m_junctionTableName, {});
    QVERIFY(resultMany.isEmpty());
}

void TestOrderedManyToManyJunction::testGetLeftIdsNonExistent()
{
    insertTestData({{1, {10, 0}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getLeftIds(m_db, m_junctionTableName, 999);
    QVERIFY(result.isEmpty());
}

void TestOrderedManyToManyJunction::testGetLeftIdsOrdering()
{
    // Insert data with specific order_ values
    insertTestData({{5, {10, 4000}}, {1, {10, 0}}, {3, {10, 2000}}, {2, {10, 1000}}, {4, {10, 3000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getLeftIds(m_db, m_junctionTableName, 10);

    // Should be ordered by order_ column: 1, 2, 3, 4, 5
    QCOMPARE(result.size(), 5);
    QCOMPARE(result[0], 1);
    QCOMPARE(result[1], 2);
    QCOMPARE(result[2], 3);
    QCOMPARE(result[3], 4);
    QCOMPARE(result[4], 5);
}

// Test cases for getRightIdsCount
void TestOrderedManyToManyJunction::testGetRightIdsCount()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}, {2, {40, 0}}});

    int count1 = JUNCTIONOPS::OrderedManyToMany::getRightIdsCount(m_db, 1, m_junctionTableName);
    int count2 = JUNCTIONOPS::OrderedManyToMany::getRightIdsCount(m_db, 2, m_junctionTableName);

    QCOMPARE(count1, 3);
    QCOMPARE(count2, 1);
}

void TestOrderedManyToManyJunction::testGetRightIdsCountZero()
{
    insertTestData({{1, {10, 0}}});

    int count = JUNCTIONOPS::OrderedManyToMany::getRightIdsCount(m_db, 2, m_junctionTableName);
    QCOMPARE(count, 0);
}

void TestOrderedManyToManyJunction::testGetRightIdsCountNonExistent()
{
    int count = JUNCTIONOPS::OrderedManyToMany::getRightIdsCount(m_db, 999, m_junctionTableName);
    QCOMPARE(count, 0);
}

// Test cases for getRightIdsInRange
void TestOrderedManyToManyJunction::testGetRightIdsInRange()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}, {1, {40, 3000}}, {1, {50, 4000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 3);

    QCOMPARE(result.size(), 3);
    QCOMPARE(result[0], 10);
    QCOMPARE(result[1], 20);
    QCOMPARE(result[2], 30);
}

void TestOrderedManyToManyJunction::testGetRightIdsInRangeOffset()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}, {1, {40, 3000}}, {1, {50, 4000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 2, 2);

    QCOMPARE(result.size(), 2);
    QCOMPARE(result[0], 30);
    QCOMPARE(result[1], 40);
}

void TestOrderedManyToManyJunction::testGetRightIdsInRangeLimit()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}});

    // Request more than available
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 5);

    QCOMPARE(result.size(), 2);
    QCOMPARE(result[0], 10);
    QCOMPARE(result[1], 20);
}

void TestOrderedManyToManyJunction::testGetRightIdsInRangeEmpty()
{
    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIdsInRange(m_db, 999, m_junctionTableName, 0, 10);

    QVERIFY(result.isEmpty());
}

void TestOrderedManyToManyJunction::testGetRightIdsInRangeOrdering()
{
    // Insert data in non-sequential order_ values
    insertTestData({{1, {50, 4000}}, {1, {10, 0}}, {1, {30, 2000}}, {1, {20, 1000}}, {1, {40, 3000}}});

    QList<int> result = JUNCTIONOPS::OrderedManyToMany::getRightIdsInRange(m_db, 1, m_junctionTableName, 1, 3);

    // Should return items at offset 1 with limit 3, ordered by order_: 20, 30, 40
    QCOMPARE(result.size(), 3);
    QCOMPARE(result[0], 20);
    QCOMPARE(result[1], 30);
    QCOMPARE(result[2], 40);
}

QTEST_MAIN(TestOrderedManyToManyJunction)
#include "tst_ordered_many_to_many_junction.moc"

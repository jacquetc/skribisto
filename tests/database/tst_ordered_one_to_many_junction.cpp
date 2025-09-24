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

#include "database/junction_table_ops/ordered_one_to_many.h"
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

using namespace Qt::StringLiterals;

namespace SCU = Skribisto::Common::Database;
namespace JUNCTIONOPS = Skribisto::Common::Database::JunctionTableOps::OrderedOneToMany;

class TestOrderedOneToManyJunction : public QObject
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
    void testUpsertRightIdsOrdering();

    // Test cases for getLeftId functions (one-to-many specific)
    void testGetLeftId();
    void testGetLeftIdMany();
    void testGetLeftIdEmpty();
    void testGetLeftIdNonExistent();
    void testGetLeftIdMultipleRightIds();

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

    // Test edge cases specific to ordered one-to-many
    void testOneToManyConstraint();
    void testOrderingPreservation();
    void testCacheInvalidation();

  private:
    QString m_junctionTableName = "test_junction"_L1;
    QString m_junctionTableDefinition = "CREATE TABLE IF NOT EXISTS test_junction ("
                                        "    left_id INTEGER NOT NULL,"
                                        "    right_id INTEGER NOT NULL UNIQUE,"
                                        "    order_ INTEGER NOT NULL,"
                                        "    PRIMARY KEY (left_id, right_id)"
                                        ");"_L1;
    QSqlDatabase m_db;
    
    void setupDatabase();
    void insertTestData(const QList<QPair<int, QPair<int, int>>> &data);
    void clearJunctionTable();
};

void TestOrderedOneToManyJunction::initTestCase()
{
    std::string duration("20000"); // 20 seconds
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
}

void TestOrderedOneToManyJunction::cleanupTestCase()
{
    m_db.close();
}

void TestOrderedOneToManyJunction::init()
{
    setupDatabase();
}

void TestOrderedOneToManyJunction::cleanup()
{
    if (m_db.isOpen())
    {
        clearJunctionTable();
        
        // Clear junction cache
        SCU::JunctionTableOps::JunctionCache::instance().clear();
        
        // Close and remove the database connection
        QString connectionName = m_db.connectionName();
        m_db.close();
        QSqlDatabase::removeDatabase(connectionName);
    }
}

void TestOrderedOneToManyJunction::setupDatabase()
{
    // Generate a truly unique connection name using thread ID and timestamp
    QString connectionName = QString("TestOrderedOneToManyJunction_%1_%2"_L1)
                                .arg(reinterpret_cast<quintptr>(QThread::currentThread()))
                                .arg(QDateTime::currentMSecsSinceEpoch());
    
    m_db = QSqlDatabase::addDatabase("QSQLITE"_L1, connectionName);
    m_db.setDatabaseName(":memory:"_L1);
    QVERIFY(m_db.open());
    
    QSqlQuery createTableQuery(m_db);
    QVERIFY(createTableQuery.exec(m_junctionTableDefinition));
}

void TestOrderedOneToManyJunction::insertTestData(const QList<QPair<int, QPair<int, int>>> &data)
{
    QSqlQuery insertQuery(m_db);
    insertQuery.prepare("INSERT INTO test_junction (left_id, right_id, order_) VALUES (?, ?, ?)"_L1);
    
    for (const auto &entry : data)
    {
        insertQuery.addBindValue(entry.first);
        insertQuery.addBindValue(entry.second.first);
        insertQuery.addBindValue(entry.second.second);
        QVERIFY(insertQuery.exec());
    }
}

void TestOrderedOneToManyJunction::clearJunctionTable()
{
    QSqlQuery clearQuery(m_db);
    QVERIFY(clearQuery.exec("DELETE FROM test_junction"_L1));
}

// Test getRightIds functions
void TestOrderedOneToManyJunction::testGetRightIds()
{
    // Test data: left_id=1 should return right_ids [10, 20, 30] in order
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}});
    
    QList<int> result = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QList<int> expected = {10, 20, 30};
    QCOMPARE(result, expected);
}

void TestOrderedOneToManyJunction::testGetRightIdsMany()
{
    // Test data for multiple left_ids with ordering
    insertTestData({
        {1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}},
        {2, {40, 0}}, {2, {50, 1000}},
        {3, {60, 0}}
    });
    
    QHash<int, QList<int>> result = JUNCTIONOPS::getRightIdsMany(m_db, {1, 2, 3}, m_junctionTableName);
    
    QCOMPARE(result[1], QList<int>({10, 20, 30}));
    QCOMPARE(result[2], QList<int>({40, 50}));
    QCOMPARE(result[3], QList<int>({60}));
}

void TestOrderedOneToManyJunction::testGetRightIdsEmpty()
{
    QList<int> result = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedOneToManyJunction::testGetRightIdsNonExistent()
{
    insertTestData({{1, {10, 0}}});
    
    QList<int> result = JUNCTIONOPS::getRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

// Test removeWithLeftIds functions
void TestOrderedOneToManyJunction::testRemoveWithLeftIds()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {30, 0}}});
    
    bool result = JUNCTIONOPS::removeWithLeftIds(m_db, 1, m_junctionTableName);
    QVERIFY(result);
    
    QList<int> remainingForLeft1 = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(remainingForLeft1.isEmpty());
    
    QList<int> remainingForLeft2 = JUNCTIONOPS::getRightIds(m_db, 2, m_junctionTableName);
    QCOMPARE(remainingForLeft2, QList<int>({30}));
}

void TestOrderedOneToManyJunction::testRemoveWithLeftIdsMany()
{
    insertTestData({{1, {10, 0}}, {2, {20, 0}}, {3, {30, 0}}});
    
    QHash<int, bool> result = JUNCTIONOPS::removeWithLeftIdsMany(m_db, {1, 2}, m_junctionTableName);
    QCOMPARE(result[1], true);
    QCOMPARE(result[2], true);
    
    QVERIFY(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName).isEmpty());
    QVERIFY(JUNCTIONOPS::getRightIds(m_db, 2, m_junctionTableName).isEmpty());
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 3, m_junctionTableName), QList<int>({30}));
}

void TestOrderedOneToManyJunction::testRemoveWithLeftIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::removeWithLeftIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedOneToManyJunction::testRemoveWithLeftIdsNonExistent()
{
    bool result = JUNCTIONOPS::removeWithLeftIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should succeed even if nothing to remove
}

// Test removeWithRightIds functions  
void TestOrderedOneToManyJunction::testRemoveWithRightIds()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {30, 0}}});
    
    bool result = JUNCTIONOPS::removeWithRightIds(m_db, 10, m_junctionTableName);
    QVERIFY(result);
    
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName), QList<int>({20}));
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 2, m_junctionTableName), QList<int>({30}));
}

void TestOrderedOneToManyJunction::testRemoveWithRightIdsMany()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {2, {40, 0}}, {2, {30, 1000}}});
    
    QHash<int, bool> result = JUNCTIONOPS::removeWithRightIdsMany(m_db, {10, 20}, m_junctionTableName);
    QCOMPARE(result[10], true);
    QCOMPARE(result[20], true);
    
    QVERIFY(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName).isEmpty());
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 2, m_junctionTableName), QList<int>({40, 30}));
}

void TestOrderedOneToManyJunction::testRemoveWithRightIdsEmpty()
{
    QHash<int, bool> result = JUNCTIONOPS::removeWithRightIdsMany(m_db, {}, m_junctionTableName);
    QVERIFY(result.isEmpty());
}

void TestOrderedOneToManyJunction::testRemoveWithRightIdsNonExistent()
{
    bool result = JUNCTIONOPS::removeWithRightIds(m_db, 999, m_junctionTableName);
    QVERIFY(result); // Should succeed even if nothing to remove
}

// Test upsertRightIds functions
void TestOrderedOneToManyJunction::testUpsertRightIds()
{
    QList<int> rightIds = {10, 20, 30};
    QList<int> result = JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);
    QCOMPARE(result, rightIds);
    
    QList<int> retrieved = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, rightIds);
}

void TestOrderedOneToManyJunction::testUpsertRightIdsMany()
{
    QHash<int, QList<int>> input;
    input[1] = {10, 20};
    input[2] = {30, 40, 50};
    
    QHash<int, QList<int>> result = JUNCTIONOPS::upsertRightIdsMany(m_db, input, m_junctionTableName);
    QCOMPARE(result[1], QList<int>({10, 20}));
    QCOMPARE(result[2], QList<int>({30, 40, 50}));
    
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName), QList<int>({10, 20}));
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 2, m_junctionTableName), QList<int>({30, 40, 50}));
}

void TestOrderedOneToManyJunction::testUpsertRightIdsOptional()
{
    // Test with nullopt
    std::optional<QList<int>> nullOpt;
    QList<int> result = JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, nullOpt);
    QVERIFY(result.isEmpty());
    
    // Test with value
    std::optional<QList<int>> withValue = QList<int>({10, 20});
    result = JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, withValue);
    QCOMPARE(result, QList<int>({10, 20}));
}

void TestOrderedOneToManyJunction::testUpsertRightIdsEmpty()
{
    QList<int> emptyList;
    QList<int> result = JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, emptyList);
    QVERIFY(result.isEmpty());
    
    QVERIFY(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName).isEmpty());
}

void TestOrderedOneToManyJunction::testUpsertRightIdsOverwrite()
{
    // Insert initial data
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, {10, 20});
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName), QList<int>({10, 20}));
    
    // Overwrite with new data
    QList<int> newRightIds = {30, 40, 50};
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, newRightIds);
    QCOMPARE(JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName), newRightIds);
}

void TestOrderedOneToManyJunction::testUpsertRightIdsOrdering()
{
    QList<int> rightIds = {30, 10, 20}; // Intentionally not in numeric order
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, rightIds);
    
    // Should preserve the insertion order
    QList<int> retrieved = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, rightIds); // Should maintain 30, 10, 20 order
}

// Test getLeftId functions
void TestOrderedOneToManyJunction::testGetLeftId()
{
    insertTestData({{1, {10, 0}}, {2, {20, 0}}});
    
    QCOMPARE(JUNCTIONOPS::getLeftId(m_db, m_junctionTableName, 10), 1);
    QCOMPARE(JUNCTIONOPS::getLeftId(m_db, m_junctionTableName, 20), 2);
}

void TestOrderedOneToManyJunction::testGetLeftIdMany()
{
    insertTestData({{1, {10, 0}}, {2, {20, 0}}, {3, {30, 0}}});
    
    QMap<int, int> result = JUNCTIONOPS::getLeftIdMany(m_db, m_junctionTableName, {10, 20, 30});
    QCOMPARE(result[10], 1);
    QCOMPARE(result[20], 2);
    QCOMPARE(result[30], 3);
}

void TestOrderedOneToManyJunction::testGetLeftIdEmpty()
{
    QMap<int, int> result = JUNCTIONOPS::getLeftIdMany(m_db, m_junctionTableName, {});
    QVERIFY(result.isEmpty());
}

void TestOrderedOneToManyJunction::testGetLeftIdNonExistent()
{
    QCOMPARE(JUNCTIONOPS::getLeftId(m_db, m_junctionTableName, 999), -1);
}

void TestOrderedOneToManyJunction::testGetLeftIdMultipleRightIds()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}});
    
    QCOMPARE(JUNCTIONOPS::getLeftId(m_db, m_junctionTableName, 10), 1);
    QCOMPARE(JUNCTIONOPS::getLeftId(m_db, m_junctionTableName, 20), 1);
}

// Test getRightIdsCount
void TestOrderedOneToManyJunction::testGetRightIdsCount()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}});
    
    QCOMPARE(JUNCTIONOPS::getRightIdsCount(m_db, 1, m_junctionTableName), 3);
}

void TestOrderedOneToManyJunction::testGetRightIdsCountZero()
{
    QCOMPARE(JUNCTIONOPS::getRightIdsCount(m_db, 1, m_junctionTableName), 0);
}

void TestOrderedOneToManyJunction::testGetRightIdsCountNonExistent()
{
    insertTestData({{1, {10, 0}}});
    QCOMPARE(JUNCTIONOPS::getRightIdsCount(m_db, 999, m_junctionTableName), 0);
}

// Test getRightIdsInRange
void TestOrderedOneToManyJunction::testGetRightIdsInRange()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}, {1, {40, 3000}}, {1, {50, 4000}}});
    
    QList<int> result = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 3);
    QCOMPARE(result, QList<int>({10, 20, 30}));
}

void TestOrderedOneToManyJunction::testGetRightIdsInRangeOffset()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}, {1, {40, 3000}}});
    
    QList<int> result = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 2, 2);
    QCOMPARE(result, QList<int>({30, 40}));
}

void TestOrderedOneToManyJunction::testGetRightIdsInRangeLimit()
{
    insertTestData({{1, {10, 0}}, {1, {20, 1000}}, {1, {30, 2000}}});
    
    QList<int> result = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 2);
    QCOMPARE(result, QList<int>({10, 20}));
}

void TestOrderedOneToManyJunction::testGetRightIdsInRangeEmpty()
{
    QList<int> result = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 5);
    QVERIFY(result.isEmpty());
}

void TestOrderedOneToManyJunction::testGetRightIdsInRangeOrdering()
{
    // Insert in non-sequential order values but with proper order_ values
    insertTestData({{1, {50, 0}}, {1, {30, 1000}}, {1, {10, 2000}}, {1, {40, 3000}}});
    
    QList<int> result = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 0, 4);
    QCOMPARE(result, QList<int>({50, 30, 10, 40})); // Should respect order_ column
}

// Test edge cases
void TestOrderedOneToManyJunction::testOneToManyConstraint()
{
    // In one-to-many, each right_id can only belong to one left_id
    insertTestData({{1, {10, 0}}});
    
    // Try to insert the same right_id with a different left_id - should fail due to UNIQUE constraint
    QSqlQuery conflictQuery(m_db);
    conflictQuery.prepare("INSERT INTO test_junction (left_id, right_id, order_) VALUES (?, ?, ?)"_L1);
    conflictQuery.addBindValue(2);
    conflictQuery.addBindValue(10); // Same right_id
    conflictQuery.addBindValue(0);
    
    QVERIFY(!conflictQuery.exec()); // Should fail due to UNIQUE constraint on right_id
}

void TestOrderedOneToManyJunction::testOrderingPreservation()
{
    QList<int> originalOrder = {100, 50, 200, 25, 300};
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, originalOrder);
    
    QList<int> retrieved = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(retrieved, originalOrder);
    
    // Test with getRightIdsInRange
    QList<int> partial = JUNCTIONOPS::getRightIdsInRange(m_db, 1, m_junctionTableName, 1, 3);
    QCOMPARE(partial, QList<int>({50, 200, 25}));
}

void TestOrderedOneToManyJunction::testCacheInvalidation()
{
    // Insert initial data
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, {10, 20});
    
    // Get data to populate cache
    QList<int> initial = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(initial, QList<int>({10, 20}));
    
    // Update data - cache should be invalidated
    JUNCTIONOPS::upsertRightIds(m_db, 1, m_junctionTableName, {30, 40});
    
    // Should get new data, not cached
    QList<int> updated = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QCOMPARE(updated, QList<int>({30, 40}));
    
    // Remove data - cache should be invalidated
    JUNCTIONOPS::removeWithLeftIds(m_db, 1, m_junctionTableName);
    
    QList<int> afterRemoval = JUNCTIONOPS::getRightIds(m_db, 1, m_junctionTableName);
    QVERIFY(afterRemoval.isEmpty());
}

QTEST_MAIN(TestOrderedOneToManyJunction)

#include "tst_ordered_one_to_many_junction.moc"
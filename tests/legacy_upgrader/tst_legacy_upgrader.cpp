#include <QFile>
#include <QSqlDatabase>
#include <QSqlQuery>
#include <QTemporaryDir>
#include <QTest>

#include "use_cases/load_work_uc/legacy_upgrader/legacy_upgrader.h"

using namespace Qt::StringLiterals;
using LegacyUpgrader = Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule::LegacyUpgrader;

// The test .skrib file is v1.8. We test migrateToV3 directly — the function that converts
// old tbl_tree schema to Qleany v3 tables. The old Upgrader::upgradeSQLite (incremental
// v1.0→v2.0 schema steps) is inherited code with a pre-existing SKRResult crash bug
// and needs separate fixing.

class TestLegacyUpgrader : public QObject
{
    Q_OBJECT

  private:
    QString m_tempFilePath;
    QTemporaryDir m_tempDir;
    LegacyUpgrader m_upgrader;

    static int s_connCounter;

    QString openDb(const QString &path)
    {
        const QString connName = u"test_conn_%1"_s.arg(++s_connCounter);
        QSqlDatabase db = QSqlDatabase::addDatabase(u"QSQLITE"_s, connName);
        db.setDatabaseName(path);
        if (!db.open())
        {
            QSqlDatabase::removeDatabase(connName);
            return {};
        }
        return connName;
    }

    void closeDb(const QString &connName)
    {
        QSqlDatabase::database(connName).close();
        QSqlDatabase::removeDatabase(connName);
    }

    int queryInt(const QString &connName, const QString &sql)
    {
        QSqlQuery q(QSqlDatabase::database(connName));
        q.exec(sql);
        return q.next() ? q.value(0).toInt() : -1;
    }

    QString queryString(const QString &connName, const QString &sql)
    {
        QSqlQuery q(QSqlDatabase::database(connName));
        q.exec(sql);
        return q.next() ? q.value(0).toString() : QString();
    }

    bool tableExists(const QString &connName, const QString &tableName)
    {
        QSqlQuery q(QSqlDatabase::database(connName));
        q.prepare(u"SELECT name FROM sqlite_master WHERE type='table' AND name=:n"_s);
        q.bindValue(u":n"_s, tableName);
        q.exec();
        return q.next();
    }

    // Run migrateToV3 on the temp file. Opens a connection, runs migration, closes it.
    bool runMigration()
    {
        auto connName = openDb(m_tempFilePath);
        if (connName.isEmpty())
            return false;
        bool ok = LegacyUpgrader::migrateToV3(connName);
        closeDb(connName);
        return ok;
    }

  private Q_SLOTS:
    void initTestCase()
    {
        qputenv("QTEST_FUNCTION_TIMEOUT", "20000");
        QVERIFY2(QFile::exists(QStringLiteral(SKRIB_TEST_FILE)), "Test .skrib file not found");
        QVERIFY(m_tempDir.isValid());
    }

    void init()
    {
        m_tempFilePath = m_tempDir.filePath(u"test_project.skrib"_s);
        QFile::remove(m_tempFilePath);
        QVERIFY(QFile::copy(QStringLiteral(SKRIB_TEST_FILE), m_tempFilePath));
        QFile::setPermissions(m_tempFilePath, QFile::ReadOwner | QFile::WriteOwner);
    }

    void cleanup()
    {
        QFile::remove(m_tempFilePath);
    }

    // ── Tests ────────────────────────────────────────────────────

    void testIsUpgradeNeeded()
    {
        QVERIFY(m_upgrader.isUpgradeNeeded(m_tempFilePath));
    }

    void testMigrateToV3Succeeds()
    {
        QVERIFY(runMigration());
    }

    void testOldTablesDropped()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QVERIFY(!tableExists(conn, u"tbl_tree"_s));
        QVERIFY(!tableExists(conn, u"tbl_tree_property"_s));
        QVERIFY(!tableExists(conn, u"tbl_tree_relationship"_s));
        QVERIFY(!tableExists(conn, u"tbl_project"_s));
        QVERIFY(!tableExists(conn, u"tbl_project_dict"_s));
        QVERIFY(!tableExists(conn, u"tbl_tag"_s));
        QVERIFY(!tableExists(conn, u"tbl_tag_relationship"_s));
        QVERIFY(!tableExists(conn, u"tbl_stat_history"_s));

        closeDb(conn);
    }

    void testV3TablesExist()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QVERIFY(tableExists(conn, u"work"_s));
        QVERIFY(tableExists(conn, u"binder"_s));
        QVERIFY(tableExists(conn, u"binder_item"_s));
        QVERIFY(tableExists(conn, u"binder_tag"_s));
        QVERIFY(tableExists(conn, u"content"_s));
        QVERIFY(tableExists(conn, u"dict_word"_s));
        QVERIFY(tableExists(conn, u"work_binders_to_binder_junction"_s));
        QVERIFY(tableExists(conn, u"work_tags_to_binder_tag_junction"_s));
        QVERIFY(tableExists(conn, u"work_dict_words_to_dict_word_junction"_s));
        QVERIFY(tableExists(conn, u"binder_binder_items_to_binder_item_junction"_s));
        QVERIFY(tableExists(conn, u"binder_item_contents_to_content_junction"_s));
        QVERIFY(tableExists(conn, u"binder_item_references_to_binder_item_junction"_s));
        QVERIFY(tableExists(conn, u"binder_item_tags_to_binder_tag_junction"_s));

        closeDb(conn);
    }

    void testWorkMetadata()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM work"_s), 1);
        QCOMPARE(queryString(conn, u"SELECT title FROM work"_s), u"test"_s);
        QCOMPARE(queryString(conn, u"SELECT dict_language FROM work"_s), u"fr"_s);
        // t_author is NULL in the test file → empty string after COALESCE or bind
        auto author = queryString(conn, u"SELECT author_name FROM work"_s);
        QVERIFY(author.isEmpty());

        closeDb(conn);
    }

    void testBinderCount()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder"_s), 2);

        // Check names and order
        QSqlQuery q(QSqlDatabase::database(conn));
        q.exec(u"SELECT b.name FROM binder b "
               "JOIN work_binders_to_binder_junction j ON j.right_id = b.id "
               "ORDER BY j.order_"_s);
        QVERIFY(q.next());
        QCOMPARE(q.value(0).toString(), u"Writings"_s);
        QVERIFY(q.next());
        QCOMPARE(q.value(0).toString(), u"Notes"_s);

        closeDb(conn);
    }

    void testBinderItemCount()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // 23 tree items (indent>0) - 2 indent-1 FOLDERs (became Binders) - 3 separators (skipped) = 18
        // (includes 1 stray "Part 1" at indent=1 TEXT, re-indented into first binder)
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item"_s), 18);

        closeDb(conn);
    }

    void testContentCount()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // 4 headings (1 book-title + 3 chapter-title) + 3 scene-text + 3 note-text + 4 synopsis-text = 14
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content"_s), 14);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content WHERE role = 'book-title'"_s), 1);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content WHERE role = 'chapter-title'"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content WHERE role = 'scene-text'"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content WHERE role = 'note-text'"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM content WHERE role = 'synopsis-text'"_s), 4);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item_contents_to_content_junction"_s), 14);

        closeDb(conn);
    }

    void testTagMigration()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_tag"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM work_tags_to_binder_tag_junction"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item_tags_to_binder_tag_junction"_s), 3);

        // All 3 tag relationships point to the same item ("1.1 Zeus")
        QSqlQuery q(QSqlDatabase::database(conn));
        q.exec(u"SELECT DISTINCT left_id FROM binder_item_tags_to_binder_tag_junction"_s);
        QVERIFY(q.next());
        int taggedItemId = q.value(0).toInt();
        QVERIFY(!q.next());
        QCOMPARE(queryString(conn, u"SELECT title FROM binder_item WHERE id = %1"_s.arg(taggedItemId)),
                 u"1.1 Zeus"_s);

        closeDb(conn);
    }

    void testDictWordMigration()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM dict_word"_s), 1);
        QCOMPARE(queryString(conn, u"SELECT word FROM dict_word"_s), u"test_project_dict_word"_s);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM work_dict_words_to_dict_word_junction"_s), 1);

        closeDb(conn);
    }

    void testReferenceMigration()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item_references_to_binder_item_junction"_s), 3);

        // All references point to the same target ("1.1 Zeus")
        QSqlQuery q(QSqlDatabase::database(conn));
        q.exec(u"SELECT DISTINCT right_id FROM binder_item_references_to_binder_item_junction"_s);
        QVERIFY(q.next());
        int targetId = q.value(0).toInt();
        QVERIFY(!q.next());
        QCOMPARE(queryString(conn, u"SELECT title FROM binder_item WHERE id = %1"_s.arg(targetId)),
                 u"1.1 Zeus"_s);

        closeDb(conn);
    }

    void testRolesAndSubRoles()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // Roles: FOLDER→"folder", TEXT/SECTION→"item"
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE role = 'folder'"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE role = 'item'"_s), 15);

        // SubRoles: 15 items with non-empty sub_role (separators are skipped entirely)
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role != ''"_s), 15);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'book-begin'"_s), 1);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'chapter'"_s), 3);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'book-end'"_s), 1);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'scene'"_s), 7);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'note'"_s), 3);

        // No separators should exist — they are dropped during migration
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE sub_role = 'separator'"_s), 0);

        closeDb(conn);
    }

    void testLabels()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE label != ''"_s), 1);
        QCOMPARE(queryString(conn, u"SELECT label FROM binder_item WHERE label != ''"_s),
                 u"this is a label"_s);

        closeDb(conn);
    }

    void testIndentAdjustment()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // No item should have negative indent
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE indent < 0"_s), 0);
        // Items exist at indent 0 and 1
        QVERIFY(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE indent = 0"_s) > 0);
        QVERIFY(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE indent = 1"_s) > 0);

        closeDb(conn);
    }

    void testActivatedState()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // 7 trashed tree items - 1 trashed separator (skipped) = 6 BinderItems with activated=0
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE activated = 0"_s), 6);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder_item WHERE activated = 1"_s), 12);

        // Both Binders active
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder WHERE activated = 1"_s), 2);
        QCOMPARE(queryInt(conn, u"SELECT COUNT(*) FROM binder WHERE activated = 0"_s), 0);

        // Content of trashed items should also be deactivated
        int trashedContent = queryInt(conn,
                                      u"SELECT COUNT(*) FROM content c "
                                      "JOIN binder_item_contents_to_content_junction j ON j.right_id = c.id "
                                      "JOIN binder_item bi ON bi.id = j.left_id "
                                      "WHERE bi.activated = 0"_s);
        int deactivatedContent = queryInt(conn, u"SELECT COUNT(*) FROM content WHERE activated = 0"_s);
        QCOMPARE(deactivatedContent, trashedContent);

        closeDb(conn);
    }

    void testStrayItemHandling()
    {
        QVERIFY(runMigration());
        auto conn = openDb(m_tempFilePath);
        QVERIFY(!conn.isEmpty());

        // "Part 1" was indent=1 TEXT (trashed) → should be in first binder with indent=0
        int firstBinderId = queryInt(conn,
                                     u"SELECT right_id FROM work_binders_to_binder_junction ORDER BY order_ LIMIT 1"_s);
        QVERIFY(firstBinderId > 0);

        QSqlQuery q(QSqlDatabase::database(conn));
        q.prepare(u"SELECT bi.title, bi.indent, bi.activated FROM binder_item bi "
                  "JOIN binder_binder_items_to_binder_item_junction j ON j.right_id = bi.id "
                  "WHERE j.left_id = :bid AND bi.title = 'Part 1'"_s);
        q.bindValue(u":bid"_s, firstBinderId);
        q.exec();
        QVERIFY(q.next());
        QCOMPARE(q.value(0).toString(), u"Part 1"_s);
        QCOMPARE(q.value(1).toInt(), 0);  // indent adjusted from 2 to 0 (2 - 2)
        QCOMPARE(q.value(2).toInt(), 0);  // trashed → activated=0

        closeDb(conn);
    }

    void testNotUpgradeNeededAfterMigration()
    {
        QVERIFY(runMigration());
        QVERIFY(!m_upgrader.isUpgradeNeeded(m_tempFilePath));
    }
};

int TestLegacyUpgrader::s_connCounter = 0;

QTEST_MAIN(TestLegacyUpgrader)
#include "tst_legacy_upgrader.moc"

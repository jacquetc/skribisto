#include "undo_redo/undo_redo_scopes.h"
#include <QtTest>

class TestScopes : public QObject
{
    Q_OBJECT

  private slots:
    void testScopes();
};

void TestScopes::testScopes()
{
    // Test the constructor with a QStringList
    QStringList scopeList = {"atelier", "author", "book"};
    Presenter::UndoRedo::Scopes scopes(scopeList);
    QCOMPARE(scopes.count(), 3);
    QVERIFY(scopes.hasScope("atelier"));
    QVERIFY(scopes.hasScope("author"));
    QVERIFY(scopes.hasScope("book"));
    QVERIFY(!scopes.hasScope("foo"));
    QCOMPARE(scopes.flags("atelier"), 1);
    QCOMPARE(scopes.flags("author"), 2);
    QCOMPARE(scopes.flags("book"), 4);
    QCOMPARE(scopes.flags(), QList<int>() << 1 << 2 << 4);

    // Test the constructor with a comma-separated string
    Presenter::UndoRedo::Scopes scopes2("atelier, author, book");
    QCOMPARE(scopes2.count(), 3);
    QVERIFY(scopes2.hasScope("atelier"));
    QVERIFY(scopes2.hasScope("author"));
    QVERIFY(scopes2.hasScope("book"));
    QVERIFY(!scopes2.hasScope("foo"));
    QCOMPARE(scopes2.flags("atelier"), 1);
    QCOMPARE(scopes2.flags("author"), 2);
    QCOMPARE(scopes2.flags("book"), 4);
    QCOMPARE(scopes2.flags(), QList<int>() << 1 << 2 << 4);

    // Test the "all" flag
    QCOMPARE(scopes.flagForAll(), 0xFFFFFFF);

    // Test the hasScope method with a Scope object
    Presenter::UndoRedo::Scope scope1;
    scope1.setScope(3); // "atelier" and "author"
    QVERIFY(scopes.hasScope(scope1));
    QVERIFY(!scope1.hasScopeFlag(scopes.flags("book"))); // no "book"

    // Test the createScopeFromString method
    auto scope3 = scopes.createScopeFromString("atelier|book");
    QCOMPARE(scope3.scope(), 5);
    auto scope4 = scopes2.createScopeFromString("atelier|author");
    QCOMPARE(scope4.scope(), 3);
}

QTEST_MAIN(TestScopes)
#include "tst_undo_redo_scope.moc"

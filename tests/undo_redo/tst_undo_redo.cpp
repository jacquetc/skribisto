#include "undo_redo/query_command.h"
#include "undo_redo/undo_redo_command.h"
#include "undo_redo/undo_redo_system.h"
#include <QtTest>

using namespace Presenter::UndoRedo;

class DummyCommand : public UndoRedoCommand
{

    // UndoRedoCommand interface
  public:
    DummyCommand(const QString &text) : UndoRedoCommand(text)
    {
    }

    Result<void> undo() override
    {
        return m_undoReturn;
    }
    void redo(QPromise<Result<void>> &progressPromise) override
    {

        // wait 100ms
        QThread::msleep(100);

        progressPromise.addResult(m_redoReturn);
    }

    void setUndoReturn(const Result<void> &result)
    {
        m_undoReturn = result;
    }
    void setRedoReturn(const Result<void> &result)
    {
        m_redoReturn = result;
    }

  private:
    Result<void> m_undoReturn;
    Result<void> m_redoReturn;
};

class UndoRedoSystemTest : public QObject
{
    Q_OBJECT

  private slots:
    void initTestCase()
    {
    }

    void testInitialState()
    {
        Scopes scopes(QStringList() << "author"
                                    << "document");
        UndoRedoSystem system(nullptr, scopes);

        QVERIFY(!system.canUndo());
        QVERIFY(!system.canRedo());
        QVERIFY(system.undoText().isEmpty());
        QVERIFY(system.redoText().isEmpty());
    }
    void testAuthorCommands()
    {
        Scopes scopes(QStringList() << "author");
        UndoRedoSystem system(nullptr, scopes);

        // Push two general commands
        system.push(new DummyCommand("Command 1"), "author");
        system.push(new DummyCommand("Command 2"), "author");

        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        QTRY_VERIFY_WITH_TIMEOUT(!system.canRedo(), 500);
        QCOMPARE(system.undoText(), QString("Command 2"));
        QCOMPARE(system.redoText(), QString());

        // Undo the first command
        system.undo();

        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        QTRY_VERIFY_WITH_TIMEOUT(system.canRedo(), 500);
        QCOMPARE(system.undoText(), QString("Command 1"));
        QCOMPARE(system.redoText(), QString("Command 2"));

        // Redo the first command
        system.redo();

        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        QVERIFY(!system.canRedo());
        QCOMPARE(system.undoText(), QString("Command 2"));
        QCOMPARE(system.redoText(), QString());

        // Push a third general command
        system.push(new DummyCommand("Command 3"), "author");

        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        QVERIFY(!system.canRedo());
        QCOMPARE(system.undoText(), QString("Command 3"));
        QCOMPARE(system.redoText(), QString());

        // Verify that the system is not running
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);

        // Undo all commands
        system.undo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        system.undo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        system.undo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);

        QTRY_VERIFY_WITH_TIMEOUT(!system.canUndo(), 500);
        QTRY_VERIFY_WITH_TIMEOUT(system.canRedo(), 500);
        QCOMPARE(system.undoText(), QString());
        QCOMPARE(system.redoText(), QString("Command 1"));

        // Redo all commands
        system.redo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        system.redo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        system.redo();
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);

        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        QTRY_VERIFY_WITH_TIMEOUT(!system.canRedo(), 500);
        QCOMPARE(system.undoText(), QString("Command 3"));
        QCOMPARE(system.redoText(), QString());

        // Verify that the system is not running
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
    }

    void testCommandInError()
    {

        Scopes scopes(QStringList() << "author");
        UndoRedoSystem system(nullptr, scopes);

        auto *command = new DummyCommand("Command 1");
        command->setRedoReturn(Result<void>(Error(Q_FUNC_INFO, Error::Critical, "test error", "this is an error")));

        QSignalSpy spy(&system, &UndoRedoSystem::errorSent);
        QVERIFY(spy.isValid());
        system.push(command, "author");

        QTRY_VERIFY_WITH_TIMEOUT(!system.canUndo(), 500);
        QVERIFY(spy.wait(500));
        QCOMPARE(spy.count(), 1);
    }
    void testMultiscopeCommands()
    {
        Scopes scopes(QStringList() << "author"
                                    << "document");
        UndoRedoSystem system(nullptr, scopes);

        QSignalSpy spy(&system, &UndoRedoSystem::stateChanged);

        // Push two general commands and two multi-scope commands
        system.push(new DummyCommand("Author Command 1"), "author");
        system.push(new DummyCommand("Document Command 1"), "document");
        system.push(new DummyCommand("Author Command 2"), "author");
        system.push(new DummyCommand("Author/Document Command 1"), "author, document");

        qDebug() << system.undoRedoTextList();
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Document Command 1");
        qDebug() << system.undoRedoTextList();
        QCOMPARE(system.queuedCommandTextListByScope("author"), QStringList() << "Author Command 2"
                                                                              << "Author/Document Command 1");
        qDebug() << system.undoRedoTextList();
        QCOMPARE(system.queuedCommandTextListByScope("document"), QStringList() << "Author/Document Command 1");
        qDebug() << system.undoRedoTextList();

        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);

        // verify that scope queues are empty
        QCOMPARE(system.queuedCommandTextListByScope("author"), QStringList());
        QCOMPARE(system.queuedCommandTextListByScope("document"), QStringList());

        // Undo the multi-scope command
        qDebug() << system.undoRedoTextList();
        QCOMPARE(system.currentIndex(), 3);
        QCOMPARE(system.undoText(), "Author/Document Command 1");
        system.undo();
        QCOMPARE(system.currentIndex(), 2);

        // Redo the multi-scope command
        QTRY_VERIFY_WITH_TIMEOUT(system.canRedo(), 500);
        QCOMPARE(system.redoText(), "Author/Document Command 1");
        system.redo();
        QCOMPARE(system.currentIndex(), 3);

        // Undo all commands
        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        QCOMPARE(system.undoText(), "Author/Document Command 1");
        system.undo();
        QCOMPARE(system.currentIndex(), 2);
        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        system.undo();
        QCOMPARE(system.currentIndex(), 1);
        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        system.undo();
        QCOMPARE(system.currentIndex(), 0);
        QTRY_VERIFY_WITH_TIMEOUT(system.canUndo(), 500);
        QCOMPARE(system.currentIndex(), 0);
        system.undo();
        QCOMPARE(system.currentIndex(), -1);
        QTRY_VERIFY_WITH_TIMEOUT(!system.isRunning(), 500);
        QVERIFY(!system.canUndo());
        QVERIFY(system.canRedo());

        system.push(new DummyCommand("Author/Document Command 2"), "author, document");

        QCOMPARE(system.undoRedoTextList(), QStringList({"Author/Document Command 2"}));
        QCOMPARE(system.queuedCommandTextListByScope("author"), QStringList());
        QCOMPARE(system.queuedCommandTextListByScope("document"), QStringList());
    }

    void testSetCurrentIndex()
    {
        Scopes scopes(QStringList() << "author");
        UndoRedoSystem system(nullptr, scopes);

        QSignalSpy spy(&system, &UndoRedoSystem::stateChanged);

        // Push three general commands
        system.push(new DummyCommand("Author Command 1"), "author");
        system.push(new DummyCommand("Author Command 2"), "author");
        system.push(new DummyCommand("Author Command 3"), "author");

        // Verify that the system is not running
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 2, 500);
        QCOMPARE(system.undoText(), QString("Author Command 3"));
        QCOMPARE(system.redoText(), QString());
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to 1
        system.setCurrentIndex(1);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 1, 500);
        QCOMPARE(system.undoText(), QString("Author Command 2"));
        QCOMPARE(system.redoText(), QString("Author Command 3"));
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to 0
        system.setCurrentIndex(0);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 0, 500);
        QCOMPARE(system.undoText(), QString("Author Command 1"));
        QCOMPARE(system.redoText(), QString("Author Command 2"));
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to 2, increasing by 2
        system.setCurrentIndex(2);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 2, 500);
        QCOMPARE(system.undoText(), QString("Author Command 3"));
        QCOMPARE(system.redoText(), QString());
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to 0, decreasing by 2
        system.setCurrentIndex(0);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 0, 500);
        QCOMPARE(system.undoText(), QString("Author Command 1"));
        QCOMPARE(system.redoText(), QString("Author Command 2"));
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to 3
        system.setCurrentIndex(3);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 2, 500);
        QCOMPARE(system.undoText(), QString("Author Command 3"));
        QCOMPARE(system.redoText(), QString());
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");

        // Set current index to -1
        system.setCurrentIndex(-1);
        QTRY_COMPARE_WITH_TIMEOUT(system.currentIndex(), 0, 500);
        QCOMPARE(system.undoText(), QString("Author Command 1"));
        QCOMPARE(system.redoText(), QString("Author Command 2"));
        QCOMPARE(system.undoRedoTextList(), QStringList() << "Author Command 1"
                                                          << "Author Command 2"
                                                          << "Author Command 3");
    }
};
QTEST_GUILESS_MAIN(UndoRedoSystemTest)

#include "tst_undo_redo.moc"

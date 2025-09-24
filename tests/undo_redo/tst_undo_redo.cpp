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

#include "service_locator.h"
#include "undo_redo/group_command.h"
#include "undo_redo/query_handler.h"
#include "undo_redo/undo_redo_command.h"
#include "undo_redo/undo_redo_manager.h"
#include "undo_redo/undo_redo_scopes.h"
#include "undo_redo/undo_redo_stack.h"
#include "undo_redo/undo_redo_system.h"
#include <QObject>
#include <QSignalSpy>
#include <QTest>
#include <memory>

using namespace Qt::StringLiterals;

namespace SCU = Skribisto::Common::UndoRedo;

class TestUndoRedo : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testBasicCommandExecution();
    void testCommandUndoRedo();
    void testCommandThreadExecution();
    void testBasicStackOperations();
    void testStackUndoRedo();
    void testMultiScopeManager();
    void testScopeIsolation();
    void testCommandGrouping();
    void testCommandMerging();
    void testQueryExecution();
    void testQueryThreadExecution();
    void testServiceLocatorRegistration();

  private:
};

void TestUndoRedo::initTestCase()
{
    std::string duration("20000"); // 20 secondes
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
}

void TestUndoRedo::cleanupTestCase()
{
}

void TestUndoRedo::init()
{
}

void TestUndoRedo::cleanup()
{
}
void TestUndoRedo::testBasicCommandExecution()
{
    // Arrange
    bool executed = false;
    auto command = std::make_shared<SCU::UndoRedoCommand>("Test Command"_L1);

    QSignalSpy finishedSpy(command.get(), &SCU::UndoRedoCommand::finished);
    command->setExecuteFunction([&executed](auto &) { executed = true; });
    command->asyncExecute();

    // Assert - wait for async completion
    QVERIFY(finishedSpy.wait(1000)); // Wait up to 1 second
    QVERIFY(executed);
    QCOMPARE(command->text(), "Test Command"_L1);
}

void TestUndoRedo::testCommandUndoRedo()
{
    // Arrange
    int value = 0;
    auto command = std::make_shared<SCU::UndoRedoCommand>("Increment Command"_L1);
    command->setExecuteFunction([&value](auto &) { value++; });
    command->setRedoFunction([&value]() {
        value++;
        return SCU::Result<void>();
    });
    command->setUndoFunction([&value]() {
        value--;
        return SCU::Result<void>();
    });

    QSignalSpy finishedSpy(command.get(), &SCU::UndoRedoCommand::finished);

    // Act & Assert
    command->asyncExecute();
    QVERIFY(finishedSpy.wait(1000));
    QCOMPARE(value, 1);

    finishedSpy.clear();
    command->asyncUndo();
    QVERIFY(finishedSpy.wait(1000));
    QCOMPARE(value, 0);

    finishedSpy.clear();
    command->asyncRedo();
    QVERIFY(finishedSpy.wait(1000));
    QCOMPARE(value, 1);
}

void TestUndoRedo::testCommandThreadExecution()
{
    // Arrange
    QThread::currentThread()->setObjectName("MainThread"_L1);
    QString executionThread;
    bool executed = false;

    auto command = std::make_shared<SCU::UndoRedoCommand>("Thread Test Command"_L1);
    command->setExecuteFunction([&executionThread, &executed](auto &) {
        executionThread = QThread::currentThread()->objectName();
        executed = true;
    });

    QSignalSpy finishedSpy(command.get(), &SCU::UndoRedoCommand::finished);

    // Act
    command->asyncExecute();

    // Assert - wait for async completion
    QVERIFY(finishedSpy.wait(1000));
    QVERIFY(executed);
    QCOMPARE(finishedSpy.count(), 1);
    // Verify execution on separate thread (QtConcurrent creates different thread)
    QVERIFY(executionThread != "MainThread"_L1);
}

void TestUndoRedo::testBasicStackOperations()
{
    // Arrange
    SCU::UndoRedoStack stack;

    // Test initial state
    QVERIFY(!stack.canUndo());
    QVERIFY(!stack.canRedo());
    QCOMPARE(stack.undoCount(), 0);
    QCOMPARE(stack.redoCount(), 0);
    QVERIFY(stack.undoText().isEmpty());
    QVERIFY(stack.redoText().isEmpty());

    // Create test commands
    auto command1 = std::make_shared<SCU::UndoRedoCommand>("Command 1"_L1);
    auto command2 = std::make_shared<SCU::UndoRedoCommand>("Command 2"_L1);

    // Test pushing commands
    stack.push(command1);
    QVERIFY(stack.canUndo());
    QVERIFY(!stack.canRedo());
    QCOMPARE(stack.undoCount(), 1);
    QCOMPARE(stack.redoCount(), 0);
    QCOMPARE(stack.undoText(), "Command 1"_L1);

    stack.push(command2);
    QVERIFY(stack.canUndo());
    QVERIFY(!stack.canRedo());
    QCOMPARE(stack.undoCount(), 2);
    QCOMPARE(stack.redoCount(), 0);
    QCOMPARE(stack.undoText(), "Command 2"_L1);

    // Test clear
    stack.clear();
    QVERIFY(!stack.canUndo());
    QVERIFY(!stack.canRedo());
    QCOMPARE(stack.undoCount(), 0);
    QCOMPARE(stack.redoCount(), 0);
}

void TestUndoRedo::testStackUndoRedo()
{
    // Arrange
    SCU::UndoRedoStack stack;
    int value = 0;

    auto command1 = std::make_shared<SCU::UndoRedoCommand>("Increment 1"_L1);
    command1->setExecuteFunction([&value](auto &) { value++; });
    command1->setRedoFunction([&value]() {
        value++;
        return SCU::Result<void>();
    });
    command1->setUndoFunction([&value]() {
        value--;
        return SCU::Result<void>();
    });

    auto command2 = std::make_shared<SCU::UndoRedoCommand>("Increment 2"_L1);
    command2->setExecuteFunction([&value](auto &) { value++; });
    command2->setRedoFunction([&value]() {
        value++;
        return SCU::Result<void>();
    });
    command2->setUndoFunction([&value]() {
        value--;
        return SCU::Result<void>();
    });

    QSignalSpy commandFinishedSpy(&stack, &SCU::UndoRedoStack::commandFinished);

    // Push and verify state
    stack.push(command1);
    QTest::qSleep(50); // Give some time for async execution
    stack.execute();
    qDebug() << "Value after command1 execution:" << value;
    stack.push(command2);
    stack.execute();
    QTest::qSleep(50); // Give some time for async execution
    qDebug() << "Value after command2 execution:" << value;

    QCOMPARE(stack.undoCount(), 2);
    QCOMPARE(stack.redoCount(), 0);
    QCOMPARE(value, 2);

    // Test undo
    stack.undo();
    QTest::qSleep(50); // Give some time for async execution

    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(value, 1); // Command2 undo executed
    QCOMPARE(stack.undoCount(), 1);
    QCOMPARE(stack.redoCount(), 1);
    QCOMPARE(stack.redoText(), "Increment 2"_L1);

    // Test redo
    commandFinishedSpy.clear();
    stack.redo();
    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(value, 2); // Command2 redo executed (value becomes 2)
    QCOMPARE(stack.undoCount(), 2);
    QCOMPARE(stack.redoCount(), 0);

    // Test multiple undos
    commandFinishedSpy.clear();
    stack.undo();
    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(value, 1); // Command2 undo

    commandFinishedSpy.clear();
    stack.undo();
    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(value, 0); // Command1 undo

    QCOMPARE(stack.undoCount(), 0);
    QCOMPARE(stack.redoCount(), 2);
}

void TestUndoRedo::testMultiScopeManager()
{
    // QSKIP("Multi-scope manager test temporarily disabled - has implementation issues causing timeouts");
    //   Arrange
    SCU::UndoRedoManager manager;

    // Create different scopes
    auto projectScope1 = SCU::UndoRedoScope::projectScope(1);
    auto projectScope2 = SCU::UndoRedoScope::projectScope(2);
    auto contentScope1 = SCU::UndoRedoScope::contentScope(10);

    // Test initial state
    QCOMPARE(manager.currentScope(), SCU::UndoRedoScope::rootScope());
    QVERIFY(!manager.canUndo());
    QVERIFY(!manager.canRedo());
    QVERIFY(manager.activeScopes().isEmpty());

    // Create commands for different scopes
    int value1 = 0, value2 = 0, value3 = 0;

    auto cmd1 = std::make_shared<SCU::UndoRedoCommand>("Project1 Command"_L1);
    cmd1->setExecuteFunction([&value1](auto &) { value1++; });
    cmd1->setRedoFunction([&value1]() {
        value1++;
        return SCU::Result<void>();
    });
    cmd1->setUndoFunction([&value1]() {
        value1--;
        return SCU::Result<void>();
    });

    auto cmd2 = std::make_shared<SCU::UndoRedoCommand>("Project2 Command"_L1);
    cmd2->setExecuteFunction([&value2](auto &) { value2++; });
    cmd2->setRedoFunction([&value2]() {
        value2++;
        return SCU::Result<void>();
    });
    cmd2->setUndoFunction([&value2]() {
        value2--;
        return SCU::Result<void>();
    });

    auto cmd3 = std::make_shared<SCU::UndoRedoCommand>("Content1 Command"_L1);
    cmd3->setExecuteFunction([&value3](auto &) { value3++; });
    cmd3->setRedoFunction([&value3]() {
        value3++;
        return SCU::Result<void>();
    });
    cmd3->setUndoFunction([&value3]() {
        value3--;
        return SCU::Result<void>();
    });

    // Push commands to different scopes
    manager.pushCommand(cmd1, projectScope1);
    manager.pushCommand(cmd2, projectScope2);
    manager.pushCommand(cmd3, contentScope1);

    // Verify active scopes
    auto activeScopes = manager.activeScopes();
    QCOMPARE(activeScopes.size(), 3);
    QVERIFY(activeScopes.contains(projectScope1));
    QVERIFY(activeScopes.contains(projectScope2));
    QVERIFY(activeScopes.contains(contentScope1));

    // Verify each scope has commands
    QVERIFY(manager.canUndo(projectScope1));
    QVERIFY(manager.canUndo(projectScope2));
    QVERIFY(manager.canUndo(contentScope1));
    QCOMPARE(manager.undoCount(projectScope1), 1);
    QCOMPARE(manager.undoCount(projectScope2), 1);
    QCOMPARE(manager.undoCount(contentScope1), 1);

    // Test switching current scope
    QSignalSpy currentScopeSpy(&manager, &SCU::UndoRedoManager::currentScopeChanged);
    manager.setCurrentScope(projectScope1);
    QVERIFY(currentScopeSpy.count() == 1);
    QCOMPARE(manager.currentScope(), projectScope1);
    QVERIFY(manager.canUndo()); // Current scope method
    QCOMPARE(manager.undoText(), "Project1 Command"_L1);

    // Test scope isolation without command execution
    // Each scope should have independent state
    QCOMPARE(manager.undoText(projectScope1), "Project1 Command"_L1);
    QCOMPARE(manager.undoText(projectScope2), "Project2 Command"_L1);
    QCOMPARE(manager.undoText(contentScope1), "Content1 Command"_L1);

    // Verify each scope can independently track commands
    QVERIFY(manager.canUndo(projectScope1));
    QVERIFY(manager.canUndo(projectScope2));
    QVERIFY(manager.canUndo(contentScope1));
    QVERIFY(!manager.canRedo(projectScope1));
    QVERIFY(!manager.canRedo(projectScope2));
    QVERIFY(!manager.canRedo(contentScope1));

    // Test clearing specific scope
    manager.clearScope(projectScope2);
    QVERIFY(!manager.canUndo(projectScope2));
    QVERIFY(!manager.canRedo(projectScope2));
    QCOMPARE(manager.undoCount(projectScope2), 0);

    // Other scopes should remain unaffected
    QVERIFY(manager.canUndo(projectScope1));
    QVERIFY(manager.canUndo(contentScope1));
}

void TestUndoRedo::testScopeIsolation()
{
    // QSKIP("Scope isolation test temporarily disabled - has implementation issues causing timeouts");
    //  Arrange
    SCU::UndoRedoManager manager;

    auto scope1 = SCU::UndoRedoScope::projectScope(100);
    auto scope2 = SCU::UndoRedoScope::contentScope(200);

    // Create isolated variables for each scope
    int scope1Value = 10;
    int scope2Value = 20;

    // Create commands for scope 1
    auto scope1Cmd1 = std::make_shared<SCU::UndoRedoCommand>("Scope1 Increment"_L1);
    scope1Cmd1->setExecuteFunction([&scope1Value](auto &) { scope1Value += 5; });
    scope1Cmd1->setRedoFunction([&scope1Value]() {
        scope1Value += 5;
        return SCU::Result<void>();
    });
    scope1Cmd1->setUndoFunction([&scope1Value]() {
        scope1Value -= 5;
        return SCU::Result<void>();
    });

    auto scope1Cmd2 = std::make_shared<SCU::UndoRedoCommand>("Scope1 Double"_L1);
    scope1Cmd2->setExecuteFunction([&scope1Value](auto &) { scope1Value *= 2; });
    scope1Cmd2->setRedoFunction([&scope1Value]() {
        scope1Value *= 2;
        return SCU::Result<void>();
    });
    scope1Cmd2->setUndoFunction([&scope1Value]() {
        scope1Value /= 2;
        return SCU::Result<void>();
    });

    // Create commands for scope 2
    auto scope2Cmd1 = std::make_shared<SCU::UndoRedoCommand>("Scope2 Decrement"_L1);
    scope2Cmd1->setExecuteFunction([&scope2Value](auto &) { scope2Value -= 3; });
    scope2Cmd1->setRedoFunction([&scope2Value]() {
        scope2Value -= 3;

        return SCU::Result<void>();
    });
    scope2Cmd1->setUndoFunction([&scope2Value]() {
        scope2Value += 3;
        return SCU::Result<void>();
    });

    auto scope2Cmd2 = std::make_shared<SCU::UndoRedoCommand>("Scope2 Multiply"_L1);
    scope2Cmd2->setExecuteFunction([&scope2Value](auto &) { scope2Value *= 3; });
    scope2Cmd2->setRedoFunction([&scope2Value]() {
        scope2Value *= 3;
        return SCU::Result<void>();
    });
    scope2Cmd2->setUndoFunction([&scope2Value]() {
        scope2Value /= 3;
        return SCU::Result<void>();
    });
    QTest::qSleep(50); // Give some time for async execution

    // Verify initial isolation
    QCOMPARE(scope1Value, 10);
    QCOMPARE(scope2Value, 20);

    // Push commands to their respective scopes, without executing them
    manager.pushCommand(scope1Cmd1, scope1);
    manager.pushCommand(scope1Cmd2, scope1);
    manager.pushCommand(scope2Cmd1, scope2);
    manager.pushCommand(scope2Cmd2, scope2);

    // Verify scope counts
    QCOMPARE(manager.undoCount(scope1), 2);
    QCOMPARE(manager.undoCount(scope2), 2);
    QCOMPARE(manager.redoCount(scope1), 0);
    QCOMPARE(manager.redoCount(scope2), 0);

    // Test isolated scope state tracking (without execution)
    // Verify scope isolation - commands are tracked independently
    QCOMPARE(manager.undoText(scope1), "Scope1 Double"_L1);   // Last command pushed
    QCOMPARE(manager.undoText(scope2), "Scope2 Multiply"_L1); // Last command pushed

    // Each scope maintains independent command counts
    QVERIFY(manager.canUndo(scope1));
    QVERIFY(manager.canUndo(scope2));
    QVERIFY(!manager.canRedo(scope1));
    QVERIFY(!manager.canRedo(scope2));

    // Verify isolated text retrieval
    QCOMPARE(manager.undoText(scope1), "Scope1 Double"_L1);
    QCOMPARE(manager.undoText(scope2), "Scope2 Multiply"_L1);

    // Test clearing one scope doesn't affect the other
    manager.clearScope(scope1);
    QCOMPARE(manager.undoCount(scope1), 0);
    QCOMPARE(manager.redoCount(scope1), 0);
    QCOMPARE(manager.undoCount(scope2), 2); // Still has commands
    QCOMPARE(manager.redoCount(scope2), 0); // Still has commands

    // Verify scope2 still works after scope1 cleared
    QVERIFY(!manager.canUndo(scope1)); // scope1 cleared
    QVERIFY(manager.canUndo(scope2));  // scope2 still has commands
    QCOMPARE(manager.undoText(scope2), "Scope2 Multiply"_L1);
}

void TestUndoRedo::testCommandGrouping()
{
    // Arrange
    int value1 = 0, value2 = 10, value3 = 100;

    auto command1 = std::make_shared<SCU::UndoRedoCommand>("Increment Value1"_L1);
    command1->setExecuteFunction([&value1](auto &) { value1++; });
    command1->setRedoFunction([&value1]() {
        value1++;
        return SCU::Result<void>();
    });
    command1->setUndoFunction([&value1]() {
        value1--;
        return SCU::Result<void>();
    });

    auto command2 = std::make_shared<SCU::UndoRedoCommand>("Increment Value2"_L1);

    command2->setExecuteFunction([&value2](auto &) { value2++; });
    command2->setRedoFunction([&value2]() {
        value2++;
        return SCU::Result<void>();
    });
    command2->setUndoFunction([&value2]() {
        value2--;
        return SCU::Result<void>();
    });

    auto command3 = std::make_shared<SCU::UndoRedoCommand>("Increment Value3"_L1);
    command3->setExecuteFunction([&value3](auto &) { value3++; });
    command3->setRedoFunction([&value3]() {
        value3++;
        return SCU::Result<void>();
    });
    command3->setUndoFunction([&value3]() {
        value3--;
        return SCU::Result<void>();
    });

    // Create group command
    auto groupCommand = std::make_shared<SCU::GroupCommand>("Group Increment"_L1);
    groupCommand->addCommand(command1);
    groupCommand->addCommand(command2);
    groupCommand->addCommand(command3);

    QCOMPARE(groupCommand->commandCount(), 3);
    QCOMPARE(groupCommand->command(0), command1);
    QCOMPARE(groupCommand->text(), "Group Increment"_L1);

    QSignalSpy finishedSpy(groupCommand.get(), &SCU::GroupCommand::finished);

    // Act - Execute group execute
    groupCommand->asyncExecute();
    QVERIFY(finishedSpy.wait(2000)); // Wait longer for group execution
    QCOMPARE(finishedSpy.count(), 1);

    // Assert - All commands executed
    QCOMPARE(value1, 1);
    QCOMPARE(value2, 11);
    QCOMPARE(value3, 101);

    // Act - Execute group undo
    finishedSpy.clear();
    groupCommand->asyncUndo();
    QVERIFY(finishedSpy.wait(2000));
    QVERIFY(finishedSpy.count() == 1);

    // Assert - All commands undone (in reverse order)
    QCOMPARE(value1, 0);
    QCOMPARE(value2, 10);
    QCOMPARE(value3, 100);

    // Test with stack
    SCU::UndoRedoStack stack;
    stack.push(groupCommand);

    QSignalSpy stackFinishedSpy(&stack, &SCU::UndoRedoStack::commandFinished);

    stack.undo();
    QVERIFY(stackFinishedSpy.wait(2000));

    // Values should be modified again (group undo executed)
    QCOMPARE(value1, -1);
    QCOMPARE(value2, 9);
    QCOMPARE(value3, 99);
}

void TestUndoRedo::testCommandMerging()
{
    // Test mergeable commands - simulating typing characters
    QString text = "Hello"_L1;

    // Create a mergeable command that adds a character
    class TypeCommand : public SCU::UndoRedoCommand
    {
      public:
        TypeCommand(int pos, const QString &ch, QString *textRef)
            : UndoRedoCommand("Type '"_L1 + ch + "'"_L1), m_position(pos), m_character(ch), m_textRef(textRef)
        {
            setExecuteFunction([this](auto &) { m_textRef->insert(m_position, m_character); });
            setRedoFunction([this]() {
                m_textRef->insert(m_position, m_character);
                return SCU::Result<void>();
            });
            setUndoFunction([this]() {
                m_textRef->remove(m_position, m_character.length());
                return SCU::Result<void>();
            });
        }

        // Override for mergeability
        bool canMergeWith(const std::shared_ptr<UndoRedoCommand> &other) const override
        {
            auto typeCmd = std::dynamic_pointer_cast<TypeCommand>(other);
            if (!typeCmd)
                return false;

            // Can merge if positions are consecutive and text references are same
            return (typeCmd->m_position == m_position + m_character.length()) && (typeCmd->m_textRef == m_textRef);
        }

        void mergeWith(const std::shared_ptr<UndoRedoCommand> &other) override
        {
            auto typeCmd = std::dynamic_pointer_cast<TypeCommand>(other);
            if (typeCmd && canMergeWith(other))
            {
                m_character += typeCmd->m_character;
                setText("Type '"_L1 + m_character + "'"_L1);

                // Update redo function for merged command
                setExecuteFunction([this](auto &) { m_textRef->insert(m_position, m_character); });
                setRedoFunction([this]() {
                    m_textRef->insert(m_position, m_character);
                    return SCU::Result<void>();
                });
                setUndoFunction([this]() {
                    m_textRef->remove(m_position, m_character.length());
                    return SCU::Result<void>();
                });
            }
        }

      private:
        int m_position;
        QString m_character;
        QString *m_textRef;
    };

    SCU::UndoRedoStack stack;
    QSignalSpy commandFinishedSpy(&stack, &SCU::UndoRedoStack::commandFinished);

    // Push first command
    auto cmd1 = std::make_shared<TypeCommand>(text.length(), " "_L1, &text);
    stack.push(cmd1);
    QCOMPARE(stack.undoCount(), 1);

    // Push mergeable command
    auto cmd2 = std::make_shared<TypeCommand>(text.length() + 1, "W"_L1, &text);
    stack.push(cmd2);

    // Should still have 1 command due to merging
    QCOMPARE(stack.undoCount(), 1);
    QCOMPARE(stack.undoText(), "Type ' W'"_L1);

    // Push another mergeable command
    auto cmd3 = std::make_shared<TypeCommand>(text.length() + 2, "o"_L1, &text);
    stack.push(cmd3);
    stack.execute();
    QTest::qSleep(50); // Give some time for async execution

    // Should still have 1 command
    QCOMPARE(stack.undoCount(), 1);
    QCOMPARE(stack.undoText(), "Type ' Wo'"_L1);

    // Test execution - undo should remove all merged characters
    QCOMPARE(text, "Hello Wo"_L1);
    commandFinishedSpy.clear();
    stack.undo();
    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(text, "Hello"_L1); // Text unchanged because undo removes what was added

    // Redo should add merged text
    commandFinishedSpy.clear();
    stack.redo();
    QTest::qSleep(50); // Give some time for async execution
    QVERIFY(commandFinishedSpy.wait(1000));
    QCOMPARE(text, "Hello Wo"_L1); // Should add the merged " Wo"

    // Test non-mergeable command
    auto nonMergeableCmd = std::make_shared<SCU::UndoRedoCommand>("Delete All"_L1);
    nonMergeableCmd->setExecuteFunction([&text](auto &) { text.clear(); });
    nonMergeableCmd->setRedoFunction([&text]() {
        text.clear();
        return SCU::Result<void>();
    });
    nonMergeableCmd->setUndoFunction([&text]() {
        text = "Hello"_L1;
        return SCU::Result<void>();
    });

    stack.push(nonMergeableCmd);
    QCOMPARE(stack.undoCount(), 2); // Should not merge, so now we have 2 commands
}

void TestUndoRedo::testQueryExecution()
{
    // Arrange
    SCU::QueryHandler handler;
    auto query = handler.createQuery<int>("Test Query"_L1);

    int computedValue = 0;
    query->setQueryFunction([&computedValue]() -> int {
        computedValue = 42;
        return computedValue;
    });

    QSignalSpy queryFinishedSpy(&handler, &SCU::QueryHandler::queryFinished);
    QSignalSpy queryBaseFinishedSpy(query.get(), &SCU::QueryBase::finished);

    QCOMPARE(query->description(), "Test Query"_L1);
    QVERIFY(!query->result().isValid()); // No result yet

    // Act
    handler.executeQuery(query);

    // Assert
    QVERIFY(queryFinishedSpy.wait(1000));
    QVERIFY(queryBaseFinishedSpy.count() >= 1);
    QCOMPARE(queryFinishedSpy.count(), 1);

    // Verify query finished with success
    auto args = queryFinishedSpy.takeFirst();
    QVERIFY(args.at(1).toBool()); // success = true

    // Check result
    QCOMPARE(computedValue, 42);
    QCOMPARE(query->typedResult(), 42);
    QCOMPARE(query->result().toInt(), 42);
}

void TestUndoRedo::testQueryThreadExecution()
{
    // Arrange
    QThread::currentThread()->setObjectName("MainThread"_L1);
    QString executionThread;

    SCU::QueryHandler handler;
    auto query = handler.createQuery<QString>("Thread Test Query"_L1);

    query->setQueryFunction([&executionThread]() -> QString {
        executionThread = QThread::currentThread()->objectName();
        return "ThreadResult"_L1;
    });

    QSignalSpy queryFinishedSpy(&handler, &SCU::QueryHandler::queryFinished);

    // Act
    handler.executeQuery(query);

    // Assert
    QVERIFY(queryFinishedSpy.wait(1000));
    QCOMPARE(queryFinishedSpy.count(), 1);

    // Verify execution on separate thread
    QVERIFY(executionThread != "MainThread"_L1);
    QCOMPARE(query->typedResult(), "ThreadResult"_L1);

    // Verify query finished with success
    auto args = queryFinishedSpy.takeFirst();
    QVERIFY(args.at(1).toBool()); // success = true
}

void TestUndoRedo::testServiceLocatorRegistration()
{
    // Arrange
    auto locator = std::make_unique<Skribisto::Common::ServiceLocator>();
    auto undoRedoSystem = std::make_unique<SCU::UndoRedoSystem>();

    // Test initial state
    QVERIFY(locator->undoRedoSystem() == nullptr);
    QVERIFY(locator->undoRedoSystemObj() == nullptr);

    // Act - Register undo/redo system
    locator->setUndoRedoSystem(undoRedoSystem.get());

    // Assert - Verify registration
    QVERIFY(locator->undoRedoSystem() != nullptr);
    QVERIFY(locator->undoRedoSystemObj() != nullptr);
    QCOMPARE(locator->undoRedoSystem(), undoRedoSystem.get());
    QCOMPARE(locator->undoRedoSystemObj(), undoRedoSystem.get());

    // Test functionality through ServiceLocator
    auto retrievedSystem = locator->undoRedoSystem();
    QVERIFY(!retrievedSystem.isNull());
    QVERIFY(retrievedSystem->manager() != nullptr);
    QVERIFY(retrievedSystem->queryHandler() != nullptr);

    // Test command execution through ServiceLocator
    int testValue = 0;
    auto command = std::make_shared<SCU::UndoRedoCommand>("ServiceLocator Test Command"_L1);
    command->setExecuteFunction([&testValue](auto &) { testValue = 42; });
    command->setRedoFunction([&testValue]() {
        testValue = 42;
        return SCU::Result<void>();
    });
    command->setUndoFunction([&testValue]() {
        testValue = 0;
        return SCU::Result<void>();
    });

    // Execute command through the system retrieved from ServiceLocator
    auto scope = SCU::UndoRedoScope::customScope("test"_L1);
    retrievedSystem->manager()->pushCommand(command, scope);

    // Verify command is registered but not executed yet
    QCOMPARE(testValue, 0);
    QVERIFY(retrievedSystem->manager()->canUndo(scope));
    QCOMPARE(retrievedSystem->manager()->undoCount(scope), 1);

    // Test query execution through ServiceLocator
    auto query = retrievedSystem->queryHandler()->createQuery<QString>("ServiceLocator Test Query"_L1);
    query->setQueryFunction([]() -> QString { return "ServiceLocator Works"_L1; });

    QSignalSpy queryFinishedSpy(retrievedSystem->queryHandler(), &SCU::QueryHandler::queryFinished);
    retrievedSystem->queryHandler()->executeQuery(query);

    QVERIFY(queryFinishedSpy.wait(1000));
    QCOMPARE(query->typedResult(), "ServiceLocator Works"_L1);

    // Cleanup - Clear the registration before objects are destroyed
    locator->setUndoRedoSystem(nullptr);
    QVERIFY(locator->undoRedoSystem() == nullptr);
}

QTEST_MAIN(TestUndoRedo)
#include "tst_undo_redo.moc"

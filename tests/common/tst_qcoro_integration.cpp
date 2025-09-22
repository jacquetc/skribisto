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

#include "undo_redo/undo_redo_command.h"
#include "undo_redo/undo_redo_system.h"
#include <QCoro/QCoroTask>
#include <QCoro/QCoroTest>
#include <QObject>
#include <QTest>

using namespace Qt::StringLiterals;
namespace SCU = Skribisto::Common::UndoRedo;

class TestQCoroIntegration : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();

    void testQCoroCommandExecution();
    void testQCoroQueryExecution();

  private:
    std::unique_ptr<SCU::UndoRedoSystem> m_system;
};

void TestQCoroIntegration::initTestCase()
{
    std::string duration("20000"); // 20 secondes
    QByteArray timeoutDuration(duration.c_str(), static_cast<int>(duration.length()));
    qputenv("QTEST_FUNCTION_TIMEOUT", timeoutDuration);
    m_system = std::make_unique<SCU::UndoRedoSystem>();
}

void TestQCoroIntegration::cleanupTestCase()
{
    m_system.reset();
}

void TestQCoroIntegration::testQCoroCommandExecution()
{
    auto testTask = [this]() -> QCoro::Task<void> {
        // Arrange
        int testValue = 0;
        auto command = std::make_shared<SCU::UndoRedoCommand>("QCoro Test Command"_L1);
        command->setExecuteFunction([&testValue](QPromise<SCU::Result<void>> &promise) {
            testValue = 42;
            promise.addResult(SCU::Result<void>());
        });
        command->setRedoFunction([&testValue]() {
            testValue = 42;
            return SCU::Result<void>();
        });
        command->setUndoFunction([&testValue]() {
            testValue = 0;
            return SCU::Result<void>();
        });

        // Act
        qDebug() << "Executing command asynchronously...";
        std::optional<bool> success = co_await m_system->executeCommandAsync(command, 1000, "test_scope"_L1);
        qDebug() << "Command execution completed.";

        // Assert
        QCORO_VERIFY(success.has_value());
        QCORO_VERIFY(success.value());
        QCORO_COMPARE(testValue, 42);
    };

    // Execute the coroutine test
    QCoro::waitFor(testTask());
}

void TestQCoroIntegration::testQCoroQueryExecution()
{
    auto testTask = [this]() -> QCoro::Task<void> {
        // Arrange
        auto query = m_system->createQuery<QString>("QCoro Test Query"_L1);
        query->setQueryFunction([]() -> QString { return "QCoro Works!"_L1; });

        // Act
        auto result = co_await m_system->executeQueryAsync(query);

        // Assert
        QCORO_COMPARE(result, "QCoro Works!"_L1);
    };

    // Execute the coroutine test
    QCoro::waitFor(testTask());
}

QTEST_MAIN(TestQCoroIntegration)
#include "tst_qcoro_integration.moc"
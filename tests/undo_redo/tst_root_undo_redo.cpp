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

#include "root/dtos.h"
#include "root/use_cases/create_uc.h"
#include "root/use_cases/i_root_unit_of_work.h"
#include "undo_redo/undo_redo_command.h"
#include <QObject>
#include <QSignalSpy>
#include <QTest>
#include <memory>

using namespace Qt::StringLiterals;

namespace SCU = Skribisto::Common::UndoRedo;
namespace SDAR = Skribisto::DirectAccess::Root;

// Mock unit of work for testing
class MockRootUnitOfWork : public SDAR::IRootUnitOfWork
{
  public:
    MockRootUnitOfWork() = default;

    void beginTransaction() override
    {
        m_inTransaction = true;
    }
    void commit() override
    {
        m_inTransaction = false;
    }
    void endTransaction() override
    {
        m_inTransaction = false;
    }
    void rollback() override
    {
        m_inTransaction = false;
    }

    void createSavepoint() override
    {
    }
    void rollbackToSavepoint() override
    {
    }
    void releaseSavepoint() override
    {
    }

    QList<Skribisto::Common::Entities::Root> createRoot(QList<Skribisto::Common::Entities::Root> roots) override
    {
        QList<Skribisto::Common::Entities::Root> result;
        for (auto &root : roots)
        {
            root.id = ++m_nextId;
            m_createdRoots.append(root);
            result.append(root);
        }
        return result;
    }

    QList<Skribisto::Common::Entities::Root> getRoot(QList<int> rootIds) override
    {
        QList<Skribisto::Common::Entities::Root> result;
        for (const auto &root : m_createdRoots)
        {
            if (rootIds.contains(root.id))
            {
                result.append(root);
            }
        }
        return result;
    }

    QList<Skribisto::Common::Entities::Root> updateRoot(QList<Skribisto::Common::Entities::Root> roots) override
    {
        return roots; // Simple mock implementation
    }

    QList<int> removeRoot(QList<int> rootIds) override
    {
        for (int id : rootIds)
        {
            m_createdRoots.removeIf([id](const auto &root) { return root.id == id; });
        }
        return rootIds;
    }

    QList<int> getRootRelationship(int rootId,
                                   Skribisto::Common::DirectAccess::Root::RootRelationshipField relationship) override
    {
        Q_UNUSED(rootId)
        Q_UNUSED(relationship)
        return {};
    }

    void setRootRelationship(int rootId, Skribisto::Common::DirectAccess::Root::RootRelationshipField relationship,
                             QList<int> relatedIds) override
    {
        Q_UNUSED(rootId)
        Q_UNUSED(relationship)
        Q_UNUSED(relatedIds)
    }

    QList<Skribisto::Common::Entities::Root> createdRoots() const
    {
        return m_createdRoots;
    }
    bool isInTransaction() const
    {
        return m_inTransaction;
    }
    QHash<int, QList<int>> getRootRelationshipMany(
        const QList<int> &rootIds, Skribisto::Common::DirectAccess::Root::RootRelationshipField relationship) override
    {
        Q_UNUSED(rootIds)
        Q_UNUSED(relationship)
        return {};
    }
    int getRootRelationshipCount(int rootId,
                                 Skribisto::Common::DirectAccess::Root::RootRelationshipField relationship) override
    {
        Q_UNUSED(rootId)
        Q_UNUSED(relationship)
        return 0;
    }
    QList<int> getRootRelationshipInRange(int rootId,
                                          Skribisto::Common::DirectAccess::Root::RootRelationshipField relationship,
                                          int offset, int limit) override
    {
        Q_UNUSED(rootId)
        Q_UNUSED(relationship)
        Q_UNUSED(offset)
        Q_UNUSED(limit)
        return {};
    }

  private:
    QList<Skribisto::Common::Entities::Root> m_createdRoots;
    int m_nextId = 0;
    bool m_inTransaction = false;
};

class TestRootUndoRedo : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void testCreateUseCaseExecuteUndoRedo();
    void testCreateUseCaseWithCommand();
    void testCreateUseCaseUndoWithoutExecute();

  private:
};

void TestRootUndoRedo::testCreateUseCaseExecuteUndoRedo()
{
    // Arrange
    auto mockUow = std::make_unique<MockRootUnitOfWork>();
    auto *uowPtr = mockUow.get();
    SDAR::CreateRootUseCase useCase(std::move(mockUow));

    QList<SDAR::CreateRootDto> createDtos;
    SDAR::CreateRootDto dto1;
    dto1.createdAt = QDateTime::currentDateTime();
    createDtos.append(dto1);

    // Act - Execute
    auto results = useCase.execute(createDtos);

    // Assert - Verify creation
    QCOMPARE(results.size(), 1);
    QVERIFY(results.first().id > 0);
    QCOMPARE(uowPtr->createdRoots().size(), 1);

    // Act - Undo
    auto undoResult = useCase.undo();

    // Assert - Verify undo
    QVERIFY(undoResult.isSuccess());
    QCOMPARE(uowPtr->createdRoots().size(), 0);

    // Act - Redo
    auto redoResult = useCase.redo();

    // Assert - Verify redo
    QVERIFY(redoResult.isSuccess());
    QCOMPARE(uowPtr->createdRoots().size(), 1);
}

void TestRootUndoRedo::testCreateUseCaseWithCommand()
{
    // Arrange
    auto mockUow = std::make_unique<MockRootUnitOfWork>();
    auto *uowPtr = mockUow.get();
    auto useCase = std::make_shared<SDAR::CreateRootUseCase>(std::move(mockUow));

    QList<SDAR::CreateRootDto> createDtos;
    SDAR::CreateRootDto dto1;
    dto1.createdAt = QDateTime::currentDateTime();
    createDtos.append(dto1);

    auto command = std::make_shared<SCU::UndoRedoCommand>("Test Create Command"_L1);
    QList<SDAR::RootDto> result;

    // Prepare lambdas
    command->setExecuteFunction([useCase, createDtos, &result](auto &) { result = useCase->execute(createDtos); });

    command->setRedoFunction([useCase, createDtos, &result]() { return useCase->redo(); });

    command->setUndoFunction([useCase]() -> SCU::Result<void> { return useCase->undo(); });

    QSignalSpy finishedSpy(command.get(), &SCU::UndoRedoCommand::finished);

    // Act - Execute command (redo)
    command->asyncExecute();
    QVERIFY(finishedSpy.wait(1000));

    // Assert - Verify execution
    QCOMPARE(result.size(), 1);
    QVERIFY(result.first().id > 0);
    QCOMPARE(uowPtr->createdRoots().size(), 1);

    // Act - Undo command
    finishedSpy.clear();
    command->asyncUndo();
    QVERIFY(finishedSpy.wait(1000));

    // Assert - Verify undo
    QCOMPARE(uowPtr->createdRoots().size(), 0);
}

void TestRootUndoRedo::testCreateUseCaseUndoWithoutExecute()
{
    // Arrange
    auto mockUow = std::make_unique<MockRootUnitOfWork>();
    SDAR::CreateRootUseCase useCase(std::move(mockUow));

    // Act - Try to undo without execute
    auto undoResult = useCase.undo();

    // Assert - Should fail
    QVERIFY(!undoResult.isSuccess());
    QVERIFY(undoResult.error().contains("Cannot undo"_L1));
}

QTEST_MAIN(TestRootUndoRedo)
#include "tst_root_undo_redo.moc"

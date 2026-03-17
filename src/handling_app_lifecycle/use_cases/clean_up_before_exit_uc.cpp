// Adapted for CleanUpBeforeExit use case

#include "clean_up_before_exit_uc.h"

namespace Skribisto::HandlingAppLifecycle
{

CleanUpBeforeExitUseCase::CleanUpBeforeExitUseCase(std::unique_ptr<ICleanUpBeforeExitUnitOfWork> uow)
    : m_uow(std::move(uow))
{
}

bool CleanUpBeforeExitUseCase::execute() const
{
    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Remove all Root entities (cascade deletes System, Works, and all children)
        auto roots = m_uow->getAllRoot();
        if (!roots.isEmpty())
        {
            QList<int> rootIds;
            for (const auto &root : roots)
                rootIds.append(root.id);
            m_uow->removeRoot(rootIds);
        }

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishCleanUpBeforeExitSignal();
    return true;
}

} // namespace Skribisto::HandlingAppLifecycle

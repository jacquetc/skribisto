#pragma once

#include "contracts_global.h"
#include "result.h"
#include <QEventLoop>
#include <QFuture>
#include <QFutureWatcher>
#include <QReadWriteLock>

/**
 * @brief A utility class that waits for a Qt future to complete in a Qt event loop.
 */
class SKR_CONTRACTS_EXPORT WaitInEventLoop
{

  public:
    /**
     * @brief Waits for a Qt future to complete in a Qt event loop and returns the result.
     * @tparam U The type of the result value.
     * @param future The Qt future to wait for.
     * @return A Result object containing the result value or an error message if the operation fails.
     */
    template <class U> auto waitInEventLoop(QFuture<Result<U>> &&future)
    {
        QEventLoop wait;
        QFutureWatcher<Result<U>> fw;

        QObject::connect(&fw, &QFutureWatcher<Result<U>>::finished, &wait, &QEventLoop::quit);

        // Connect progressRangeChanged signal to a lambda function that emits a signal with progress info
        QObject::connect(&fw, &QFutureWatcher<Result<U>>::progressRangeChanged, [this](int minimum, int maximum) {
            // emit a signal with progress info
            emit progressChanged(minimum, maximum, 0);
        });

        // Connect progressValueChanged signal to a lambda function that emits a signal with progress info
        QObject::connect(&fw, &QFutureWatcher<Result<U>>::progressValueChanged, [this](int progressValue) {
            // emit a signal with progress info
            emit progressChanged(0, 0, progressValue);
        });

        fw.setFuture(future);
        wait.exec();

        // Disconnect the signals after the event loop has finished
        QObject::disconnect(&fw, &QFutureWatcher<Result<U>>::finished, &wait, &QEventLoop::quit);
        QObject::disconnect(&fw, &QFutureWatcher<Result<U>>::progressRangeChanged, nullptr, nullptr);
        QObject::disconnect(&fw, &QFutureWatcher<Result<U>>::progressValueChanged, nullptr, nullptr);

        return fw.result();
    }

    /**
     * @brief Waits for a Qt future to complete in a Qt event loop and returns the result.
     * @tparam U The type of the result value.
     * @param future The Qt future to wait for.
     * @return A Result object containing the result value or an error message if the operation fails.
     */
    template <class U> static auto basicWaitInEventLoop(QFuture<Result<U>> &&future)
    {
        QEventLoop wait;
        QFutureWatcher<Result<U>> fw;

        QObject::connect(&fw, &QFutureWatcher<Result<U>>::finished, &wait, &QEventLoop::quit);

        fw.setFuture(future);
        wait.exec();

        // Disconnect the signals after the event loop has finished
        QObject::disconnect(&fw, &QFutureWatcher<Result<U>>::finished, &wait, &QEventLoop::quit);

        return fw.result();
    }

  signals:
    /**
     * @brief A signal that is emitted when the progress of a task changes.
     * @param minimum The minimum progress value.
     * @param maximum The maximum progress value.
     * @param value The current progress value.
     */
    void progressChanged(int minimum, int maximum, int value);
};

#pragma once

#include "application_global.h"
#include "result.h"
#include <QFuture>
#include <QFutureWatcher>
#include <QObject>
#include <QReadWriteLock>

namespace Application
{

class SKR_APPLICATION_EXPORT Handler : public QObject
{
    Q_OBJECT
  public:
    explicit Handler(QObject *parent = nullptr);

  protected:
    //    template <class U> void runAsync(QFuture<Result<U>> &&future)
    //    {

    //        QFutureWatcher<Result<U>> fw(this);

    //        // Connect progressRangeChanged signal to a lambda function that emits a signal with progress info
    //        QObject::connect(&fw, &QFutureWatcher<Result<U>>::progressRangeChanged, [this](int minimum, int maximum) {
    //            // emit a signal with progress info
    //            emit progressRangeChanged(minimum, maximum);
    //        });

    //        // Connect progressValueChanged signal to a lambda function that emits a signal with progress info
    //        QObject::connect(&fw, &QFutureWatcher<Result<U>>::progressValueChanged, [this](int progressValue) {
    //            // emit a signal with progress info
    //            emit progressValueChanged(progressValue);
    //        });

    //        fw.setFuture(future);
    //    }
  signals:
    //    void progressRangeChanged(int minimum, int maximum);
    //    void progressValueChanged(int progress);

  private:
};
} // namespace Application

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
    // virtual Result<void> handle(QPromise<Result<void>> &progressPromise, ) = 0;
    // virtual Result<void> restore() = 0;

  protected:
  signals:

  private:
};
} // namespace Application

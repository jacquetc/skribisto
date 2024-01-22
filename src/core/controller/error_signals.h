#pragma once

#include "controller_export.h"
#include <QObject>
#include <qleany/common/error.h>

namespace Skribisto::Controller
{

class SKRIBISTO_CONTROLLER_EXPORT ErrorSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ErrorSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void warningSent(const Qleany::Error &error);
    void errorSent(const Qleany::Error &error);
};
} // namespace Skribisto::Controller
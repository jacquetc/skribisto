#pragma once

#include <QObject>

namespace Presenter::Entities
{

class SceneSignals : public QObject
{
    Q_OBJECT
  public:
    explicit SceneSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void scenesDeleted(QList<int> entityIds);
    // Soft delete:
    void scenesActiveStateChanged(QList<int> entityIds, bool active);
};

} // namespace Presenter::Entities

#pragma once

#include <QObject>

namespace Presenter::Entities
{

class SceneParagraphSignals : public QObject
{
    Q_OBJECT
  public:
    explicit SceneParagraphSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void sceneParagraphsDeleted(QList<int> entityIds);
    // Soft delete:
    void sceneParagraphsActiveStateChanged(QList<int> entityIds, bool active);
};

} // namespace Presenter::Entities

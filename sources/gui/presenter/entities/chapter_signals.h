#pragma once

#include <QObject>

namespace Presenter::Entities
{

class ChapterSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ChapterSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void chaptersDeleted(QList<int> entityIds);
    // Soft delete:
    void chaptersActiveStateChanged(QList<int> entityIds, bool active);
};

} // namespace Presenter::Entities

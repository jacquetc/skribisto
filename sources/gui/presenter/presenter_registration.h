#pragma once
#include "presenter_global.h"
#include "undo_redo/threaded_undo_redo_system.h"

#include <QObject>

namespace Presenter
{

class SKR_PRESENTER_EXPORT PresenterRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit PresenterRegistration(QObject *parent);

  signals:

  private:
    QScopedPointer<UndoRedo::ThreadedUndoRedoSystem> s_undoRedoSystem;
};
} // namespace Presenter

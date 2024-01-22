// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"
#include <QObject>
#include <qleany/tools/undo_redo/undo_redo_scopes.h>

namespace Skribisto::Controller
{

class SKRIBISTO_CONTROLLER_EXPORT UndoRedoSignals : public QObject
{
    Q_OBJECT
  public:
    explicit UndoRedoSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:

    /*!
     * \brief A signal that is emitted when the undo redo system state has
     *changed. Useful for the undo and redo
     * actions.
     */
    void stateChanged();

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start redoing.
     * actions.
     */
    void redoing(Qleany::Tools::UndoRedo::Scope scope, bool active);

    /*!
     * \brief A signal that is emitted when the undo redo system is about to
     *start undoing.
     * actions.
     */
    void undoing(Qleany::Tools::UndoRedo::Scope scope, bool active);
};
} // namespace Skribisto::Controller
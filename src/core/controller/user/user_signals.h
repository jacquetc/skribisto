// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "user/user_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::User;

class SKRIBISTO_CONTROLLER_EXPORT UserSignals : public QObject
{
    Q_OBJECT
  public:
    explicit UserSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(UserDTO dto);
    void updated(UserDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(UserDTO dto);
};
} // namespace Skribisto::Controller
#pragma once
#include "../../cxxqt/presenter/cxx-qt-gen/structure_management_controller.cxxqt.h"
#include <QQmlEngine>

struct StructureManagementController
{
    Q_GADGET
    QML_FOREIGN(presenter::structure_management::StructureManagementController)
    QML_SINGLETON
    QML_NAMED_ELEMENT(StructureManagementController)

  public:
  private:
};

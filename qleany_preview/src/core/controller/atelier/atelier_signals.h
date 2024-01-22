// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "atelier/atelier_with_details_dto.h"

#include "atelier/atelier_dto.h"

#include "atelier/atelier_relation_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::Atelier;

class SKRIBISTO_CONTROLLER_EXPORT AtelierSignals : public QObject
{
    Q_OBJECT
  public:
    explicit AtelierSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void getReplied(AtelierDTO dto);
    void getWithDetailsReplied(AtelierWithDetailsDTO dto);

    void relationInserted(AtelierRelationDTO dto);
    void relationRemoved(AtelierRelationDTO dto);
};
} // namespace Skribisto::Controller
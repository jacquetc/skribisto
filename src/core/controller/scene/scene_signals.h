// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "scene/scene_with_details_dto.h"

#include "scene/scene_dto.h"

#include "scene/scene_relation_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::Scene;

class SKRIBISTO_CONTROLLER_EXPORT SceneSignals : public QObject
{
    Q_OBJECT
  public:
    explicit SceneSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(SceneDTO dto);
    void updated(SceneDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(SceneDTO dto);
    void getWithDetailsReplied(SceneWithDetailsDTO dto);

    void relationInserted(SceneRelationDTO dto);
    void relationRemoved(SceneRelationDTO dto);
};
} // namespace Skribisto::Controller
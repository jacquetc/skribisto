// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "scene_paragraph/scene_paragraph_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::SceneParagraph;

class SKRIBISTO_CONTROLLER_EXPORT SceneParagraphSignals : public QObject
{
    Q_OBJECT
  public:
    explicit SceneParagraphSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(SceneParagraphDTO dto);
    void updated(SceneParagraphDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(SceneParagraphDTO dto);
};
} // namespace Skribisto::Controller
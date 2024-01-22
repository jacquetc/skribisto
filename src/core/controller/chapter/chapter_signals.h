// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "chapter/chapter_with_details_dto.h"

#include "chapter/chapter_dto.h"

#include "chapter/chapter_relation_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::Chapter;

class SKRIBISTO_CONTROLLER_EXPORT ChapterSignals : public QObject
{
    Q_OBJECT
  public:
    explicit ChapterSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(ChapterDTO dto);
    void updated(ChapterDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(ChapterDTO dto);
    void getWithDetailsReplied(ChapterWithDetailsDTO dto);

    void relationInserted(ChapterRelationDTO dto);
    void relationRemoved(ChapterRelationDTO dto);
};
} // namespace Skribisto::Controller
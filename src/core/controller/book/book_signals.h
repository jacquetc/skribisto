// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "book/book_with_details_dto.h"

#include "book/book_dto.h"

#include "book/book_relation_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::Book;

class SKRIBISTO_CONTROLLER_EXPORT BookSignals : public QObject
{
    Q_OBJECT
  public:
    explicit BookSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void removed(QList<int> removedIds);
    void activeStatusChanged(QList<int> changedIds, bool isActive);
    void created(BookDTO dto);
    void updated(BookDTO dto);
    void allRelationsInvalidated(int id);
    void getReplied(BookDTO dto);
    void getWithDetailsReplied(BookWithDetailsDTO dto);

    void relationInserted(BookRelationDTO dto);
    void relationRemoved(BookRelationDTO dto);
};
} // namespace Skribisto::Controller
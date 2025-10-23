/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "work/work_controller.h"
#include <QCoro/QCoroQml>
#include <QCoro/QCoroQmlTask>
#include <QQmlEngine>

struct ForeignWorkController : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(WorkController)

  public:
    explicit ForeignWorkController(QObject *parent = nullptr)
        : QObject(parent), m_controller(new Skribisto::DirectAccess::Work::WorkController(this))

    {
    }
    Q_INVOKABLE QCoro::QmlTask get(const QList<int> &ids)
    {
        return m_controller->get(ids);
    }

    Q_INVOKABLE static Skribisto::DirectAccess::Work::CreateWorkDto getCreateDto()
    {
        return Skribisto::DirectAccess::Work::WorkController::getCreateDto();
    }

    Q_INVOKABLE QCoro::QmlTask create(const QList<Skribisto::DirectAccess::Work::CreateWorkDto> &dtos)
    {
        return m_controller->create(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask update(const QList<Skribisto::DirectAccess::Work::WorkDto> &dtos)
    {
        return m_controller->update(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask remove(const QList<int> &ids)
    {
        return m_controller->remove(ids);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIds(int workId,
                                                  Skribisto::DirectAccess::Work::WorkRelationshipField relationship)
    {
        return m_controller->getRelationshipIds(workId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask setRelationshipIds(int workId,
                                                  Skribisto::DirectAccess::Work::WorkRelationshipField relationship,
                                                  const QList<int> &relatedIds)
    {
        return m_controller->setRelationshipIds(workId, relationship, relatedIds);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsCount(
        int workId, Skribisto::DirectAccess::Work::WorkRelationshipField relationship)
    {
        return m_controller->getRelationshipIdsCount(workId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsInRange(
        int workId, Skribisto::DirectAccess::Work::WorkRelationshipField relationship, int offset, int limit)
    {
        return m_controller->getRelationshipIdsInRange(workId, relationship, offset, limit);
    }

  private:
    Skribisto::DirectAccess::Work::WorkController *m_controller;
};
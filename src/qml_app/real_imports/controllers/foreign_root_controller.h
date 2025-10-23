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
#include "root/root_controller.h"
#include <QCoro/QCoroQml>
#include <QCoro/QCoroQmlTask>
#include <QQmlEngine>

struct ForeignRootController : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(RootController)

  public:
    explicit ForeignRootController(QObject *parent = nullptr)
        : QObject(parent), m_controller(new Skribisto::DirectAccess::Root::RootController(this))

    {
    }
    Q_INVOKABLE QCoro::QmlTask get(const QList<int> &ids)
    {
        return m_controller->get(ids);
    }

    Q_INVOKABLE static Skribisto::DirectAccess::Root::CreateRootDto getCreateDto()
    {
        return Skribisto::DirectAccess::Root::RootController::getCreateDto();
    }

    Q_INVOKABLE QCoro::QmlTask create(const QList<Skribisto::DirectAccess::Root::CreateRootDto> &dtos)
    {
        return m_controller->create(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask update(const QList<Skribisto::DirectAccess::Root::RootDto> &dtos)
    {
        return m_controller->update(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask remove(const QList<int> &ids)
    {
        return m_controller->remove(ids);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIds(int rootId,
                                                  Skribisto::DirectAccess::Root::RootRelationshipField relationship)
    {
        return m_controller->getRelationshipIds(rootId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask setRelationshipIds(int rootId,
                                                  Skribisto::DirectAccess::Root::RootRelationshipField relationship,
                                                  const QList<int> &relatedIds)
    {
        return m_controller->setRelationshipIds(rootId, relationship, relatedIds);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsCount(
        int rootId, Skribisto::DirectAccess::Root::RootRelationshipField relationship)
    {
        return m_controller->getRelationshipIdsCount(rootId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsInRange(
        int rootId, Skribisto::DirectAccess::Root::RootRelationshipField relationship, int offset, int limit)
    {
        return m_controller->getRelationshipIdsInRange(rootId, relationship, offset, limit);
    }

  private:
    Skribisto::DirectAccess::Root::RootController *m_controller;
};
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
#include "binder_item/binder_item_controller.h"
#include <QCoro/QCoroQml>
#include <QCoro/QCoroQmlTask>
#include <QQmlEngine>

struct ForeignBinderItemController : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(BinderItemController)

  public:
    explicit ForeignBinderItemController(QObject *parent = nullptr)
        : QObject(parent), m_controller(new Skribisto::DirectAccess::BinderItem::BinderItemController(this))

    {
    }
    Q_INVOKABLE QCoro::QmlTask get(const QList<int> &ids)
    {
        return m_controller->get(ids);
    }

    Q_INVOKABLE static Skribisto::DirectAccess::BinderItem::CreateBinderItemDto getCreateDto()
    {
        return Skribisto::DirectAccess::BinderItem::BinderItemController::getCreateDto();
    }

    Q_INVOKABLE QCoro::QmlTask create(const QList<Skribisto::DirectAccess::BinderItem::CreateBinderItemDto> &dtos)
    {
        return m_controller->create(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask update(const QList<Skribisto::DirectAccess::BinderItem::BinderItemDto> &dtos)
    {
        return m_controller->update(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask remove(const QList<int> &ids)
    {
        return m_controller->remove(ids);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIds(
        int binderItemId, Skribisto::DirectAccess::BinderItem::BinderItemRelationshipField relationship)
    {
        return m_controller->getRelationshipIds(binderItemId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask setRelationshipIds(
        int binderItemId, Skribisto::DirectAccess::BinderItem::BinderItemRelationshipField relationship,
        const QList<int> &relatedIds)
    {
        return m_controller->setRelationshipIds(binderItemId, relationship, relatedIds);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsCount(
        int binderItemId, Skribisto::DirectAccess::BinderItem::BinderItemRelationshipField relationship)
    {
        return m_controller->getRelationshipIdsCount(binderItemId, relationship);
    }

    Q_INVOKABLE QCoro::QmlTask getRelationshipIdsInRange(
        int binderItemId, Skribisto::DirectAccess::BinderItem::BinderItemRelationshipField relationship, int offset,
        int limit)
    {
        return m_controller->getRelationshipIdsInRange(binderItemId, relationship, offset, limit);
    }

  private:
    Skribisto::DirectAccess::BinderItem::BinderItemController *m_controller;
};
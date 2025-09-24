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
#include "binder_tag/binder_tag_controller.h"
#include <QCoro/QCoroQml>
#include <QCoro/QCoroQmlTask>
#include <QQmlEngine>

struct ForeignBinderTagController : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(BinderTagController)

  public:
    explicit ForeignBinderTagController(QObject *parent = nullptr)
        : QObject(parent), m_controller(new Skribisto::DirectAccess::BinderTag::BinderTagController(this))

    {
    }
    Q_INVOKABLE QCoro::QmlTask get(const QList<int> &ids)
    {
        return m_controller->get(ids);
    }

    Q_INVOKABLE static Skribisto::DirectAccess::BinderTag::CreateBinderTagDto getCreateDto()
    {
        return Skribisto::DirectAccess::BinderTag::BinderTagController::getCreateDto();
    }

    Q_INVOKABLE QCoro::QmlTask create(const QList<Skribisto::DirectAccess::BinderTag::CreateBinderTagDto> &dto)
    {
        return m_controller->create(dto);
    }

    Q_INVOKABLE QCoro::QmlTask update(const QList<Skribisto::DirectAccess::BinderTag::BinderTagDto> &dtos)
    {
        return m_controller->update(dtos);
    }

    Q_INVOKABLE QCoro::QmlTask remove(const QList<int> &ids)
    {
        return m_controller->remove(ids);
    }

  private:
    Skribisto::DirectAccess::BinderTag::BinderTagController *m_controller;
};
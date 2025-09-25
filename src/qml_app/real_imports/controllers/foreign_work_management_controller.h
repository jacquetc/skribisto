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
#include "work_management_controller.h"
#include "work_management_dtos.h"
#include <QCoro/QCoroQml>
#include <QCoro/QCoroQmlTask>
#include <QQmlEngine>

namespace Skribisto::WorkManagement
{
struct ForeignWorkManagementController : public QObject
{
    Q_OBJECT
    QML_NAMED_ELEMENT(WorkManagementController)

  public:
    explicit ForeignWorkManagementController(QObject *parent = nullptr)
        : QObject(parent), m_controller(new Skribisto::WorkManagement::WorkManagementController(this))

    {
    }
    Q_INVOKABLE QCoro::QmlTask loadWork(const Skribisto::WorkManagement::LoadWorkDto &loadWorkDto)
    {
        return m_controller->loadWork(loadWorkDto);
    }

    Q_INVOKABLE static Skribisto::WorkManagement::LoadWorkDto getLoadWorkDto()
    {
        return Skribisto::WorkManagement::WorkManagementController::getLoadWorkDto();
    }
    Q_INVOKABLE QCoro::QmlTask saveWork(const Skribisto::WorkManagement::SaveWorkDto &saveWorkDto)
    {
        return m_controller->saveWork(saveWorkDto);
    }

    Q_INVOKABLE static Skribisto::WorkManagement::SaveWorkDto getSaveWorkDto()
    {
        return Skribisto::WorkManagement::WorkManagementController::getSaveWorkDto();
    }

  private:
    Skribisto::WorkManagement::WorkManagementController *m_controller;
};
} // namespace Skribisto::WorkManagement
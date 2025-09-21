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

#pragma once
#include "service_locator.h"

#include <QObject>
#include <QQmlEngine>

class ForeignServiceLocator
{
    Q_GADGET
    QML_FOREIGN(Skribisto::Common::ServiceLocator)
    QML_NAMED_ELEMENT(ServiceLocator) // exposed to QML as “ServiceLocator”
                                      // public:
                                      //   explicit ForeignServiceLocator(QObject *parent = nullptr) : QObject(parent)
                                      //   {
                                      //   }

    // Q_INVOKABLE QObject *dbContext() const
    // {
    //     auto *core = Skribisto::Common::ServiceLocator::instance();
    //     return core ? reinterpret_cast<QObject *>(core->dbContext()) : nullptr;
    // }
    // Q_INVOKABLE QObject *eventRegistry() const
    // {
    //     auto *core = Skribisto::Common::ServiceLocator::instance();
    //     return core ? reinterpret_cast<QObject *>(core->eventRegistry()) : nullptr;
    // }
};
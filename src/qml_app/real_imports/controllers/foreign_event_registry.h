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

#include "direct_access/event_registry.h"
#include "service_locator.h"
#include <QQmlEngine>

struct ForeignEventDispatcher
{
    Q_GADGET
    QML_FOREIGN(Skribisto::Common::DirectAccess::EventRegistry)
    QML_SINGLETON
    QML_NAMED_ELEMENT(EventRegistry)

  public:
    // Initialize this singleton instance with the given engine.

    inline static Skribisto::Common::DirectAccess::EventRegistry *s_singletonInstance = nullptr;

    static Skribisto::Common::DirectAccess::EventRegistry *create(QQmlEngine *, QJSEngine *engine)
    {
        s_singletonInstance = Skribisto::Common::ServiceLocator::instance()->eventRegistry();
        ;

        // The instance has to exist before it is used. We cannot replace it.
        Q_ASSERT(s_singletonInstance);

        // The engine has to have the same thread affinity as the singleton.
        Q_ASSERT(engine->thread() == s_singletonInstance->thread());

        // There can only be one engine accessing the singleton.
        if (s_engine)
            Q_ASSERT(engine == s_engine);
        else
            s_engine = engine;

        // Explicitly specify C++ ownership so that the engine doesn't delete
        // the instance.
        QJSEngine::setObjectOwnership(s_singletonInstance, QJSEngine::CppOwnership);

        return s_singletonInstance;
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::Root::RootEvents *getRootEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::Root::RootEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::Project::ProjectEvents *getProjectEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::Project::ProjectEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::Binder::BinderEvents *getBinderEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::Binder::BinderEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::BinderItem::BinderItemEvents *getBinderItemEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::BinderItem::BinderItemEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::RecentProject::RecentProjectEvents *getRecentProjectEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::RecentProject::RecentProjectEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::Content::ContentEvents *getContentEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::Content::ContentEvents>();
    }

    Q_INVOKABLE Skribisto::Common::DirectAccess::BinderTag::BinderTagEvents *getBinderTagEvents() const
    {
        return s_singletonInstance->getEvents<Skribisto::Common::DirectAccess::BinderTag::BinderTagEvents>();
    }

  private:
    inline static QJSEngine *s_engine = nullptr;
};
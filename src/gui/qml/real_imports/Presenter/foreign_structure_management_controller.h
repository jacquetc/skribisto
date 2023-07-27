#pragma once
#include "presenter_controller.h"
#include <QQmlEngine>

struct StructureManagementController
{
    Q_GADGET
    QML_FOREIGN(presenter::structure_management::StructureManagementController)
    QML_SINGLETON
    QML_NAMED_ELEMENT(StructureManagementController)

  public:
    // Initialize this singleton instance with the given engine.

    inline static Presenter::Author::AuthorController *s_singletonInstance = nullptr;

    static Presenter::Author::AuthorController *create(QQmlEngine *, QJSEngine *engine)
    {
        s_singletonInstance = Presenter::Author::AuthorController::instance();

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

  private:
    inline static QJSEngine *s_engine = nullptr;
};

#pragma once
#include "scene/scene_controller.h"
#include <QQmlEngine>

struct ForeignSceneController
{
    Q_GADGET
    QML_FOREIGN(Presenter::Scene::SceneController)
    QML_SINGLETON
    QML_NAMED_ELEMENT(SceneController)

public:

    // Initialize this singleton instance with the given engine.

    inline static Presenter::Scene::SceneController *s_singletonInstance = nullptr;

    static Presenter::Scene::SceneController* create(QQmlEngine *, QJSEngine *engine)
    {
        s_singletonInstance = Presenter::Scene::SceneController::instance();

        // The instance has to exist before it is used. We cannot replace it.
        Q_ASSERT(s_singletonInstance);

        // The engine has to have the same thread affinity as the singleton.
        Q_ASSERT(engine->thread() == s_singletonInstance->thread());

        // There can only be one engine accessing the singleton.
        if (s_engine) Q_ASSERT(engine == s_engine);
        else s_engine = engine;

        // Explicitly specify C++ ownership so that the engine doesn't delete
        // the instance.
        QJSEngine::setObjectOwnership(s_singletonInstance, QJSEngine::CppOwnership);

        return s_singletonInstance;
    }

private:

    inline static QJSEngine *s_engine = nullptr;
};
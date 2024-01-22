// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "scene_paragraph/scene_paragraph_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::SceneParagraph;

class ForeignSceneParagraphController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(SceneParagraphController)

  public:
    ForeignSceneParagraphController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = SceneParagraphController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE CreateSceneParagraphDTO getCreateDTO()
    {
        return s_controllerInstance->getCreateDTO();
    }

    Q_INVOKABLE UpdateSceneParagraphDTO getUpdateDTO()
    {
        return s_controllerInstance->getUpdateDTO();
    }

    Q_INVOKABLE QCoro::QmlTask create(const CreateSceneParagraphDTO &dto)
    {
        return s_controllerInstance->create(dto);
    }

    Q_INVOKABLE QCoro::QmlTask update(const UpdateSceneParagraphDTO &dto)
    {
        return s_controllerInstance->update(dto);
    }

    Q_INVOKABLE QCoro::QmlTask remove(int id)
    {
        return s_controllerInstance->remove(id);
    }

  private:
    SceneParagraphController *s_controllerInstance = nullptr;
};
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "system/system_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::System;

class ForeignSystemController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(SystemController)

  public:
    ForeignSystemController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = SystemController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask getCurrentTime() const
    {
        return s_controllerInstance->getCurrentTime();
    }

    Q_INVOKABLE QCoro::QmlTask createNewAtelier(CreateNewAtelierDTO dto)
    {
        return s_controllerInstance->createNewAtelier(dto);
    }

    Q_INVOKABLE QCoro::QmlTask loadAtelier(LoadAtelierDTO dto)
    {
        return s_controllerInstance->loadAtelier(dto);
    }

    Q_INVOKABLE QCoro::QmlTask saveAtelier()
    {
        return s_controllerInstance->saveAtelier();
    }

    Q_INVOKABLE QCoro::QmlTask closeAtelier()
    {
        return s_controllerInstance->closeAtelier();
    }

  private:
    SystemController *s_controllerInstance = nullptr;
};
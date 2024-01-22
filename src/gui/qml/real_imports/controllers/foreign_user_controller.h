// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "user/user_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::User;

class ForeignUserController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(UserController)

  public:
    ForeignUserController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = UserController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE UpdateUserDTO getUpdateDTO()
    {
        return s_controllerInstance->getUpdateDTO();
    }

    Q_INVOKABLE QCoro::QmlTask update(const UpdateUserDTO &dto)
    {
        return s_controllerInstance->update(dto);
    }

  private:
    UserController *s_controllerInstance = nullptr;
};
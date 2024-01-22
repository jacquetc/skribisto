// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "atelier/atelier_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::Atelier;

class ForeignAtelierController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(AtelierController)

  public:
    ForeignAtelierController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = AtelierController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask getWithDetails(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask getAll() const
    {
        return s_controllerInstance->getAll();
    }

  private:
    AtelierController *s_controllerInstance = nullptr;
};
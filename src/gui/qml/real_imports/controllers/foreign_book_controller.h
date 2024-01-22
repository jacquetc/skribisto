// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "book/book_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::Book;

class ForeignBookController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(BookController)

  public:
    ForeignBookController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = BookController::instance();
    }

    Q_INVOKABLE QCoro::QmlTask get(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE QCoro::QmlTask getWithDetails(int id) const
    {
        return s_controllerInstance->get(id);
    }

    Q_INVOKABLE UpdateBookDTO getUpdateDTO()
    {
        return s_controllerInstance->getUpdateDTO();
    }

    Q_INVOKABLE QCoro::QmlTask update(const UpdateBookDTO &dto)
    {
        return s_controllerInstance->update(dto);
    }

  private:
    BookController *s_controllerInstance = nullptr;
};
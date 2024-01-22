// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "file/file_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::File;

class ForeignFileController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(FileController)

  public:
    ForeignFileController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = FileController::instance();
    }

  private:
    FileController *s_controllerInstance = nullptr;
};
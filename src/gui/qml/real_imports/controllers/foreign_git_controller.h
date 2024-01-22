// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once
#include "git/git_controller.h"
#include <QCoroQml>
#include <QCoroQmlTask>
#include <QQmlEngine>

using namespace Skribisto::Controller::Git;

class ForeignGitController : public QObject
{
    Q_OBJECT
    QML_SINGLETON
    QML_NAMED_ELEMENT(GitController)

  public:
    ForeignGitController(QObject *parent = nullptr) : QObject(parent)
    {
        s_controllerInstance = GitController::instance();
    }

  private:
    GitController *s_controllerInstance = nullptr;
};
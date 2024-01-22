// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.

#pragma once

#include "controller_export.h"
#include <QObject>
#include <qleany/contracts/repository/interface_repository_provider.h>

namespace Skribisto::Controller
{

class SKRIBISTO_CONTROLLER_EXPORT ControllerRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit ControllerRegistration(QObject *parent,
                                    Qleany::Contracts::Repository::InterfaceRepositoryProvider *repositoryProvider);
    ~ControllerRegistration();

  private:
};

} // namespace Skribisto::Controller
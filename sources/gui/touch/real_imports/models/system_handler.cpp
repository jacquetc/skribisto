#include "system_handler.h"
#include "jsdto_mapper.h"
#include "load_system_dto.h"
#include "save_system_as_dto.h"
#include "system/system_controller.h"

using namespace Contracts::DTO::System;
using namespace Presenter::System;

SystemHandler::SystemHandler()
{
    connect(SystemController::instance(), &SystemController::loadSystemProgressFinished, this,
            &SystemHandler::loadSystemProgressFinished);
    connect(SystemController::instance(), &SystemController::loadSystemProgressRangeChanged, this,
            &SystemHandler::loadSystemProgressRangeChanged);
    connect(SystemController::instance(), &SystemController::loadSystemProgressTextChanged, this,
            &SystemHandler::loadSystemProgressTextChanged);
    connect(SystemController::instance(), &SystemController::loadSystemProgressValueChanged, this,
            &SystemHandler::loadSystemProgressValueChanged);
    connect(SystemController::instance(), &SystemController::systemLoaded, this, &SystemHandler::systemLoaded);
    connect(SystemController::instance(), &SystemController::systemSaved, this, &SystemHandler::systemSaved);
    connect(SystemController::instance(), &SystemController::systemClosed, this, &SystemHandler::systemClosed);
}

void SystemHandler::loadSystem(const QJSValue &jsDto)
{

    LoadSystemDTO cppDto = mapToDto<LoadSystemDTO>(jsDto);
    Presenter::System::SystemController::instance()->loadSystem(cppDto);
}

void SystemHandler::saveSystem()
{
}

void SystemHandler::saveSystemAs(const QJSValue &jsDto)
{

    SaveSystemAsDTO cppDto = mapToDto<SaveSystemAsDTO>(jsDto);
    Presenter::System::SystemController::instance()->saveSystemAs(cppDto);
}

void SystemHandler::closeSystem()
{
}

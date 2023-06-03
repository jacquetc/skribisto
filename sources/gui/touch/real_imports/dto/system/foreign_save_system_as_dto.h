#pragma once
#include "save_system_as_dto.h"
#include <QQmlEngine>

struct SaveSystemAsDTO
{
    Q_GADGET
    QML_FOREIGN(Contracts::DTO::System::SaveSystemAsDTO)
    QML_ELEMENT
};

#pragma once

#include "QtCore/qobjectdefs.h"
#include "QtCore/qtmetamacros.h"

namespace Domain::Entities
{
Q_NAMESPACE

enum EntityEnum
{
    Entity,
    Author,
    Book,
    Chapter,
    Scene,
    SceneParagraph,
    Atelier
};
Q_ENUM_NS(EntityEnum);

} // namespace Domain::Entities

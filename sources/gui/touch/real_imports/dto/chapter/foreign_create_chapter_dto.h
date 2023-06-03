#pragma once
#include "create_chapter_dto.h"
#include <QQmlEngine>

struct CreateChapterDTO
{
    Q_GADGET
    QML_FOREIGN(Contracts::DTO::Chapter::CreateChapterDTO)
    QML_ELEMENT
};

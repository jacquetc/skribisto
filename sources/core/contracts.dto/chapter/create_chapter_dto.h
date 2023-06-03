#pragma once

#include "QtCore/qobject.h"
#include "chapter_dto_base.h"
#include <QDateTime>
#include <QObject>

namespace Contracts::DTO::Chapter
{

//-------------------------------------------------

class CreateChapterDTO : public ChapterDTOBase
{
    Q_OBJECT
  public:
    CreateChapterDTO(QObject *parent = nullptr) : ChapterDTOBase(parent)
    {
    }
    CreateChapterDTO(const QString &title) : ChapterDTOBase(title)
    {
    }

    CreateChapterDTO(const CreateChapterDTO &other) : ChapterDTOBase(other)
    {
    }
    CreateChapterDTO &operator=(const CreateChapterDTO &other)
    {
        if (this != &other)
        {
            ChapterDTOBase::operator=(other);
        }
        return *this;
    }

  private:
};

} // namespace Contracts::DTO::Chapter

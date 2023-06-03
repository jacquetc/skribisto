#pragma once

#include "chapter_dto_base.h"
#include <QUuid>

namespace Contracts::DTO::Chapter
{

class UpdateChapterDTO : public ChapterDTOBase
{
    Q_OBJECT
    Q_PROPERTY(int id READ id WRITE setId)
  public:
    UpdateChapterDTO(QObject *parent = nullptr) : ChapterDTOBase(parent)
    {
    }

    UpdateChapterDTO(int id, const QString &title) : ChapterDTOBase(title), m_id(id)
    {
    }

    UpdateChapterDTO(const UpdateChapterDTO &other) : ChapterDTOBase(other), m_id(other.m_id)
    {
    }
    UpdateChapterDTO &operator=(const UpdateChapterDTO &other)
    {
        if (this != &other)
        {
            ChapterDTOBase::operator=(other);
            m_id = other.m_id;
        }
        return *this;
    }
    int id() const;
    void setId(int newId);

  private:
    int m_id;
};

//-------------------------------------------------

inline int UpdateChapterDTO::id() const
{
    return m_id;
}

inline void UpdateChapterDTO::setId(int newId)
{
    m_id = newId;
}

} // namespace Contracts::DTO::Chapter

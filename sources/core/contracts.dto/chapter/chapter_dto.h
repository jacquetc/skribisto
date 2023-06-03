#pragma once

#include "chapter_dto_base.h"
#include <QUuid>

namespace Contracts::DTO::Chapter
{

//-------------------------------------------------

class ChapterDTO : public ChapterDTOBase
{
    Q_OBJECT
    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
  public:
    ChapterDTO(QObject *parent = nullptr) : ChapterDTOBase(parent)
    {
    }

    ChapterDTO(int id, const QUuid &uuid, const QString &title) : ChapterDTOBase(title), m_id(id), m_uuid(uuid)
    {
    }

    ChapterDTO(const ChapterDTO &other) : ChapterDTOBase(other), m_id(other.m_id), m_uuid(other.m_uuid)
    {
    }
    ChapterDTO &operator=(const ChapterDTO &other)
    {
        if (this != &other)
        {
            ChapterDTOBase::operator=(other);
            m_id = other.m_id;
            m_uuid = other.m_uuid;
        }
        return *this;
    }

    bool operator==(const ChapterDTO &other) const
    {

        return ChapterDTOBase::operator==(other) && m_id == other.m_id && m_uuid == other.m_uuid;
    }
    int id() const;
    void setId(int newId);
    QUuid uuid() const;
    void setUuid(const QUuid &newUuid);

  private:
    int m_id;
    QUuid m_uuid;
};

inline QUuid ChapterDTO::uuid() const
{
    return m_uuid;
}

inline void ChapterDTO::setUuid(const QUuid &newUuid)
{

    m_uuid = newUuid;
}
//-------------------------------------------------

inline int ChapterDTO::id() const
{
    return m_id;
}

inline void ChapterDTO::setId(int newId)
{

    m_id = newId;
}

} // namespace Contracts::DTO::Chapter

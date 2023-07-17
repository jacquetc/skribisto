#pragma once

#include <QObject>
#include <QUuid>

namespace Contracts::DTO::Writing
{

class GetSceneParagraphIdDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid paragraphUuid READ paragraphUuid WRITE setParagraphUuid)

  public:
    GetSceneParagraphIdDTO() : m_paragraphUuid(QUuid())
    {
    }

    ~GetSceneParagraphIdDTO()
    {
    }

    GetSceneParagraphIdDTO(const QUuid &paragraphUuid) : m_paragraphUuid(paragraphUuid)
    {
    }

    GetSceneParagraphIdDTO(const GetSceneParagraphIdDTO &other) : m_paragraphUuid(other.m_paragraphUuid)
    {
    }

    GetSceneParagraphIdDTO &operator=(const GetSceneParagraphIdDTO &other)
    {
        if (this != &other)
        {
            m_paragraphUuid = other.m_paragraphUuid;
        }
        return *this;
    }

    friend bool operator==(const GetSceneParagraphIdDTO &lhs, const GetSceneParagraphIdDTO &rhs);

    friend uint qHash(const GetSceneParagraphIdDTO &dto, uint seed) noexcept;

    // ------ paragraphUuid : -----

    QUuid paragraphUuid() const
    {
        return m_paragraphUuid;
    }

    void setParagraphUuid(const QUuid &paragraphUuid)
    {
        m_paragraphUuid = paragraphUuid;
    }

  private:
    QUuid m_paragraphUuid;
};

inline bool operator==(const GetSceneParagraphIdDTO &lhs, const GetSceneParagraphIdDTO &rhs)
{

    return lhs.m_paragraphUuid == rhs.m_paragraphUuid;
}

inline uint qHash(const GetSceneParagraphIdDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_paragraphUuid, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::GetSceneParagraphIdDTO)
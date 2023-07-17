#pragma once

#include <QObject>
#include <QUuid>

namespace Contracts::DTO::Writing
{

class GetSceneParagraphIdReplyDTO
{
    Q_GADGET

    Q_PROPERTY(int paragraphId READ paragraphId WRITE setParagraphId)
    Q_PROPERTY(QUuid paragraphUuid READ paragraphUuid WRITE setParagraphUuid)

  public:
    GetSceneParagraphIdReplyDTO() : m_paragraphId(0), m_paragraphUuid(QUuid())
    {
    }

    ~GetSceneParagraphIdReplyDTO()
    {
    }

    GetSceneParagraphIdReplyDTO(int paragraphId, const QUuid &paragraphUuid)
        : m_paragraphId(paragraphId), m_paragraphUuid(paragraphUuid)
    {
    }

    GetSceneParagraphIdReplyDTO(const GetSceneParagraphIdReplyDTO &other)
        : m_paragraphId(other.m_paragraphId), m_paragraphUuid(other.m_paragraphUuid)
    {
    }

    GetSceneParagraphIdReplyDTO &operator=(const GetSceneParagraphIdReplyDTO &other)
    {
        if (this != &other)
        {
            m_paragraphId = other.m_paragraphId;
            m_paragraphUuid = other.m_paragraphUuid;
        }
        return *this;
    }

    friend bool operator==(const GetSceneParagraphIdReplyDTO &lhs, const GetSceneParagraphIdReplyDTO &rhs);

    friend uint qHash(const GetSceneParagraphIdReplyDTO &dto, uint seed) noexcept;

    // ------ paragraphId : -----

    int paragraphId() const
    {
        return m_paragraphId;
    }

    void setParagraphId(int paragraphId)
    {
        m_paragraphId = paragraphId;
    }

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
    int m_paragraphId;
    QUuid m_paragraphUuid;
};

inline bool operator==(const GetSceneParagraphIdReplyDTO &lhs, const GetSceneParagraphIdReplyDTO &rhs)
{

    return lhs.m_paragraphId == rhs.m_paragraphId && lhs.m_paragraphUuid == rhs.m_paragraphUuid;
}

inline uint qHash(const GetSceneParagraphIdReplyDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_paragraphId, seed);
    hash ^= ::qHash(dto.m_paragraphUuid, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::GetSceneParagraphIdReplyDTO)
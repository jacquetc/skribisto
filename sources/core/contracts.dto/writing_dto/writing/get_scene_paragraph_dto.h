#pragma once

#include <QObject>

namespace Contracts::DTO::Writing
{

class GetSceneParagraphDTO
{
    Q_GADGET

    Q_PROPERTY(int paragraphId READ paragraphId WRITE setParagraphId)

  public:
    GetSceneParagraphDTO() : m_paragraphId(0)
    {
    }

    ~GetSceneParagraphDTO()
    {
    }

    GetSceneParagraphDTO(int paragraphId) : m_paragraphId(paragraphId)
    {
    }

    GetSceneParagraphDTO(const GetSceneParagraphDTO &other) : m_paragraphId(other.m_paragraphId)
    {
    }

    GetSceneParagraphDTO &operator=(const GetSceneParagraphDTO &other)
    {
        if (this != &other)
        {
            m_paragraphId = other.m_paragraphId;
        }
        return *this;
    }

    friend bool operator==(const GetSceneParagraphDTO &lhs, const GetSceneParagraphDTO &rhs);

    friend uint qHash(const GetSceneParagraphDTO &dto, uint seed) noexcept;

    // ------ paragraphId : -----

    int paragraphId() const
    {
        return m_paragraphId;
    }

    void setParagraphId(int paragraphId)
    {
        m_paragraphId = paragraphId;
    }

  private:
    int m_paragraphId;
};

inline bool operator==(const GetSceneParagraphDTO &lhs, const GetSceneParagraphDTO &rhs)
{

    return lhs.m_paragraphId == rhs.m_paragraphId;
}

inline uint qHash(const GetSceneParagraphDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_paragraphId, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::GetSceneParagraphDTO)
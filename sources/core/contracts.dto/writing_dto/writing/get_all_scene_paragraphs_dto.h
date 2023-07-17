#pragma once

#include <QObject>

namespace Contracts::DTO::Writing
{

class GetAllSceneParagraphsDTO
{
    Q_GADGET

    Q_PROPERTY(int sceneId READ sceneId WRITE setSceneId)

  public:
    GetAllSceneParagraphsDTO() : m_sceneId(0)
    {
    }

    ~GetAllSceneParagraphsDTO()
    {
    }

    GetAllSceneParagraphsDTO(int sceneId) : m_sceneId(sceneId)
    {
    }

    GetAllSceneParagraphsDTO(const GetAllSceneParagraphsDTO &other) : m_sceneId(other.m_sceneId)
    {
    }

    GetAllSceneParagraphsDTO &operator=(const GetAllSceneParagraphsDTO &other)
    {
        if (this != &other)
        {
            m_sceneId = other.m_sceneId;
        }
        return *this;
    }

    friend bool operator==(const GetAllSceneParagraphsDTO &lhs, const GetAllSceneParagraphsDTO &rhs);

    friend uint qHash(const GetAllSceneParagraphsDTO &dto, uint seed) noexcept;

    // ------ sceneId : -----

    int sceneId() const
    {
        return m_sceneId;
    }

    void setSceneId(int sceneId)
    {
        m_sceneId = sceneId;
    }

  private:
    int m_sceneId;
};

inline bool operator==(const GetAllSceneParagraphsDTO &lhs, const GetAllSceneParagraphsDTO &rhs)
{

    return lhs.m_sceneId == rhs.m_sceneId;
}

inline uint qHash(const GetAllSceneParagraphsDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_sceneId, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::GetAllSceneParagraphsDTO)
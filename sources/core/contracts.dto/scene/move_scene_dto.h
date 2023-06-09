#pragma once

#include <QObject>




namespace Contracts::DTO::Scene
{

class MoveSceneDTO
{
    Q_GADGET

    Q_PROPERTY(int origiChapterId READ origiChapterId WRITE setOrigiChapterId)
    Q_PROPERTY(int targetedSceneId READ targetedSceneId WRITE setTargetedSceneId)
    Q_PROPERTY(int destinationChapterId READ destinationChapterId WRITE setDestinationChapterId)
    Q_PROPERTY(int scenePositionAtDestination READ scenePositionAtDestination WRITE setScenePositionAtDestination)

  public:
    MoveSceneDTO() : m_origiChapterId(0), m_targetedSceneId(0), m_destinationChapterId(0), m_scenePositionAtDestination(0)
    {
    }

    ~MoveSceneDTO()
    {
    }

    MoveSceneDTO( int origiChapterId,   int targetedSceneId,   int destinationChapterId,   int scenePositionAtDestination ) 
        : m_origiChapterId(origiChapterId), m_targetedSceneId(targetedSceneId), m_destinationChapterId(destinationChapterId), m_scenePositionAtDestination(scenePositionAtDestination)
    {
    }

    MoveSceneDTO(const MoveSceneDTO &other) : m_origiChapterId(other.m_origiChapterId), m_targetedSceneId(other.m_targetedSceneId), m_destinationChapterId(other.m_destinationChapterId), m_scenePositionAtDestination(other.m_scenePositionAtDestination)
    {
    }

    MoveSceneDTO &operator=(const MoveSceneDTO &other)
    {
        if (this != &other)
        {
            m_origiChapterId = other.m_origiChapterId;
            m_targetedSceneId = other.m_targetedSceneId;
            m_destinationChapterId = other.m_destinationChapterId;
            m_scenePositionAtDestination = other.m_scenePositionAtDestination;
            
        }
        return *this;
    }

    friend bool operator==(const MoveSceneDTO &lhs, const MoveSceneDTO &rhs);


    friend uint qHash(const MoveSceneDTO &dto, uint seed) noexcept;



    // ------ origiChapterId : -----

    int origiChapterId() const
    {
        return m_origiChapterId;
    }

    void setOrigiChapterId( int origiChapterId)
    {
        m_origiChapterId = origiChapterId;
    }
    

    // ------ targetedSceneId : -----

    int targetedSceneId() const
    {
        return m_targetedSceneId;
    }

    void setTargetedSceneId( int targetedSceneId)
    {
        m_targetedSceneId = targetedSceneId;
    }
    

    // ------ destinationChapterId : -----

    int destinationChapterId() const
    {
        return m_destinationChapterId;
    }

    void setDestinationChapterId( int destinationChapterId)
    {
        m_destinationChapterId = destinationChapterId;
    }
    

    // ------ scenePositionAtDestination : -----

    int scenePositionAtDestination() const
    {
        return m_scenePositionAtDestination;
    }

    void setScenePositionAtDestination( int scenePositionAtDestination)
    {
        m_scenePositionAtDestination = scenePositionAtDestination;
    }
    


  private:

    int m_origiChapterId;
    int m_targetedSceneId;
    int m_destinationChapterId;
    int m_scenePositionAtDestination;
};

inline bool operator==(const MoveSceneDTO &lhs, const MoveSceneDTO &rhs)
{

    return 
            lhs.m_origiChapterId == rhs.m_origiChapterId  && lhs.m_targetedSceneId == rhs.m_targetedSceneId  && lhs.m_destinationChapterId == rhs.m_destinationChapterId  && lhs.m_scenePositionAtDestination == rhs.m_scenePositionAtDestination 
    ;
}

inline uint qHash(const MoveSceneDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_origiChapterId, seed);
        hash ^= ::qHash(dto.m_targetedSceneId, seed);
        hash ^= ::qHash(dto.m_destinationChapterId, seed);
        hash ^= ::qHash(dto.m_scenePositionAtDestination, seed);
        

        return hash;
}

} // namespace Contracts::DTO::Scene
Q_DECLARE_METATYPE(Contracts::DTO::Scene::MoveSceneDTO)
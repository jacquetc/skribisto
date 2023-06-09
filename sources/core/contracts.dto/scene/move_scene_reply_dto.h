#pragma once

#include <QObject>




namespace Contracts::DTO::Scene
{

class MoveSceneReplyDTO
{
    Q_GADGET

    Q_PROPERTY(int originChapterId READ originChapterId WRITE setOriginChapterId)
    Q_PROPERTY(int targetedSceneId READ targetedSceneId WRITE setTargetedSceneId)
    Q_PROPERTY(int scenePositionAtOrigin READ scenePositionAtOrigin WRITE setScenePositionAtOrigin)
    Q_PROPERTY(int destinationChapterId READ destinationChapterId WRITE setDestinationChapterId)
    Q_PROPERTY(int scenePositionAtDestination READ scenePositionAtDestination WRITE setScenePositionAtDestination)

  public:
    MoveSceneReplyDTO() : m_originChapterId(0), m_targetedSceneId(0), m_scenePositionAtOrigin(0), m_destinationChapterId(0), m_scenePositionAtDestination(0)
    {
    }

    ~MoveSceneReplyDTO()
    {
    }

    MoveSceneReplyDTO( int originChapterId,   int targetedSceneId,   int scenePositionAtOrigin,   int destinationChapterId,   int scenePositionAtDestination ) 
        : m_originChapterId(originChapterId), m_targetedSceneId(targetedSceneId), m_scenePositionAtOrigin(scenePositionAtOrigin), m_destinationChapterId(destinationChapterId), m_scenePositionAtDestination(scenePositionAtDestination)
    {
    }

    MoveSceneReplyDTO(const MoveSceneReplyDTO &other) : m_originChapterId(other.m_originChapterId), m_targetedSceneId(other.m_targetedSceneId), m_scenePositionAtOrigin(other.m_scenePositionAtOrigin), m_destinationChapterId(other.m_destinationChapterId), m_scenePositionAtDestination(other.m_scenePositionAtDestination)
    {
    }

    MoveSceneReplyDTO &operator=(const MoveSceneReplyDTO &other)
    {
        if (this != &other)
        {
            m_originChapterId = other.m_originChapterId;
            m_targetedSceneId = other.m_targetedSceneId;
            m_scenePositionAtOrigin = other.m_scenePositionAtOrigin;
            m_destinationChapterId = other.m_destinationChapterId;
            m_scenePositionAtDestination = other.m_scenePositionAtDestination;
            
        }
        return *this;
    }

    friend bool operator==(const MoveSceneReplyDTO &lhs, const MoveSceneReplyDTO &rhs);


    friend uint qHash(const MoveSceneReplyDTO &dto, uint seed) noexcept;



    // ------ originChapterId : -----

    int originChapterId() const
    {
        return m_originChapterId;
    }

    void setOriginChapterId( int originChapterId)
    {
        m_originChapterId = originChapterId;
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
    

    // ------ scenePositionAtOrigin : -----

    int scenePositionAtOrigin() const
    {
        return m_scenePositionAtOrigin;
    }

    void setScenePositionAtOrigin( int scenePositionAtOrigin)
    {
        m_scenePositionAtOrigin = scenePositionAtOrigin;
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

    int m_originChapterId;
    int m_targetedSceneId;
    int m_scenePositionAtOrigin;
    int m_destinationChapterId;
    int m_scenePositionAtDestination;
};

inline bool operator==(const MoveSceneReplyDTO &lhs, const MoveSceneReplyDTO &rhs)
{

    return 
            lhs.m_originChapterId == rhs.m_originChapterId  && lhs.m_targetedSceneId == rhs.m_targetedSceneId  && lhs.m_scenePositionAtOrigin == rhs.m_scenePositionAtOrigin  && lhs.m_destinationChapterId == rhs.m_destinationChapterId  && lhs.m_scenePositionAtDestination == rhs.m_scenePositionAtDestination 
    ;
}

inline uint qHash(const MoveSceneReplyDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_originChapterId, seed);
        hash ^= ::qHash(dto.m_targetedSceneId, seed);
        hash ^= ::qHash(dto.m_scenePositionAtOrigin, seed);
        hash ^= ::qHash(dto.m_destinationChapterId, seed);
        hash ^= ::qHash(dto.m_scenePositionAtDestination, seed);
        

        return hash;
}

} // namespace Contracts::DTO::Scene
Q_DECLARE_METATYPE(Contracts::DTO::Scene::MoveSceneReplyDTO)
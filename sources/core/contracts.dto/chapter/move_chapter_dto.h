#pragma once

#include <QObject>




namespace Contracts::DTO::Chapter
{

class MoveChapterDTO
{
    Q_GADGET

    Q_PROPERTY(int originBookId READ originBookId WRITE setOriginBookId)
    Q_PROPERTY(int targetedSceneId READ targetedSceneId WRITE setTargetedSceneId)
    Q_PROPERTY(int destinationBookId READ destinationBookId WRITE setDestinationBookId)
    Q_PROPERTY(int scenePositionAtDestination READ scenePositionAtDestination WRITE setScenePositionAtDestination)

  public:
    MoveChapterDTO() : m_originBookId(0), m_targetedSceneId(0), m_destinationBookId(0), m_scenePositionAtDestination(0)
    {
    }

    ~MoveChapterDTO()
    {
    }

    MoveChapterDTO( int originBookId,   int targetedSceneId,   int destinationBookId,   int scenePositionAtDestination ) 
        : m_originBookId(originBookId), m_targetedSceneId(targetedSceneId), m_destinationBookId(destinationBookId), m_scenePositionAtDestination(scenePositionAtDestination)
    {
    }

    MoveChapterDTO(const MoveChapterDTO &other) : m_originBookId(other.m_originBookId), m_targetedSceneId(other.m_targetedSceneId), m_destinationBookId(other.m_destinationBookId), m_scenePositionAtDestination(other.m_scenePositionAtDestination)
    {
    }

    MoveChapterDTO &operator=(const MoveChapterDTO &other)
    {
        if (this != &other)
        {
            m_originBookId = other.m_originBookId;
            m_targetedSceneId = other.m_targetedSceneId;
            m_destinationBookId = other.m_destinationBookId;
            m_scenePositionAtDestination = other.m_scenePositionAtDestination;
            
        }
        return *this;
    }

    friend bool operator==(const MoveChapterDTO &lhs, const MoveChapterDTO &rhs);


    friend uint qHash(const MoveChapterDTO &dto, uint seed) noexcept;



    // ------ originBookId : -----

    int originBookId() const
    {
        return m_originBookId;
    }

    void setOriginBookId( int originBookId)
    {
        m_originBookId = originBookId;
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
    

    // ------ destinationBookId : -----

    int destinationBookId() const
    {
        return m_destinationBookId;
    }

    void setDestinationBookId( int destinationBookId)
    {
        m_destinationBookId = destinationBookId;
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

    int m_originBookId;
    int m_targetedSceneId;
    int m_destinationBookId;
    int m_scenePositionAtDestination;
};

inline bool operator==(const MoveChapterDTO &lhs, const MoveChapterDTO &rhs)
{

    return 
            lhs.m_originBookId == rhs.m_originBookId  && lhs.m_targetedSceneId == rhs.m_targetedSceneId  && lhs.m_destinationBookId == rhs.m_destinationBookId  && lhs.m_scenePositionAtDestination == rhs.m_scenePositionAtDestination 
    ;
}

inline uint qHash(const MoveChapterDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_originBookId, seed);
        hash ^= ::qHash(dto.m_targetedSceneId, seed);
        hash ^= ::qHash(dto.m_destinationBookId, seed);
        hash ^= ::qHash(dto.m_scenePositionAtDestination, seed);
        

        return hash;
}

} // namespace Contracts::DTO::Chapter
Q_DECLARE_METATYPE(Contracts::DTO::Chapter::MoveChapterDTO)
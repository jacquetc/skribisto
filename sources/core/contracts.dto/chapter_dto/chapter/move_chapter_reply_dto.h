#pragma once

#include <QObject>


namespace Contracts::DTO::Chapter {
class MoveChapterReplyDTO {
    Q_GADGET

    Q_PROPERTY(int originBookId READ originBookId WRITE setOriginBookId)
    Q_PROPERTY(int targetedChapterId READ targetedChapterId WRITE setTargetedChapterId)
    Q_PROPERTY(int scenePositionAtOrigin READ scenePositionAtOrigin WRITE setScenePositionAtOrigin)
    Q_PROPERTY(int destinationChapterId READ destinationChapterId WRITE setDestinationChapterId)
    Q_PROPERTY(int scenePositionAtDestination READ scenePositionAtDestination WRITE setScenePositionAtDestination)

public:

    MoveChapterReplyDTO() : m_originBookId(0), m_targetedChapterId(0), m_scenePositionAtOrigin(0),
        m_destinationChapterId(0), m_scenePositionAtDestination(0)
    {}

    ~MoveChapterReplyDTO()
    {}

    MoveChapterReplyDTO(int originBookId,
                        int targetedChapterId,
                        int scenePositionAtOrigin,
                        int destinationChapterId,
                        int scenePositionAtDestination)
        : m_originBookId(originBookId), m_targetedChapterId(targetedChapterId), m_scenePositionAtOrigin(
            scenePositionAtOrigin), m_destinationChapterId(destinationChapterId), m_scenePositionAtDestination(
            scenePositionAtDestination)
    {}

    MoveChapterReplyDTO(const MoveChapterReplyDTO& other) : m_originBookId(other.m_originBookId), m_targetedChapterId(
            other.m_targetedChapterId), m_scenePositionAtOrigin(other.m_scenePositionAtOrigin), m_destinationChapterId(
            other.m_destinationChapterId), m_scenePositionAtDestination(other.m_scenePositionAtDestination)
    {}

    MoveChapterReplyDTO& operator=(const MoveChapterReplyDTO& other)
    {
        if (this != &other)
        {
            m_originBookId               = other.m_originBookId;
            m_targetedChapterId          = other.m_targetedChapterId;
            m_scenePositionAtOrigin      = other.m_scenePositionAtOrigin;
            m_destinationChapterId       = other.m_destinationChapterId;
            m_scenePositionAtDestination = other.m_scenePositionAtDestination;
        }
        return *this;
    }

    friend bool operator==(const MoveChapterReplyDTO& lhs,
                           const MoveChapterReplyDTO& rhs);


    friend uint qHash(const MoveChapterReplyDTO& dto,
                      uint                       seed) noexcept;


    // ------ originBookId : -----

    int originBookId() const
    {
        return m_originBookId;
    }

    void setOriginBookId(int originBookId)
    {
        m_originBookId = originBookId;
    }

    // ------ targetedChapterId : -----

    int targetedChapterId() const
    {
        return m_targetedChapterId;
    }

    void setTargetedChapterId(int targetedChapterId)
    {
        m_targetedChapterId = targetedChapterId;
    }

    // ------ scenePositionAtOrigin : -----

    int scenePositionAtOrigin() const
    {
        return m_scenePositionAtOrigin;
    }

    void setScenePositionAtOrigin(int scenePositionAtOrigin)
    {
        m_scenePositionAtOrigin = scenePositionAtOrigin;
    }

    // ------ destinationChapterId : -----

    int destinationChapterId() const
    {
        return m_destinationChapterId;
    }

    void setDestinationChapterId(int destinationChapterId)
    {
        m_destinationChapterId = destinationChapterId;
    }

    // ------ scenePositionAtDestination : -----

    int scenePositionAtDestination() const
    {
        return m_scenePositionAtDestination;
    }

    void setScenePositionAtDestination(int scenePositionAtDestination)
    {
        m_scenePositionAtDestination = scenePositionAtDestination;
    }

private:

    int m_originBookId;
    int m_targetedChapterId;
    int m_scenePositionAtOrigin;
    int m_destinationChapterId;
    int m_scenePositionAtDestination;
};

inline bool operator==(const MoveChapterReplyDTO& lhs, const MoveChapterReplyDTO& rhs)
{
    return
        lhs.m_originBookId == rhs.m_originBookId  && lhs.m_targetedChapterId == rhs.m_targetedChapterId  &&
        lhs.m_scenePositionAtOrigin == rhs.m_scenePositionAtOrigin  &&
        lhs.m_destinationChapterId == rhs.m_destinationChapterId  &&
        lhs.m_scenePositionAtDestination == rhs.m_scenePositionAtDestination
    ;
}

inline uint qHash(const MoveChapterReplyDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_originBookId, seed);
    hash ^= ::qHash(dto.m_targetedChapterId, seed);
    hash ^= ::qHash(dto.m_scenePositionAtOrigin, seed);
    hash ^= ::qHash(dto.m_destinationChapterId, seed);
    hash ^= ::qHash(dto.m_scenePositionAtDestination, seed);


    return hash;
}
} // namespace Contracts::DTO::Chapter
Q_DECLARE_METATYPE(Contracts::DTO::Chapter::MoveChapterReplyDTO)

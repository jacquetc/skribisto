#pragma once

#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::Writing
{

class UpdateSceneParagraphDTO
{
    Q_GADGET

    Q_PROPERTY(int paragraphId READ paragraphId WRITE setParagraphId)
    Q_PROPERTY(QUuid paragraphUuid READ paragraphUuid WRITE setParagraphUuid)
    Q_PROPERTY(int sceneId READ sceneId WRITE setSceneId)
    Q_PROPERTY(QUuid sceneUuid READ sceneUuid WRITE setSceneUuid)
    Q_PROPERTY(int paragraphIndexInScene READ paragraphIndexInScene WRITE setParagraphIndexInScene)
    Q_PROPERTY(QString text READ text WRITE setText)
    Q_PROPERTY(int oldCursorPosition READ oldCursorPosition WRITE setOldCursorPosition)
    Q_PROPERTY(int newCursorPosition READ newCursorPosition WRITE setNewCursorPosition)

  public:
    UpdateSceneParagraphDTO()
        : m_paragraphId(0), m_paragraphUuid(QUuid()), m_sceneId(0), m_sceneUuid(QUuid()), m_paragraphIndexInScene(0),
          m_text(QString()), m_oldCursorPosition(0), m_newCursorPosition(0)
    {
    }

    ~UpdateSceneParagraphDTO()
    {
    }

    UpdateSceneParagraphDTO(int paragraphId, const QUuid &paragraphUuid, int sceneId, const QUuid &sceneUuid,
                            int paragraphIndexInScene, const QString &text, int oldCursorPosition,
                            int newCursorPosition)
        : m_paragraphId(paragraphId), m_paragraphUuid(paragraphUuid), m_sceneId(sceneId), m_sceneUuid(sceneUuid),
          m_paragraphIndexInScene(paragraphIndexInScene), m_text(text), m_oldCursorPosition(oldCursorPosition),
          m_newCursorPosition(newCursorPosition)
    {
    }

    UpdateSceneParagraphDTO(const UpdateSceneParagraphDTO &other)
        : m_paragraphId(other.m_paragraphId), m_paragraphUuid(other.m_paragraphUuid), m_sceneId(other.m_sceneId),
          m_sceneUuid(other.m_sceneUuid), m_paragraphIndexInScene(other.m_paragraphIndexInScene), m_text(other.m_text),
          m_oldCursorPosition(other.m_oldCursorPosition), m_newCursorPosition(other.m_newCursorPosition)
    {
    }

    UpdateSceneParagraphDTO &operator=(const UpdateSceneParagraphDTO &other)
    {
        if (this != &other)
        {
            m_paragraphId = other.m_paragraphId;
            m_paragraphUuid = other.m_paragraphUuid;
            m_sceneId = other.m_sceneId;
            m_sceneUuid = other.m_sceneUuid;
            m_paragraphIndexInScene = other.m_paragraphIndexInScene;
            m_text = other.m_text;
            m_oldCursorPosition = other.m_oldCursorPosition;
            m_newCursorPosition = other.m_newCursorPosition;
        }
        return *this;
    }

    friend bool operator==(const UpdateSceneParagraphDTO &lhs, const UpdateSceneParagraphDTO &rhs);

    friend uint qHash(const UpdateSceneParagraphDTO &dto, uint seed) noexcept;

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

    // ------ sceneId : -----

    int sceneId() const
    {
        return m_sceneId;
    }

    void setSceneId(int sceneId)
    {
        m_sceneId = sceneId;
    }

    // ------ sceneUuid : -----

    QUuid sceneUuid() const
    {
        return m_sceneUuid;
    }

    void setSceneUuid(const QUuid &sceneUuid)
    {
        m_sceneUuid = sceneUuid;
    }

    // ------ paragraphIndexInScene : -----

    int paragraphIndexInScene() const
    {
        return m_paragraphIndexInScene;
    }

    void setParagraphIndexInScene(int paragraphIndexInScene)
    {
        m_paragraphIndexInScene = paragraphIndexInScene;
    }

    // ------ text : -----

    QString text() const
    {
        return m_text;
    }

    void setText(const QString &text)
    {
        m_text = text;
    }

    // ------ oldCursorPosition : -----

    int oldCursorPosition() const
    {
        return m_oldCursorPosition;
    }

    void setOldCursorPosition(int oldCursorPosition)
    {
        m_oldCursorPosition = oldCursorPosition;
    }

    // ------ newCursorPosition : -----

    int newCursorPosition() const
    {
        return m_newCursorPosition;
    }

    void setNewCursorPosition(int newCursorPosition)
    {
        m_newCursorPosition = newCursorPosition;
    }

  private:
    int m_paragraphId;
    QUuid m_paragraphUuid;
    int m_sceneId;
    QUuid m_sceneUuid;
    int m_paragraphIndexInScene;
    QString m_text;
    int m_oldCursorPosition;
    int m_newCursorPosition;
};

inline bool operator==(const UpdateSceneParagraphDTO &lhs, const UpdateSceneParagraphDTO &rhs)
{

    return lhs.m_paragraphId == rhs.m_paragraphId && lhs.m_paragraphUuid == rhs.m_paragraphUuid &&
           lhs.m_sceneId == rhs.m_sceneId && lhs.m_sceneUuid == rhs.m_sceneUuid &&
           lhs.m_paragraphIndexInScene == rhs.m_paragraphIndexInScene && lhs.m_text == rhs.m_text &&
           lhs.m_oldCursorPosition == rhs.m_oldCursorPosition && lhs.m_newCursorPosition == rhs.m_newCursorPosition;
}

inline uint qHash(const UpdateSceneParagraphDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_paragraphId, seed);
    hash ^= ::qHash(dto.m_paragraphUuid, seed);
    hash ^= ::qHash(dto.m_sceneId, seed);
    hash ^= ::qHash(dto.m_sceneUuid, seed);
    hash ^= ::qHash(dto.m_paragraphIndexInScene, seed);
    hash ^= ::qHash(dto.m_text, seed);
    hash ^= ::qHash(dto.m_oldCursorPosition, seed);
    hash ^= ::qHash(dto.m_newCursorPosition, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::UpdateSceneParagraphDTO)

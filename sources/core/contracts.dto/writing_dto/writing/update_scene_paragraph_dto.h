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
    Q_PROPERTY(QUuid sceneUuid READ sceneUuid WRITE setSceneUuid)
    Q_PROPERTY(QString text READ text WRITE setText)
    Q_PROPERTY(int cursorPosition READ cursorPosition WRITE setCursorPosition)

  public:
    UpdateSceneParagraphDTO()
        : m_paragraphId(0), m_paragraphUuid(QUuid()), m_sceneUuid(QUuid()), m_text(QString()), m_cursorPosition(0)
    {
    }

    ~UpdateSceneParagraphDTO()
    {
    }

    UpdateSceneParagraphDTO(int paragraphId, const QUuid &paragraphUuid, const QUuid &sceneUuid, const QString &text,
                            int cursorPosition)
        : m_paragraphId(paragraphId), m_paragraphUuid(paragraphUuid), m_sceneUuid(sceneUuid), m_text(text),
          m_cursorPosition(cursorPosition)
    {
    }

    UpdateSceneParagraphDTO(const UpdateSceneParagraphDTO &other)
        : m_paragraphId(other.m_paragraphId), m_paragraphUuid(other.m_paragraphUuid), m_sceneUuid(other.m_sceneUuid),
          m_text(other.m_text), m_cursorPosition(other.m_cursorPosition)
    {
    }

    UpdateSceneParagraphDTO &operator=(const UpdateSceneParagraphDTO &other)
    {
        if (this != &other)
        {
            m_paragraphId = other.m_paragraphId;
            m_paragraphUuid = other.m_paragraphUuid;
            m_sceneUuid = other.m_sceneUuid;
            m_text = other.m_text;
            m_cursorPosition = other.m_cursorPosition;
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

    // ------ sceneUuid : -----

    QUuid sceneUuid() const
    {
        return m_sceneUuid;
    }

    void setSceneUuid(const QUuid &sceneUuid)
    {
        m_sceneUuid = sceneUuid;
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

    // ------ cursorPosition : -----

    int cursorPosition() const
    {
        return m_cursorPosition;
    }

    void setCursorPosition(int cursorPosition)
    {
        m_cursorPosition = cursorPosition;
    }

  private:
    int m_paragraphId;
    QUuid m_paragraphUuid;
    QUuid m_sceneUuid;
    QString m_text;
    int m_cursorPosition;
};

inline bool operator==(const UpdateSceneParagraphDTO &lhs, const UpdateSceneParagraphDTO &rhs)
{

    return lhs.m_paragraphId == rhs.m_paragraphId && lhs.m_paragraphUuid == rhs.m_paragraphUuid &&
           lhs.m_sceneUuid == rhs.m_sceneUuid && lhs.m_text == rhs.m_text &&
           lhs.m_cursorPosition == rhs.m_cursorPosition;
}

inline uint qHash(const UpdateSceneParagraphDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_paragraphId, seed);
    hash ^= ::qHash(dto.m_paragraphUuid, seed);
    hash ^= ::qHash(dto.m_sceneUuid, seed);
    hash ^= ::qHash(dto.m_text, seed);
    hash ^= ::qHash(dto.m_cursorPosition, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::UpdateSceneParagraphDTO)
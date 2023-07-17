#pragma once

#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::Writing
{

class GetAllSceneParagraphsReplyDTO
{
    Q_GADGET

    Q_PROPERTY(int sceneId READ sceneId WRITE setSceneId)
    Q_PROPERTY(QList<int> paragraphIdList READ paragraphIdList WRITE setParagraphIdList)
    Q_PROPERTY(QList<QUuid> paragraphUuidList READ paragraphUuidList WRITE setParagraphUuidList)
    Q_PROPERTY(QList<QString> texts READ texts WRITE setTexts)

  public:
    GetAllSceneParagraphsReplyDTO() : m_sceneId(0), m_paragraphIdList({}), m_paragraphUuidList({}), m_texts({})
    {
    }

    ~GetAllSceneParagraphsReplyDTO()
    {
    }

    GetAllSceneParagraphsReplyDTO(int sceneId, const QList<int> &paragraphIdList, const QList<QUuid> &paragraphUuidList,
                                  const QList<QString> &texts)
        : m_sceneId(sceneId), m_paragraphIdList(paragraphIdList), m_paragraphUuidList(paragraphUuidList), m_texts(texts)
    {
    }

    GetAllSceneParagraphsReplyDTO(const GetAllSceneParagraphsReplyDTO &other)
        : m_sceneId(other.m_sceneId), m_paragraphIdList(other.m_paragraphIdList),
          m_paragraphUuidList(other.m_paragraphUuidList), m_texts(other.m_texts)
    {
    }

    GetAllSceneParagraphsReplyDTO &operator=(const GetAllSceneParagraphsReplyDTO &other)
    {
        if (this != &other)
        {
            m_sceneId = other.m_sceneId;
            m_paragraphIdList = other.m_paragraphIdList;
            m_paragraphUuidList = other.m_paragraphUuidList;
            m_texts = other.m_texts;
        }
        return *this;
    }

    friend bool operator==(const GetAllSceneParagraphsReplyDTO &lhs, const GetAllSceneParagraphsReplyDTO &rhs);

    friend uint qHash(const GetAllSceneParagraphsReplyDTO &dto, uint seed) noexcept;

    // ------ sceneId : -----

    int sceneId() const
    {
        return m_sceneId;
    }

    void setSceneId(int sceneId)
    {
        m_sceneId = sceneId;
    }

    // ------ paragraphIdList : -----

    QList<int> paragraphIdList() const
    {
        return m_paragraphIdList;
    }

    void setParagraphIdList(const QList<int> &paragraphIdList)
    {
        m_paragraphIdList = paragraphIdList;
    }

    // ------ paragraphUuidList : -----

    QList<QUuid> paragraphUuidList() const
    {
        return m_paragraphUuidList;
    }

    void setParagraphUuidList(const QList<QUuid> &paragraphUuidList)
    {
        m_paragraphUuidList = paragraphUuidList;
    }

    // ------ texts : -----

    QList<QString> texts() const
    {
        return m_texts;
    }

    void setTexts(const QList<QString> &texts)
    {
        m_texts = texts;
    }

  private:
    int m_sceneId;
    QList<int> m_paragraphIdList;
    QList<QUuid> m_paragraphUuidList;
    QList<QString> m_texts;
};

inline bool operator==(const GetAllSceneParagraphsReplyDTO &lhs, const GetAllSceneParagraphsReplyDTO &rhs)
{

    return lhs.m_sceneId == rhs.m_sceneId && lhs.m_paragraphIdList == rhs.m_paragraphIdList &&
           lhs.m_paragraphUuidList == rhs.m_paragraphUuidList && lhs.m_texts == rhs.m_texts;
}

inline uint qHash(const GetAllSceneParagraphsReplyDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_sceneId, seed);
    hash ^= ::qHash(dto.m_paragraphIdList, seed);
    hash ^= ::qHash(dto.m_paragraphUuidList, seed);
    hash ^= ::qHash(dto.m_texts, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::GetAllSceneParagraphsReplyDTO)

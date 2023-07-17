#pragma once

#include <QObject>
#include <QUuid>

namespace Contracts::DTO::Writing
{

class UpdateMultipleSceneParagraphsDTO
{
    Q_GADGET

    Q_PROPERTY(int sceneId READ sceneId WRITE setSceneId)
    Q_PROPERTY(int parapgraphIndex READ parapgraphIndex WRITE setParapgraphIndex)
    Q_PROPERTY(QList<int> paragraphIdList READ paragraphIdList WRITE setParagraphIdList)
    Q_PROPERTY(QList<QUuid> paragraphUuidList READ paragraphUuidList WRITE setParagraphUuidList)
    Q_PROPERTY(QList<QUuid> uuidListToDelete READ uuidListToDelete WRITE setUuidListToDelete)
    Q_PROPERTY(QList<QString> texts READ texts WRITE setTexts)

  public:
    UpdateMultipleSceneParagraphsDTO()
        : m_sceneId(0), m_parapgraphIndex(0), m_paragraphIdList({}), m_paragraphUuidList({}), m_uuidListToDelete({}),
          m_texts({})
    {
    }

    ~UpdateMultipleSceneParagraphsDTO()
    {
    }

    UpdateMultipleSceneParagraphsDTO(int sceneId, int parapgraphIndex, const QList<int> &paragraphIdList,
                                     const QList<QUuid> &paragraphUuidList, const QList<QUuid> &uuidListToDelete,
                                     const QList<QString> &texts)
        : m_sceneId(sceneId), m_parapgraphIndex(parapgraphIndex), m_paragraphIdList(paragraphIdList),
          m_paragraphUuidList(paragraphUuidList), m_uuidListToDelete(uuidListToDelete), m_texts(texts)
    {
    }

    UpdateMultipleSceneParagraphsDTO(const UpdateMultipleSceneParagraphsDTO &other)
        : m_sceneId(other.m_sceneId), m_parapgraphIndex(other.m_parapgraphIndex),
          m_paragraphIdList(other.m_paragraphIdList), m_paragraphUuidList(other.m_paragraphUuidList),
          m_uuidListToDelete(other.m_uuidListToDelete), m_texts(other.m_texts)
    {
    }

    UpdateMultipleSceneParagraphsDTO &operator=(const UpdateMultipleSceneParagraphsDTO &other)
    {
        if (this != &other)
        {
            m_sceneId = other.m_sceneId;
            m_parapgraphIndex = other.m_parapgraphIndex;
            m_paragraphIdList = other.m_paragraphIdList;
            m_paragraphUuidList = other.m_paragraphUuidList;
            m_uuidListToDelete = other.m_uuidListToDelete;
            m_texts = other.m_texts;
        }
        return *this;
    }

    friend bool operator==(const UpdateMultipleSceneParagraphsDTO &lhs, const UpdateMultipleSceneParagraphsDTO &rhs);

    friend uint qHash(const UpdateMultipleSceneParagraphsDTO &dto, uint seed) noexcept;

    // ------ sceneId : -----

    int sceneId() const
    {
        return m_sceneId;
    }

    void setSceneId(int sceneId)
    {
        m_sceneId = sceneId;
    }

    // ------ parapgraphIndex : -----

    int parapgraphIndex() const
    {
        return m_parapgraphIndex;
    }

    void setParapgraphIndex(int parapgraphIndex)
    {
        m_parapgraphIndex = parapgraphIndex;
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

    // ------ uuidListToDelete : -----

    QList<QUuid> uuidListToDelete() const
    {
        return m_uuidListToDelete;
    }

    void setUuidListToDelete(const QList<QUuid> &uuidListToDelete)
    {
        m_uuidListToDelete = uuidListToDelete;
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
    int m_parapgraphIndex;
    QList<int> m_paragraphIdList;
    QList<QUuid> m_paragraphUuidList;
    QList<QUuid> m_uuidListToDelete;
    QList<QString> m_texts;
};

inline bool operator==(const UpdateMultipleSceneParagraphsDTO &lhs, const UpdateMultipleSceneParagraphsDTO &rhs)
{

    return lhs.m_sceneId == rhs.m_sceneId && lhs.m_parapgraphIndex == rhs.m_parapgraphIndex &&
           lhs.m_paragraphIdList == rhs.m_paragraphIdList && lhs.m_paragraphUuidList == rhs.m_paragraphUuidList &&
           lhs.m_uuidListToDelete == rhs.m_uuidListToDelete && lhs.m_texts == rhs.m_texts;
}

inline uint qHash(const UpdateMultipleSceneParagraphsDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_sceneId, seed);
    hash ^= ::qHash(dto.m_parapgraphIndex, seed);
    hash ^= ::qHash(dto.m_paragraphIdList, seed);
    hash ^= ::qHash(dto.m_paragraphUuidList, seed);
    hash ^= ::qHash(dto.m_uuidListToDelete, seed);
    hash ^= ::qHash(dto.m_texts, seed);

    return hash;
}

} // namespace Contracts::DTO::Writing
Q_DECLARE_METATYPE(Contracts::DTO::Writing::UpdateMultipleSceneParagraphsDTO)

#pragma once

#include "domain_global.h"
#include "dummyotherentity.h"
#include <QString>

#include "dummy_entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT DummyEntityWithForeign : public DummyEntity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QList<DummyOtherEntity> lists READ lists WRITE setLists)

    Q_PROPERTY(bool listsLoaded MEMBER m_listsLoaded)

    Q_PROPERTY(QSet<DummyOtherEntity> sets READ sets WRITE setSets)

    Q_PROPERTY(bool setsLoaded MEMBER m_setsLoaded)

    Q_PROPERTY(DummyOtherEntity unique READ unique WRITE setUnique)

    Q_PROPERTY(bool uniqueLoaded MEMBER m_uniqueLoaded)

  public:
    DummyEntityWithForeign() : DummyEntity(), m_name(QString())
    {
    }

    ~DummyEntityWithForeign()
    {
    }

    DummyEntityWithForeign(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QString &name,
                           const QList<DummyOtherEntity> &lists, const QSet<DummyOtherEntity> &sets,
                           const DummyOtherEntity &unique)
        : DummyEntity(id, uuid, creationDate), m_name(name), m_lists(lists), m_sets(sets), m_unique(unique)
    {
    }

    DummyEntityWithForeign(const DummyEntityWithForeign &other)
        : DummyEntity(other), m_name(other.m_name), m_lists(other.m_lists), m_listsLoaded(other.m_listsLoaded),
          m_sets(other.m_sets), m_setsLoaded(other.m_setsLoaded), m_unique(other.m_unique),
          m_uniqueLoaded(other.m_uniqueLoaded)
    {
    }

    DummyEntityWithForeign &operator=(const DummyEntityWithForeign &other)
    {
        if (this != &other)
        {
            DummyEntity::operator=(other);
            m_name = other.m_name;
            m_lists = other.m_lists;
            m_listsLoaded = other.m_listsLoaded;
            m_sets = other.m_sets;
            m_setsLoaded = other.m_setsLoaded;
            m_unique = other.m_unique;
            m_uniqueLoaded = other.m_uniqueLoaded;
        }
        return *this;
    }

    friend bool operator==(const DummyEntityWithForeign &lhs, const DummyEntityWithForeign &rhs);

    friend uint qHash(const DummyEntityWithForeign &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ lists : -----

    QList<DummyOtherEntity> lists()
    {
        if (!m_listsLoaded && m_listsLoader)
        {
            m_lists = m_listsLoader();
            m_listsLoaded = true;
        }
        return m_lists;
    }

    void setLists(const QList<DummyOtherEntity> &lists)
    {
        m_lists = lists;
    }

    using ListsLoader = std::function<QList<DummyOtherEntity>()>;

    void setListsLoader(const ListsLoader &loader)
    {
        m_listsLoader = loader;
    }

    // ------ sets : -----

    QSet<DummyOtherEntity> sets()
    {
        if (!m_setsLoaded && m_setsLoader)
        {
            m_sets = m_setsLoader();
            m_setsLoaded = true;
        }
        return m_sets;
    }

    void setSets(const QSet<DummyOtherEntity> &sets)
    {
        m_sets = sets;
    }

    using SetsLoader = std::function<QSet<DummyOtherEntity>()>;

    void setSetsLoader(const SetsLoader &loader)
    {
        m_setsLoader = loader;
    }

    // ------ unique : -----

    DummyOtherEntity unique()
    {
        if (!m_uniqueLoaded && m_uniqueLoader)
        {
            m_unique = m_uniqueLoader();
            m_uniqueLoaded = true;
        }
        return m_unique;
    }

    void setUnique(const DummyOtherEntity &unique)
    {
        m_unique = unique;
    }

    using UniqueLoader = std::function<DummyOtherEntity()>;

    void setUniqueLoader(const UniqueLoader &loader)
    {
        m_uniqueLoader = loader;
    }

  private:
    QString m_name;
    QList<DummyOtherEntity> m_lists;
    ListsLoader m_listsLoader;
    bool m_listsLoaded = false;
    QSet<DummyOtherEntity> m_sets;
    SetsLoader m_setsLoader;
    bool m_setsLoaded = false;
    DummyOtherEntity m_unique;
    UniqueLoader m_uniqueLoader;
    bool m_uniqueLoaded = false;
};

inline bool operator==(const DummyEntityWithForeign &lhs, const DummyEntityWithForeign &rhs)
{

    return static_cast<const DummyEntity &>(lhs) == static_cast<const DummyEntity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_lists == rhs.m_lists && lhs.m_sets == rhs.m_sets &&
           lhs.m_unique == rhs.m_unique;
}

inline uint qHash(const DummyEntityWithForeign &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const DummyEntity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_lists, seed);
    hash ^= ::qHash(entity.m_sets, seed);
    hash ^= ::qHash(entity.m_unique, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyEntityWithForeign)

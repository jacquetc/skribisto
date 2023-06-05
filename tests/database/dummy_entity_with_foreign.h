#pragma once

#include "dummy_other_entity.h"
#include "entity.h"

namespace Domain
{

class DummyEntityWithForeign : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QList<DummyOtherEntity> lists READ lists WRITE setLists)
    Q_PROPERTY(bool listsLoaded MEMBER m_listsLoaded)
    Q_PROPERTY(QSet<DummyOtherEntity> sets READ sets WRITE setSets)
    Q_PROPERTY(bool setsLoaded MEMBER m_setsLoaded)
    Q_PROPERTY(DummyOtherEntity unique READ unique WRITE setUnique)
    Q_PROPERTY(bool uniqueLoaded MEMBER m_uniqueLoaded)

  public:
    DummyEntityWithForeign() : Entity(){};

    DummyEntityWithForeign(int id, const QUuid &uuid, const QString &name, const QList<DummyOtherEntity> &lists,
                           const QSet<DummyOtherEntity> &sets, const DummyOtherEntity &unique)
        : Entity(id, uuid, QDateTime(), QDateTime())
    {
        m_name = name;
        m_lists = lists;
        m_sets = sets;
        m_unique = unique;
    }

    DummyEntityWithForeign(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                           const QString &name, const QList<DummyOtherEntity> &lists,
                           const QSet<DummyOtherEntity> &sets, const DummyOtherEntity &unique)
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_lists(lists), m_sets(sets), m_unique(unique)
    {
    }

    DummyEntityWithForeign(const DummyEntityWithForeign &other)
        : Entity(other), m_name(other.m_name), m_lists(other.m_lists)
    {
    }

    DummyEntityWithForeign &operator=(const DummyEntityWithForeign &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_lists = other.m_lists;
            m_sets = other.m_sets;
            m_unique = other.m_unique;
        }
        return *this;
    }

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

} // namespace Domain
Q_DECLARE_METATYPE(Domain::DummyEntityWithForeign)

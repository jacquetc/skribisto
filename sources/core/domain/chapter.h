#pragma once

#include "domain_global.h"
#include "scene.h"
#include <QString>

#include "entities.h"
#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Chapter : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    Q_PROPERTY(QList<Scene> scenes READ scenes WRITE setScenes)

    Q_PROPERTY(bool scenesLoaded MEMBER m_scenesLoaded)

  public:
    Chapter() : Entity(), m_title(QString())
    {
    }

    ~Chapter()
    {
    }

    Chapter(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
            const QString &title, const QList<Scene> &scenes)
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_scenes(scenes)
    {
    }

    Chapter(const Chapter &other)
        : Entity(other), m_title(other.m_title), m_scenes(other.m_scenes), m_scenesLoaded(other.m_scenesLoaded)
    {
    }

    static Domain::Entities::EntityEnum enumValue()
    {
        return Domain::Entities::EntityEnum::Chapter;
    }

    Chapter &operator=(const Chapter &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_scenes = other.m_scenes;
            m_scenesLoaded = other.m_scenesLoaded;
        }
        return *this;
    }

    friend bool operator==(const Chapter &lhs, const Chapter &rhs);

    friend uint qHash(const Chapter &entity, uint seed) noexcept;

    // ------ title : -----

    QString title() const
    {

        return m_title;
    }

    void setTitle(const QString &title)
    {
        m_title = title;
    }

    // ------ scenes : -----

    QList<Scene> scenes()
    {
        if (!m_scenesLoaded && m_scenesLoader)
        {
            m_scenes = m_scenesLoader(this->id());
            m_scenesLoaded = true;
        }
        return m_scenes;
    }

    void setScenes(const QList<Scene> &scenes)
    {
        m_scenes = scenes;
    }

    using ScenesLoader = std::function<QList<Scene>(int entityId)>;

    void setScenesLoader(const ScenesLoader &loader)
    {
        m_scenesLoader = loader;
    }

  private:
    QString m_title;
    QList<Scene> m_scenes;
    ScenesLoader m_scenesLoader;
    bool m_scenesLoaded = false;
};

inline bool operator==(const Chapter &lhs, const Chapter &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_title == rhs.m_title && lhs.m_scenes == rhs.m_scenes;
}

inline uint qHash(const Chapter &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_title, seed);
    hash ^= ::qHash(entity.m_scenes, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Chapter)
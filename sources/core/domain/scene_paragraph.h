#pragma once

#include "domain_global.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT SceneParagraph : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString content READ content WRITE setContent)

  public:
    SceneParagraph() : Entity(), m_content(QString())
    {
    }

    ~SceneParagraph()
    {
    }

    SceneParagraph(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                   const QString &content)
        : Entity(id, uuid, creationDate, updateDate), m_content(content)
    {
    }

    SceneParagraph(const SceneParagraph &other) : Entity(other), m_content(other.m_content)
    {
    }

    SceneParagraph &operator=(const SceneParagraph &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_content = other.m_content;
        }
        return *this;
    }

    friend bool operator==(const SceneParagraph &lhs, const SceneParagraph &rhs);

    friend uint qHash(const SceneParagraph &entity, uint seed) noexcept;

    // ------ content : -----

    QString content() const
    {

        return m_content;
    }

    void setContent(const QString &content)
    {
        m_content = content;
    }

  private:
    QString m_content;
};

inline bool operator==(const SceneParagraph &lhs, const SceneParagraph &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_content == rhs.m_content;
}

inline uint qHash(const SceneParagraph &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_content, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::SceneParagraph)
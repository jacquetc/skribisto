#pragma once

#include "domain_global.h"
#include "scene_paragraph.h"
#include <QString>

#include "entities.h"
#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Scene : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    Q_PROPERTY(QList<SceneParagraph> paragraphs READ paragraphs WRITE setParagraphs)

    Q_PROPERTY(bool paragraphsLoaded MEMBER m_paragraphsLoaded)

  public:
    Scene() : Entity(), m_title(QString())
    {
    }

    ~Scene()
    {
    }

    Scene(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
          const QString &title, const QList<SceneParagraph> &paragraphs)
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_paragraphs(paragraphs)
    {
    }

    Scene(const Scene &other)
        : Entity(other), m_title(other.m_title), m_paragraphs(other.m_paragraphs),
          m_paragraphsLoaded(other.m_paragraphsLoaded)
    {
    }

    static Domain::Entities::EntityEnum enumValue()
    {
        return Domain::Entities::EntityEnum::Scene;
    }

    Scene &operator=(const Scene &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_paragraphs = other.m_paragraphs;
            m_paragraphsLoaded = other.m_paragraphsLoaded;
        }
        return *this;
    }

    friend bool operator==(const Scene &lhs, const Scene &rhs);

    friend uint qHash(const Scene &entity, uint seed) noexcept;

    // ------ title : -----

    QString title() const
    {

        return m_title;
    }

    void setTitle(const QString &title)
    {
        m_title = title;
    }

    // ------ paragraphs : -----

    QList<SceneParagraph> paragraphs()
    {
        if (!m_paragraphsLoaded && m_paragraphsLoader)
        {
            m_paragraphs = m_paragraphsLoader(this->id());
            m_paragraphsLoaded = true;
        }
        return m_paragraphs;
    }

    void setParagraphs(const QList<SceneParagraph> &paragraphs)
    {
        m_paragraphs = paragraphs;
    }

    using ParagraphsLoader = std::function<QList<SceneParagraph>(int entityId)>;

    void setParagraphsLoader(const ParagraphsLoader &loader)
    {
        m_paragraphsLoader = loader;
    }

  private:
    QString m_title;
    QList<SceneParagraph> m_paragraphs;
    ParagraphsLoader m_paragraphsLoader;
    bool m_paragraphsLoaded = false;
};

inline bool operator==(const Scene &lhs, const Scene &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_title == rhs.m_title && lhs.m_paragraphs == rhs.m_paragraphs;
}

inline uint qHash(const Scene &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_title, seed);
    hash ^= ::qHash(entity.m_paragraphs, seed);

    return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Scene)
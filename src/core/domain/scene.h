// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "scene_paragraph.h"
#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Scene : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    Q_PROPERTY(QString label READ label WRITE setLabel)

    Q_PROPERTY(QList<SceneParagraph> paragraphs READ paragraphs WRITE setParagraphs)

  public:
    struct MetaData
    {
        MetaData(Scene *entity) : m_entity(entity)
        {
        }
        MetaData(Scene *entity, const MetaData &other) : m_entity(entity)
        {
            this->paragraphsSet = other.paragraphsSet;
            this->paragraphsLoaded = other.paragraphsLoaded;
        }

        bool paragraphsSet = false;
        bool paragraphsLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "title")
            {
                return true;
            }
            if (fieldName == "label")
            {
                return true;
            }
            if (fieldName == "paragraphs")
            {
                return paragraphsSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "title")
            {
                return true;
            }
            if (fieldName == "label")
            {
                return true;
            }
            if (fieldName == "paragraphs")
            {
                return paragraphsLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Scene *m_entity = nullptr;
    };

    Scene() : Entity(), m_title(QString()), m_label(QString()), m_metaData(this)
    {
    }

    ~Scene()
    {
    }

    Scene(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
          const QString &title, const QString &label, const QList<SceneParagraph> &paragraphs)
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_label(label), m_paragraphs(paragraphs),
          m_metaData(this)
    {
    }

    Scene(const Scene &other)
        : Entity(other), m_metaData(other.m_metaData), m_title(other.m_title), m_label(other.m_label),
          m_paragraphs(other.m_paragraphs)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Scene;
    }

    Scene &operator=(const Scene &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_label = other.m_label;
            m_paragraphs = other.m_paragraphs;

            m_metaData = MetaData(this, other.metaData());
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

    // ------ label : -----

    QString label() const
    {

        return m_label;
    }

    void setLabel(const QString &label)
    {
        m_label = label;
    }

    // ------ paragraphs : -----

    QList<SceneParagraph> paragraphs()
    {
        if (!m_metaData.paragraphsLoaded && m_paragraphsLoader)
        {
            m_paragraphs = m_paragraphsLoader(this->id());
            m_metaData.paragraphsLoaded = true;
        }
        return m_paragraphs;
    }

    void setParagraphs(const QList<SceneParagraph> &paragraphs)
    {
        m_paragraphs = paragraphs;

        m_metaData.paragraphsSet = true;
    }

    using ParagraphsLoader = std::function<QList<SceneParagraph>(int entityId)>;

    void setParagraphsLoader(const ParagraphsLoader &loader)
    {
        m_paragraphsLoader = loader;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_title;
    QString m_label;
    QList<SceneParagraph> m_paragraphs;
    ParagraphsLoader m_paragraphsLoader;
};

inline bool operator==(const Scene &lhs, const Scene &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_title == rhs.m_title && lhs.m_label == rhs.m_label && lhs.m_paragraphs == rhs.m_paragraphs;
}

inline uint qHash(const Scene &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_title, seed);
    hash ^= ::qHash(entity.m_label, seed);
    hash ^= ::qHash(entity.m_paragraphs, seed);

    return hash;
}

/// Schema for Scene entity
inline Qleany::Domain::EntitySchema Scene::schema = {
    Skribisto::Domain::Entities::EntityEnum::Scene,
    "Scene",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Chapter, "Chapter", Skribisto::Domain::Entities::EntityEnum::Scene,
      "Scene", "scenes", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Backward},
     {Skribisto::Domain::Entities::EntityEnum::Scene, "Scene", Skribisto::Domain::Entities::EntityEnum::SceneParagraph,
      "SceneParagraph", "paragraphs", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"title", FieldType::String, false, false},
     {"label", FieldType::String, false, false},
     {"paragraphs", FieldType::Entity, false, true}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Scene)
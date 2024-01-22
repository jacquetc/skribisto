// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "scene.h"
#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Chapter : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    Q_PROPERTY(QString label READ label WRITE setLabel)

    Q_PROPERTY(QList<Scene> scenes READ scenes WRITE setScenes)

  public:
    struct MetaData
    {
        MetaData(Chapter *entity) : m_entity(entity)
        {
        }
        MetaData(Chapter *entity, const MetaData &other) : m_entity(entity)
        {
            this->scenesSet = other.scenesSet;
            this->scenesLoaded = other.scenesLoaded;
        }

        bool scenesSet = false;
        bool scenesLoaded = false;

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
            if (fieldName == "scenes")
            {
                return scenesSet;
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
            if (fieldName == "scenes")
            {
                return scenesLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Chapter *m_entity = nullptr;
    };

    Chapter() : Entity(), m_title(QString()), m_label(QString()), m_metaData(this)
    {
    }

    ~Chapter()
    {
    }

    Chapter(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
            const QString &title, const QString &label, const QList<Scene> &scenes)
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_label(label), m_scenes(scenes), m_metaData(this)
    {
    }

    Chapter(const Chapter &other)
        : Entity(other), m_metaData(other.m_metaData), m_title(other.m_title), m_label(other.m_label),
          m_scenes(other.m_scenes)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Chapter;
    }

    Chapter &operator=(const Chapter &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_label = other.m_label;
            m_scenes = other.m_scenes;

            m_metaData = MetaData(this, other.metaData());
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

    // ------ label : -----

    QString label() const
    {

        return m_label;
    }

    void setLabel(const QString &label)
    {
        m_label = label;
    }

    // ------ scenes : -----

    QList<Scene> scenes()
    {
        if (!m_metaData.scenesLoaded && m_scenesLoader)
        {
            m_scenes = m_scenesLoader(this->id());
            m_metaData.scenesLoaded = true;
        }
        return m_scenes;
    }

    void setScenes(const QList<Scene> &scenes)
    {
        m_scenes = scenes;

        m_metaData.scenesSet = true;
    }

    using ScenesLoader = std::function<QList<Scene>(int entityId)>;

    void setScenesLoader(const ScenesLoader &loader)
    {
        m_scenesLoader = loader;
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
    QList<Scene> m_scenes;
    ScenesLoader m_scenesLoader;
};

inline bool operator==(const Chapter &lhs, const Chapter &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_title == rhs.m_title && lhs.m_label == rhs.m_label && lhs.m_scenes == rhs.m_scenes;
}

inline uint qHash(const Chapter &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_title, seed);
    hash ^= ::qHash(entity.m_label, seed);
    hash ^= ::qHash(entity.m_scenes, seed);

    return hash;
}

/// Schema for Chapter entity
inline Qleany::Domain::EntitySchema Chapter::schema = {
    Skribisto::Domain::Entities::EntityEnum::Chapter,
    "Chapter",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Book, "Book", Skribisto::Domain::Entities::EntityEnum::Chapter,
      "Chapter", "chapters", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Backward},
     {Skribisto::Domain::Entities::EntityEnum::Chapter, "Chapter", Skribisto::Domain::Entities::EntityEnum::Scene,
      "Scene", "scenes", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"title", FieldType::String, false, false},
     {"label", FieldType::String, false, false},
     {"scenes", FieldType::Entity, false, true}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Chapter)
// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class SceneParagraph : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString content READ content WRITE setContent)

    Q_PROPERTY(int wordCount READ wordCount WRITE setWordCount)

  public:
    struct MetaData
    {
        MetaData(SceneParagraph *entity) : m_entity(entity)
        {
        }
        MetaData(SceneParagraph *entity, const MetaData &other) : m_entity(entity)
        {
        }

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "content")
            {
                return true;
            }
            if (fieldName == "wordCount")
            {
                return true;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "content")
            {
                return true;
            }
            if (fieldName == "wordCount")
            {
                return true;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        SceneParagraph *m_entity = nullptr;
    };

    SceneParagraph() : Entity(), m_content(QString()), m_wordCount(0), m_metaData(this)
    {
    }

    ~SceneParagraph()
    {
    }

    SceneParagraph(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                   const QString &content, int wordCount)
        : Entity(id, uuid, creationDate, updateDate), m_content(content), m_wordCount(wordCount), m_metaData(this)
    {
    }

    SceneParagraph(const SceneParagraph &other)
        : Entity(other), m_metaData(other.m_metaData), m_content(other.m_content), m_wordCount(other.m_wordCount)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::SceneParagraph;
    }

    SceneParagraph &operator=(const SceneParagraph &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_content = other.m_content;
            m_wordCount = other.m_wordCount;

            m_metaData = MetaData(this, other.metaData());
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

    // ------ wordCount : -----

    int wordCount() const
    {

        return m_wordCount;
    }

    void setWordCount(int wordCount)
    {
        m_wordCount = wordCount;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_content;
    int m_wordCount;
};

inline bool operator==(const SceneParagraph &lhs, const SceneParagraph &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_content == rhs.m_content && lhs.m_wordCount == rhs.m_wordCount;
}

inline uint qHash(const SceneParagraph &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_content, seed);
    hash ^= ::qHash(entity.m_wordCount, seed);

    return hash;
}

/// Schema for SceneParagraph entity
inline Qleany::Domain::EntitySchema SceneParagraph::schema = {
    Skribisto::Domain::Entities::EntityEnum::SceneParagraph,
    "SceneParagraph",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Scene, "Scene", Skribisto::Domain::Entities::EntityEnum::SceneParagraph,
      "SceneParagraph", "paragraphs", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Backward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"content", FieldType::String, false, false},
     {"wordCount", FieldType::Integer, false, false}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::SceneParagraph)
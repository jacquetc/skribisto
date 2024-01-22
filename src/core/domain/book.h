// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter.h"
#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Book : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    Q_PROPERTY(QList<Chapter> chapters READ chapters WRITE setChapters)

  public:
    struct MetaData
    {
        MetaData(Book *entity) : m_entity(entity)
        {
        }
        MetaData(Book *entity, const MetaData &other) : m_entity(entity)
        {
            this->chaptersSet = other.chaptersSet;
            this->chaptersLoaded = other.chaptersLoaded;
        }

        bool chaptersSet = false;
        bool chaptersLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "title")
            {
                return true;
            }
            if (fieldName == "chapters")
            {
                return chaptersSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "title")
            {
                return true;
            }
            if (fieldName == "chapters")
            {
                return chaptersLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Book *m_entity = nullptr;
    };

    Book() : Entity(), m_title(QString()), m_metaData(this)
    {
    }

    ~Book()
    {
    }

    Book(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
         const QString &title, const QList<Chapter> &chapters)
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_chapters(chapters), m_metaData(this)
    {
    }

    Book(const Book &other)
        : Entity(other), m_metaData(other.m_metaData), m_title(other.m_title), m_chapters(other.m_chapters)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Book;
    }

    Book &operator=(const Book &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_chapters = other.m_chapters;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Book &lhs, const Book &rhs);

    friend uint qHash(const Book &entity, uint seed) noexcept;

    // ------ title : -----

    QString title() const
    {

        return m_title;
    }

    void setTitle(const QString &title)
    {
        m_title = title;
    }

    // ------ chapters : -----

    QList<Chapter> chapters()
    {
        if (!m_metaData.chaptersLoaded && m_chaptersLoader)
        {
            m_chapters = m_chaptersLoader(this->id());
            m_metaData.chaptersLoaded = true;
        }
        return m_chapters;
    }

    void setChapters(const QList<Chapter> &chapters)
    {
        m_chapters = chapters;

        m_metaData.chaptersSet = true;
    }

    using ChaptersLoader = std::function<QList<Chapter>(int entityId)>;

    void setChaptersLoader(const ChaptersLoader &loader)
    {
        m_chaptersLoader = loader;
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
    QList<Chapter> m_chapters;
    ChaptersLoader m_chaptersLoader;
};

inline bool operator==(const Book &lhs, const Book &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_title == rhs.m_title && lhs.m_chapters == rhs.m_chapters;
}

inline uint qHash(const Book &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_title, seed);
    hash ^= ::qHash(entity.m_chapters, seed);

    return hash;
}

/// Schema for Book entity
inline Qleany::Domain::EntitySchema Book::schema = {
    Skribisto::Domain::Entities::EntityEnum::Book,
    "Book",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Book, "Book", Skribisto::Domain::Entities::EntityEnum::Chapter,
      "Chapter", "chapters", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyOrdered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"title", FieldType::String, false, false},
     {"chapters", FieldType::Entity, false, true}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Book)
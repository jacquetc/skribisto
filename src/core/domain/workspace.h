// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "file.h"
#include <QString>

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Workspace : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString name READ name WRITE setName)

    Q_PROPERTY(QString checkPointHash READ checkPointHash WRITE setCheckPointHash)

    Q_PROPERTY(bool userOwned READ userOwned WRITE setUserOwned)

    Q_PROPERTY(bool isCommonWorkSpace READ isCommonWorkSpace WRITE setIsCommonWorkSpace)

    Q_PROPERTY(QList<File> files READ files WRITE setFiles)

  public:
    struct MetaData
    {
        MetaData(Workspace *entity) : m_entity(entity)
        {
        }
        MetaData(Workspace *entity, const MetaData &other) : m_entity(entity)
        {
            this->filesSet = other.filesSet;
            this->filesLoaded = other.filesLoaded;
        }

        bool filesSet = false;
        bool filesLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "name")
            {
                return true;
            }
            if (fieldName == "checkPointHash")
            {
                return true;
            }
            if (fieldName == "userOwned")
            {
                return true;
            }
            if (fieldName == "isCommonWorkSpace")
            {
                return true;
            }
            if (fieldName == "files")
            {
                return filesSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "name")
            {
                return true;
            }
            if (fieldName == "checkPointHash")
            {
                return true;
            }
            if (fieldName == "userOwned")
            {
                return true;
            }
            if (fieldName == "isCommonWorkSpace")
            {
                return true;
            }
            if (fieldName == "files")
            {
                return filesLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Workspace *m_entity = nullptr;
    };

    Workspace()
        : Entity(), m_name(QString()), m_checkPointHash(QString()), m_userOwned(false), m_isCommonWorkSpace(false),
          m_metaData(this)
    {
    }

    ~Workspace()
    {
    }

    Workspace(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
              const QString &name, const QString &checkPointHash, bool userOwned, bool isCommonWorkSpace,
              const QList<File> &files)
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_checkPointHash(checkPointHash),
          m_userOwned(userOwned), m_isCommonWorkSpace(isCommonWorkSpace), m_files(files), m_metaData(this)
    {
    }

    Workspace(const Workspace &other)
        : Entity(other), m_metaData(other.m_metaData), m_name(other.m_name), m_checkPointHash(other.m_checkPointHash),
          m_userOwned(other.m_userOwned), m_isCommonWorkSpace(other.m_isCommonWorkSpace), m_files(other.m_files)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Workspace;
    }

    Workspace &operator=(const Workspace &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_checkPointHash = other.m_checkPointHash;
            m_userOwned = other.m_userOwned;
            m_isCommonWorkSpace = other.m_isCommonWorkSpace;
            m_files = other.m_files;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Workspace &lhs, const Workspace &rhs);

    friend uint qHash(const Workspace &entity, uint seed) noexcept;

    // ------ name : -----

    QString name() const
    {

        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ checkPointHash : -----

    QString checkPointHash() const
    {

        return m_checkPointHash;
    }

    void setCheckPointHash(const QString &checkPointHash)
    {
        m_checkPointHash = checkPointHash;
    }

    // ------ userOwned : -----

    bool userOwned() const
    {

        return m_userOwned;
    }

    void setUserOwned(bool userOwned)
    {
        m_userOwned = userOwned;
    }

    // ------ isCommonWorkSpace : -----

    bool isCommonWorkSpace() const
    {

        return m_isCommonWorkSpace;
    }

    void setIsCommonWorkSpace(bool isCommonWorkSpace)
    {
        m_isCommonWorkSpace = isCommonWorkSpace;
    }

    // ------ files : -----

    QList<File> files()
    {
        if (!m_metaData.filesLoaded && m_filesLoader)
        {
            m_files = m_filesLoader(this->id());
            m_metaData.filesLoaded = true;
        }
        return m_files;
    }

    void setFiles(const QList<File> &files)
    {
        m_files = files;

        m_metaData.filesSet = true;
    }

    using FilesLoader = std::function<QList<File>(int entityId)>;

    void setFilesLoader(const FilesLoader &loader)
    {
        m_filesLoader = loader;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    QString m_name;
    QString m_checkPointHash;
    bool m_userOwned;
    bool m_isCommonWorkSpace;
    QList<File> m_files;
    FilesLoader m_filesLoader;
};

inline bool operator==(const Workspace &lhs, const Workspace &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_name == rhs.m_name && lhs.m_checkPointHash == rhs.m_checkPointHash &&
           lhs.m_userOwned == rhs.m_userOwned && lhs.m_isCommonWorkSpace == rhs.m_isCommonWorkSpace &&
           lhs.m_files == rhs.m_files;
}

inline uint qHash(const Workspace &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_name, seed);
    hash ^= ::qHash(entity.m_checkPointHash, seed);
    hash ^= ::qHash(entity.m_userOwned, seed);
    hash ^= ::qHash(entity.m_isCommonWorkSpace, seed);
    hash ^= ::qHash(entity.m_files, seed);

    return hash;
}

/// Schema for Workspace entity
inline Qleany::Domain::EntitySchema Workspace::schema = {
    Skribisto::Domain::Entities::EntityEnum::Workspace,
    "Workspace",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Atelier, "Atelier", Skribisto::Domain::Entities::EntityEnum::Workspace,
      "Workspace", "workspaces", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyUnordered, RelationshipDirection::Backward},
     {Skribisto::Domain::Entities::EntityEnum::Workspace, "Workspace", Skribisto::Domain::Entities::EntityEnum::File,
      "File", "files", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyUnordered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"name", FieldType::String, false, false},
     {"checkPointHash", FieldType::String, false, false},
     {"userOwned", FieldType::Bool, false, false},
     {"isCommonWorkSpace", FieldType::Bool, false, false},
     {"files", FieldType::Entity, false, true}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Workspace)
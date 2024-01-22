// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "git.h"
#include "workspace.h"

#include "entities.h"
#include "entity.h"
#include <qleany/domain/entity_schema.h>

using namespace Qleany::Domain;

namespace Skribisto::Domain
{

class Atelier : public Entity
{
    Q_GADGET

    Q_PROPERTY(bool path READ path WRITE setPath)

    Q_PROPERTY(Git git READ git WRITE setGit)

    Q_PROPERTY(QList<Workspace> workspaces READ workspaces WRITE setWorkspaces)

  public:
    struct MetaData
    {
        MetaData(Atelier *entity) : m_entity(entity)
        {
        }
        MetaData(Atelier *entity, const MetaData &other) : m_entity(entity)
        {
            this->gitSet = other.gitSet;
            this->gitLoaded = other.gitLoaded;
            this->workspacesSet = other.workspacesSet;
            this->workspacesLoaded = other.workspacesLoaded;
        }

        bool gitSet = false;
        bool gitLoaded = false;

        bool workspacesSet = false;
        bool workspacesLoaded = false;

        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "git")
            {
                return gitSet;
            }
            if (fieldName == "workspaces")
            {
                return workspacesSet;
            }
            return m_entity->Entity::metaData().getSet(fieldName);
        }

        bool getLoaded(const QString &fieldName) const
        {

            if (fieldName == "path")
            {
                return true;
            }
            if (fieldName == "git")
            {
                return gitLoaded;
            }
            if (fieldName == "workspaces")
            {
                return workspacesLoaded;
            }
            return m_entity->Entity::metaData().getLoaded(fieldName);
        }

      private:
        Atelier *m_entity = nullptr;
    };

    Atelier() : Entity(), m_path(false), m_metaData(this)
    {
    }

    ~Atelier()
    {
    }

    Atelier(const int &id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate, bool path,
            const Git &git, const QList<Workspace> &workspaces)
        : Entity(id, uuid, creationDate, updateDate), m_path(path), m_git(git), m_workspaces(workspaces),
          m_metaData(this)
    {
    }

    Atelier(const Atelier &other)
        : Entity(other), m_metaData(other.m_metaData), m_path(other.m_path), m_git(other.m_git),
          m_workspaces(other.m_workspaces)
    {
        m_metaData = MetaData(this, other.metaData());
    }

    static Skribisto::Domain::Entities::EntityEnum enumValue()
    {
        return Skribisto::Domain::Entities::EntityEnum::Atelier;
    }

    Atelier &operator=(const Atelier &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_path = other.m_path;
            m_git = other.m_git;
            m_workspaces = other.m_workspaces;

            m_metaData = MetaData(this, other.metaData());
        }
        return *this;
    }

    friend bool operator==(const Atelier &lhs, const Atelier &rhs);

    friend uint qHash(const Atelier &entity, uint seed) noexcept;

    // ------ path : -----

    bool path() const
    {

        return m_path;
    }

    void setPath(bool path)
    {
        m_path = path;
    }

    // ------ git : -----

    Git git()
    {
        if (!m_metaData.gitLoaded && m_gitLoader)
        {
            m_git = m_gitLoader(this->id());
            m_metaData.gitLoaded = true;
        }
        return m_git;
    }

    void setGit(const Git &git)
    {
        m_git = git;

        m_metaData.gitSet = true;
    }

    using GitLoader = std::function<Git(int entityId)>;

    void setGitLoader(const GitLoader &loader)
    {
        m_gitLoader = loader;
    }

    // ------ workspaces : -----

    QList<Workspace> workspaces()
    {
        if (!m_metaData.workspacesLoaded && m_workspacesLoader)
        {
            m_workspaces = m_workspacesLoader(this->id());
            m_metaData.workspacesLoaded = true;
        }
        return m_workspaces;
    }

    void setWorkspaces(const QList<Workspace> &workspaces)
    {
        m_workspaces = workspaces;

        m_metaData.workspacesSet = true;
    }

    using WorkspacesLoader = std::function<QList<Workspace>(int entityId)>;

    void setWorkspacesLoader(const WorkspacesLoader &loader)
    {
        m_workspacesLoader = loader;
    }

    static Qleany::Domain::EntitySchema schema;

    MetaData metaData() const
    {
        return m_metaData;
    }

  protected:
    MetaData m_metaData;

  private:
    bool m_path;
    Git m_git;
    GitLoader m_gitLoader;
    QList<Workspace> m_workspaces;
    WorkspacesLoader m_workspacesLoader;
};

inline bool operator==(const Atelier &lhs, const Atelier &rhs)
{

    return static_cast<const Entity &>(lhs) == static_cast<const Entity &>(rhs) &&

           lhs.m_path == rhs.m_path && lhs.m_git == rhs.m_git && lhs.m_workspaces == rhs.m_workspaces;
}

inline uint qHash(const Atelier &entity, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;
    hash ^= qHash(static_cast<const Entity &>(entity), seed);

    // Combine with this class's properties
    hash ^= ::qHash(entity.m_path, seed);
    hash ^= ::qHash(entity.m_git, seed);
    hash ^= ::qHash(entity.m_workspaces, seed);

    return hash;
}

/// Schema for Atelier entity
inline Qleany::Domain::EntitySchema Atelier::schema = {
    Skribisto::Domain::Entities::EntityEnum::Atelier,
    "Atelier",

    // relationships:
    {{Skribisto::Domain::Entities::EntityEnum::Atelier, "Atelier", Skribisto::Domain::Entities::EntityEnum::Git, "Git",
      "git", RelationshipType::OneToOne, RelationshipStrength::Strong, RelationshipCardinality::One,
      RelationshipDirection::Forward},
     {Skribisto::Domain::Entities::EntityEnum::Atelier, "Atelier", Skribisto::Domain::Entities::EntityEnum::Workspace,
      "Workspace", "workspaces", RelationshipType::OneToMany, RelationshipStrength::Strong,
      RelationshipCardinality::ManyUnordered, RelationshipDirection::Forward}},

    // fields:
    {{"id", FieldType::Integer, true, false},
     {"uuid", FieldType::Uuid, false, false},
     {"creationDate", FieldType::DateTime, false, false},
     {"updateDate", FieldType::DateTime, false, false},
     {"path", FieldType::Bool, false, false},
     {"git", FieldType::Entity, false, true},
     {"workspaces", FieldType::Entity, false, true}}};

} // namespace Skribisto::Domain
Q_DECLARE_METATYPE(Skribisto::Domain::Atelier)
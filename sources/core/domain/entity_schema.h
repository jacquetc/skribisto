#pragma once

#include "QtCore/qdebug.h"
#include "entities.h"
#include <QObject>

namespace Domain
{

class EntitySchema : public QObject
{

    Q_OBJECT
  public:
    enum Relationship
    {
        OneToOne,
        OneToMany,
    };
    Q_ENUM(Relationship)

    enum Dependency
    {
        Strong,
        Weak,
    };
    Q_ENUM(Dependency)

    struct EntitySchemaRelationship
    {
        int entity;
        int otherEntity;
        QString propertyName;
        Relationship relationship;
        Dependency dependency;
    };

    explicit EntitySchema(QObject *parent = nullptr);

    void addRelationship(int entity, int otherEntity, const QString &propertyName, Relationship relationship,
                         Dependency dependency)
    {
        EntitySchemaRelationship relationshipSchema;
        relationshipSchema.entity = entity;
        relationshipSchema.otherEntity = otherEntity;
        relationshipSchema.propertyName = propertyName;
        relationshipSchema.relationship = relationship;
        relationshipSchema.dependency = dependency;
        m_relationships.append(relationshipSchema);
    }

    EntitySchemaRelationship relationship(int entity, const QString &propertyName) const
    {
        for (auto relationship : m_relationships)
        {
            if (relationship.entity == entity && relationship.propertyName == propertyName)
            {
                return relationship;
            }
        }

        qCritical() << "Relationship not found for entity" << entity << "and property" << propertyName;
        return EntitySchemaRelationship();
    }

  signals:

  private:
    QList<EntitySchemaRelationship> m_relationships;
};
} // namespace Domain

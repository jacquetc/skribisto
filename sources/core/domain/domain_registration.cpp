#include "domain_registration.h"

using namespace Domain;

DomainRegistration::DomainRegistration(QObject *parent) : QObject{parent}
{

    m_entitySchema = new EntitySchema{this};
    m_entitySchema->addRelationship(Entities::Atelier, Entities::Book, "books", EntitySchema::OneToMany,
                                    EntitySchema::Strong);
    m_entitySchema->addRelationship(Entities::Book, Entities::Author, "author", EntitySchema::OneToOne,
                                    EntitySchema::Strong);
    m_entitySchema->addRelationship(Entities::Book, Entities::Chapter, "chapters", EntitySchema::OneToMany,
                                    EntitySchema::Strong);
    m_entitySchema->addRelationship(Entities::Chapter, Entities::Scene, "scenes", EntitySchema::OneToMany,
                                    EntitySchema::Strong);
    m_entitySchema->addRelationship(Entities::Scene, Entities::SceneParagraph, "paragraphs", EntitySchema::OneToMany,
                                    EntitySchema::Strong);
}

EntitySchema *DomainRegistration::entitySchema() const
{
    return m_entitySchema;
}

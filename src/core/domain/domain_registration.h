// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "atelier.h"
#include "book.h"
#include "chapter.h"
#include "entity.h"
#include "file.h"
#include "git.h"
#include "recent_atelier.h"
#include "scene.h"
#include "scene_paragraph.h"
#include "user.h"
#include "workspace.h"

#include <QObject>

namespace Skribisto::Domain
{

class DomainRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit DomainRegistration(QObject *parent) : QObject(parent)
    {

        qRegisterMetaType<Skribisto::Domain::Entity>("Skribisto::Domain::Entity");
        qRegisterMetaType<Skribisto::Domain::RecentAtelier>("Skribisto::Domain::RecentAtelier");
        qRegisterMetaType<Skribisto::Domain::User>("Skribisto::Domain::User");
        qRegisterMetaType<Skribisto::Domain::Atelier>("Skribisto::Domain::Atelier");
        qRegisterMetaType<Skribisto::Domain::Git>("Skribisto::Domain::Git");
        qRegisterMetaType<Skribisto::Domain::Workspace>("Skribisto::Domain::Workspace");
        qRegisterMetaType<Skribisto::Domain::File>("Skribisto::Domain::File");
        qRegisterMetaType<Skribisto::Domain::Book>("Skribisto::Domain::Book");
        qRegisterMetaType<Skribisto::Domain::Chapter>("Skribisto::Domain::Chapter");
        qRegisterMetaType<Skribisto::Domain::Scene>("Skribisto::Domain::Scene");
        qRegisterMetaType<Skribisto::Domain::SceneParagraph>("Skribisto::Domain::SceneParagraph");
    }
};

} // namespace Skribisto::Domain
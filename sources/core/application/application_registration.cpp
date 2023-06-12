#include "application_registration.h"
#include "author.h"
#include "chapter.h"
#include "chapter_dto.h"
#include "create_author_dto.h"
#include "author_dto.h"
#include "automapper/automapper.h"
#include "create_chapter_dto.h"
#include "create_scene_dto.h"
#include "scene_dto.h"
#include "scene.h"


using namespace Application;

ApplicationRegistration::ApplicationRegistration(QObject *parent) : QObject{parent}
{
    AutoMapper::AutoMapper::registerMapping<Domain::Author, Contracts::DTO::Author::AuthorDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Author::CreateAuthorDTO, Domain::Author>();

    // Scene
    AutoMapper::AutoMapper::registerMapping<Domain::Scene, Contracts::DTO::Scene::SceneDTO>(true);
    AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Scene::CreateSceneDTO, Domain::Scene>();

    // Chapter
    //    AutoMapper::AutoMapper::registerMapping<Domain::Chapter,
    // Contracts::DTO::Chapter::ChapterDTO>(true);
    //
    //  AutoMapper::AutoMapper::registerMapping<Contracts::DTO::Chapter::CreateChapterDTO,
    // Domain::Chapter>();
}

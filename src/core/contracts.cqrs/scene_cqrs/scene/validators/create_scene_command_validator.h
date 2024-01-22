// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene/create_scene_dto.h"


#include "repository/interface_scene_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::Scene;

namespace Skribisto::Contracts::CQRS::Scene::Validators
{
class CreateSceneCommandValidator
{
  public:
    CreateSceneCommandValidator(InterfaceSceneRepository *sceneRepository)
        :  m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(const CreateSceneDTO &dto) const

    {





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceSceneRepository *m_sceneRepository;

};
} // namespace Skribisto::Contracts::CQRS::Scene::Validators
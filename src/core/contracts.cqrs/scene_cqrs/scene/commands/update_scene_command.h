// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene/update_scene_dto.h"


namespace Skribisto::Contracts::CQRS::Scene::Commands
{
class UpdateSceneCommand
{
  public:
    UpdateSceneCommand()
    {
    }



    Skribisto::Contracts::DTO::Scene::UpdateSceneDTO req;


};
} // namespace Skribisto::Contracts::CQRS::Scene::Commands
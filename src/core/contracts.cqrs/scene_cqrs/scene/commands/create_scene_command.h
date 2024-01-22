// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene/create_scene_dto.h"


namespace Skribisto::Contracts::CQRS::Scene::Commands
{
class CreateSceneCommand
{
  public:
    CreateSceneCommand()
    {
    }



    Skribisto::Contracts::DTO::Scene::CreateSceneDTO req;


};
} // namespace Skribisto::Contracts::CQRS::Scene::Commands
// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene_paragraph/create_scene_paragraph_dto.h"


namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands
{
class CreateSceneParagraphCommand
{
  public:
    CreateSceneParagraphCommand()
    {
    }



    Skribisto::Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO req;


};
} // namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands
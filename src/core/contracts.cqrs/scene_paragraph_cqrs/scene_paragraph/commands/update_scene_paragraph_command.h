// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene_paragraph/update_scene_paragraph_dto.h"


namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands
{
class UpdateSceneParagraphCommand
{
  public:
    UpdateSceneParagraphCommand()
    {
    }



    Skribisto::Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO req;


};
} // namespace Skribisto::Contracts::CQRS::SceneParagraph::Commands
// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "chapter/update_chapter_dto.h"


namespace Skribisto::Contracts::CQRS::Chapter::Commands
{
class UpdateChapterCommand
{
  public:
    UpdateChapterCommand()
    {
    }



    Skribisto::Contracts::DTO::Chapter::UpdateChapterDTO req;


};
} // namespace Skribisto::Contracts::CQRS::Chapter::Commands
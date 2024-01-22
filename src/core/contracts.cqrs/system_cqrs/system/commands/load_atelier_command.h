// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "system/load_atelier_dto.h"


namespace Skribisto::Contracts::CQRS::System::Commands
{
class LoadAtelierCommand
{
  public:
    LoadAtelierCommand()
    {
    }



    Skribisto::Contracts::DTO::System::LoadAtelierDTO req;


};
} // namespace Skribisto::Contracts::CQRS::System::Commands
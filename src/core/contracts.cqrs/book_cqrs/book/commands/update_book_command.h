// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "book/update_book_dto.h"


namespace Skribisto::Contracts::CQRS::Book::Commands
{
class UpdateBookCommand
{
  public:
    UpdateBookCommand()
    {
    }



    Skribisto::Contracts::DTO::Book::UpdateBookDTO req;


};
} // namespace Skribisto::Contracts::CQRS::Book::Commands
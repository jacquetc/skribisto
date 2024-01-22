// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once



#include "repository/interface_chapter_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;



namespace Skribisto::Contracts::CQRS::Chapter::Validators
{
class RemoveChapterCommandValidator
{
  public:
    RemoveChapterCommandValidator(InterfaceChapterRepository *chapterRepository)
        :  m_chapterRepository(chapterRepository)
    {
    }

    Result<void> validate(int id) const

    {




        Result<bool> existsResult = m_chapterRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceChapterRepository *m_chapterRepository;

};
} // namespace Skribisto::Contracts::CQRS::Chapter::Validators
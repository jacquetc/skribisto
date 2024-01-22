// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once


#include "scene_paragraph/update_scene_paragraph_dto.h"


#include "repository/interface_scene_paragraph_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;

using namespace Skribisto::Contracts::DTO::SceneParagraph;

namespace Skribisto::Contracts::CQRS::SceneParagraph::Validators
{
class UpdateSceneParagraphCommandValidator
{
  public:
    UpdateSceneParagraphCommandValidator(InterfaceSceneParagraphRepository *sceneParagraphRepository)
        :  m_sceneParagraphRepository(sceneParagraphRepository)
    {
    }

    Result<void> validate(const UpdateSceneParagraphDTO &dto) const

    {




        Result<bool> existsResult = m_sceneParagraphRepository->exists(dto.id());

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceSceneParagraphRepository *m_sceneParagraphRepository;

};
} // namespace Skribisto::Contracts::CQRS::SceneParagraph::Validators
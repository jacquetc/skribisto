// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once



#include "repository/interface_scene_repository.h"

#include <qleany/common/result.h>

using namespace Qleany;

using namespace Skribisto::Contracts::Repository;



namespace Skribisto::Contracts::CQRS::Scene::Validators
{
class RemoveSceneCommandValidator
{
  public:
    RemoveSceneCommandValidator(InterfaceSceneRepository *sceneRepository)
        :  m_sceneRepository(sceneRepository)
    {
    }

    Result<void> validate(int id) const

    {




        Result<bool> existsResult = m_sceneRepository->exists(id);

        if (!existsResult.value())
        {
            return Result<void>(QLN_ERROR_1(Q_FUNC_INFO, Error::Critical, "id_not_found"));
        }





        // Return that is Ok :
        return Result<void>();
    }

  private:

    InterfaceSceneRepository *m_sceneRepository;

};
} // namespace Skribisto::Contracts::CQRS::Scene::Validators
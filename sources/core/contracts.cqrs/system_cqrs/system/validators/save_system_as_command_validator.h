#pragma once

#include "system/save_system_as_dto.h"

#include "result.h"

#include <QFile>

using namespace Contracts::DTO::System;

namespace Contracts::CQRS::System::Validators
{
class SaveSystemAsCommandValidator
{
  public:
    SaveSystemAsCommandValidator()

    {
    }

    Result<void> validate(const SaveSystemAsDTO &dto) const

    {

        QUrl url = dto.fileName();

        if (!url.isValid())
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "invalid_filename"));
        }

        if (!url.isLocalFile() && url.scheme() != "qrc")
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "invalid_not_local"));
        }

        QString fileNameString;

        if (url.scheme() == "qrc")
        {
            fileNameString = url.toString(QUrl::RemoveScheme);
            fileNameString = ":" + fileNameString;
        }
        else if (url.path().at(2) == ':')
        { // means Windows
            fileNameString = url.path().remove(0, 1);
        }
        else
        {
            fileNameString = url.path();
        }

        QFile file(fileNameString);

        if (!file.exists())
        {

            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "absent_filename",
                                      fileNameString + " doesn't exist", fileNameString));
        }

        if (!file.open(QIODevice::ReadOnly))
        {
            return Result<void>(Error(Q_FUNC_INFO, Error::Critical, "readonly_filename",
                                      fileNameString + " can't be opened", fileNameString));
        }

        // Return that is Ok :
        return Result<void>();
    }

  private:
};
} // namespace Contracts::CQRS::System::Validators

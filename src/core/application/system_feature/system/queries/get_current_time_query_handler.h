// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "application_system_export.h"
#include "system/get_current_time_reply_dto.h"
#include "system/queries/get_current_time_query.h"

#include <QPromise>
#include <qleany/common/result.h>

using namespace Qleany;
using namespace Skribisto::Contracts::DTO::System;

using namespace Skribisto::Contracts::CQRS::System::Queries;

namespace Skribisto::Application::Features::System::Queries
{
class SKRIBISTO_APPLICATION_SYSTEM_EXPORT GetCurrentTimeQueryHandler : public QObject
{
    Q_OBJECT
  public:
    GetCurrentTimeQueryHandler();

    Result<GetCurrentTimeReplyDTO> handle(QPromise<Result<void>> &progressPromise, const GetCurrentTimeQuery &request);

  signals:
    void getCurrentTimeChanged(Skribisto::Contracts::DTO::System::GetCurrentTimeReplyDTO getCurrentTimeReplyDTO);

  private:
    Result<GetCurrentTimeReplyDTO> handleImpl(QPromise<Result<void>> &progressPromise,
                                              const GetCurrentTimeQuery &request);
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace Skribisto::Application::Features::System::Queries
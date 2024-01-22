// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "controller_export.h"

#include "system/get_current_time_reply_dto.h"

#include <QObject>

namespace Skribisto::Controller
{

using namespace Skribisto::Contracts::DTO::System;

class SKRIBISTO_CONTROLLER_EXPORT SystemSignals : public QObject
{
    Q_OBJECT
  public:
    explicit SystemSignals(QObject *parent = nullptr) : QObject{parent}
    {
    }

  signals:
    void createNewAtelierChanged();
    void loadAtelierChanged();
    void saveAtelierChanged();
    void closeAtelierChanged();
    void getCurrentTimeReplied(GetCurrentTimeReplyDTO dto);
};
} // namespace Skribisto::Controller
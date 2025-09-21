#pragma once
#include "single_passenger.h"
#include <QQmlEngine>

struct ForeignSinglePassenger {
    Q_GADGET
    QML_FOREIGN(FrontEnds::Presenter::SinglePassenger)
    QML_NAMED_ELEMENT(SinglePassenger)
};
#pragma once
#include "single_car.h"
#include <QQmlEngine>

struct ForeignSingleCar {
    Q_GADGET
    QML_FOREIGN(FrontEnds::Presenter::SingleCar)
    QML_NAMED_ELEMENT(SingleCar)
};
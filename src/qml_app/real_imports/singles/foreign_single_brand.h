#pragma once
#include "single_brand.h"
#include <QQmlEngine>

struct ForeignSingleBrand {
    Q_GADGET
    QML_FOREIGN(FrontEnds::Presenter::SingleBrand)
    QML_NAMED_ELEMENT(SingleBrand)
};
#pragma once
#include "single_client.h"
#include <QQmlEngine>

struct ForeignSingleClient {
    Q_GADGET
    QML_FOREIGN(FrontEnds::Presenter::SingleClient)
    QML_NAMED_ELEMENT(SingleClient)
};
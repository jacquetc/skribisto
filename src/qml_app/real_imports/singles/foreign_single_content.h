#pragma once
#include "content/single_content.h"
#include <QQmlEngine>

struct ForeignSingleContent
{
    Q_GADGET
    QML_FOREIGN(Skribisto::DirectAccess::Content::SingleContent)
    QML_NAMED_ELEMENT(SingleContent)
};
#pragma once
#include "binder_item/single_binder_item.h"
#include <QQmlEngine>

struct ForeignSingleContent
{
    Q_GADGET
    QML_FOREIGN(Skribisto::DirectAccess::BinderItem::SingleBinderItem)
    QML_NAMED_ELEMENT(SingleBinderItem)
};
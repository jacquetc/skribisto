#include "plmsheetlistmodel.h"
#include "skr.h"

PLMSheetListModel::PLMSheetListModel(QObject *parent)
    : SKRPaperListModel(parent, SKR::Sheet)
{}

#include "skrsearchsheetlistproxymodel.h"
#include "plmmodels.h"
#include "skr.h"


SKRSearchSheetListProxyModel::SKRSearchSheetListProxyModel() :
    SKRSearchPaperListProxyModel(SKR::Sheet)
{}

SKRSearchSheetListProxyModel *SKRSearchSheetListProxyModel::clone()
{
    return static_cast<SKRSearchSheetListProxyModel *>(SKRSearchPaperListProxyModel::clone()) ;
}

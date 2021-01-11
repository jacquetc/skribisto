#include "skrsearchnotelistproxymodel.h"
#include "plmmodels.h"
#include "skr.h"


SKRSearchNoteListProxyModel::SKRSearchNoteListProxyModel() :
    SKRSearchPaperListProxyModel(SKR::Note)
{}

SKRSearchNoteListProxyModel *SKRSearchNoteListProxyModel::clone()
{
    return static_cast<SKRSearchNoteListProxyModel *>(SKRSearchPaperListProxyModel::clone()) ;
}

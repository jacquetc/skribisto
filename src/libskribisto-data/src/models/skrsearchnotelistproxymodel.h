#ifndef SKRSEARCHNOTELISTPROXYMODEL_H
#define SKRSEARCHNOTELISTPROXYMODEL_H


#include "skrsearchpaperlistproxymodel.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchNoteListProxyModel : public SKRSearchPaperListProxyModel {
    Q_OBJECT

public:

    explicit SKRSearchNoteListProxyModel();

    Q_INVOKABLE SKRSearchNoteListProxyModel *clone();

};

#endif // SKRSEARCHNOTELISTPROXYMODEL_H

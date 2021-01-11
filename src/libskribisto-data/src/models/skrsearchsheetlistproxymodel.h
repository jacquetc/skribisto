#ifndef PLMSEARCHSHEETLISTPROXYMODEL_H
#define PLMSEARCHSHEETLISTPROXYMODEL_H


#include "skrsearchpaperlistproxymodel.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchSheetListProxyModel : public SKRSearchPaperListProxyModel {
    Q_OBJECT

public:

    explicit SKRSearchSheetListProxyModel();

    Q_INVOKABLE SKRSearchSheetListProxyModel *clone();
};

#endif // PLMSEARCHSHEETLISTPROXYMODEL_H

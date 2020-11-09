#ifndef PLMSHEETLISTMODEL_H
#define PLMSHEETLISTMODEL_H

#include "skrpaperlistmodel.h"
#include "./skribisto_data_global.h"


class EXPORT PLMSheetListModel : public SKRPaperListModel {
    Q_OBJECT

public:

    explicit PLMSheetListModel(QObject *parent);
};


#endif // PLMSHEETLISTMODEL_H

#ifndef PLMNOTELISTMODEL_H
#define PLMNOTELISTMODEL_H

#include "skrpaperlistmodel.h"
#include "./skribisto_data_global.h"


class EXPORT PLMNoteListModel : public SKRPaperListModel {
    Q_OBJECT

public:

    explicit PLMNoteListModel(QObject *parent);
};

#endif // PLMNOTELISTMODEL_H

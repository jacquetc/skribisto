#ifndef PLMWRITEDOCUMENTLISTMODEL_H
#define PLMWRITEDOCUMENTLISTMODEL_H

#include "plmdocumentlistmodel.h"
#include "./skribisto_data_global.h"


class EXPORT PLMWriteDocumentListModel : public PLMDocumentListModel {
    Q_OBJECT

public:

    PLMWriteDocumentListModel(QObject *parent = nullptr);
    Q_INVOKABLE QVariant getDocumentData(int                              projectId,
                                         int                              paperId,
                                         int                              subWindowId,
                                         PLMWriteDocumentListModel::Roles role) const;
    Q_INVOKABLE int closeDocument(int projectId,
                                  int paperId,
                                  int subWindowId);

private:

    QString m_tableName;
};

#endif // PLMWRITEDOCUMENTLISTMODEL_H

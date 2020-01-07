#ifndef PLMWRITEDOCUMENTLISTMODEL_H
#define PLMWRITEDOCUMENTLISTMODEL_H

#include "plmdocumentlistmodel.h"
#include "./skribisto_data_global.h"



class EXPORT PLMWriteDocumentListModel : public PLMDocumentListModel
{
    Q_OBJECT
public:
    PLMWriteDocumentListModel(QObject *parent = nullptr);

private:
    QString m_tableName;
};

#endif // PLMWRITEDOCUMENTLISTMODEL_H

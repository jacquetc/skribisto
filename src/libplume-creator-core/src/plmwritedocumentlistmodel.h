#ifndef PLMWRITEDOCUMENTLISTMODEL_H
#define PLMWRITEDOCUMENTLISTMODEL_H

#include "plmdocumentlistmodel.h"



class PLMWriteDocumentListModel : public PLMDocumentListModel
{
public:
    PLMWriteDocumentListModel(QObject *parent);

private:
    QString m_tableName;
};

#endif // PLMWRITEDOCUMENTLISTMODEL_H

#include "plmwritedocumentlistmodel.h"

#include <plmsheethub.h>
#include <plmdata.h>

PLMWriteDocumentListModel::PLMWriteDocumentListModel(QObject *parent):
    PLMDocumentListModel(parent, "tbl_user_writewindow_doc_list"), m_tableName("tbl_user_writewindow_doc_list")
{

    connect(plmdata->sheetHub(),
            &PLMSheetHub::titleChanged,
            [=](int            projectId,
            int            paperId,
            const QString& newTitle){
        QList<int> documentIds = this->getDocumentIdEverywhere(projectId, paperId);
        for(int docId : documentIds){
            plmdata->userHub()->set(projectId, m_tableName, docId, "t_title", newTitle, true);
        }

    }
    );


}

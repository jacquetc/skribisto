#include "plmwritedocumentlistmodel.h"

#include <plmsheethub.h>
#include <plmdata.h>

PLMWriteDocumentListModel::PLMWriteDocumentListModel(QObject *parent) :
    PLMDocumentListModel(parent, "tbl_user_writewindow_doc_list"), m_tableName(
        "tbl_user_writewindow_doc_list")
{
    connect(plmdata->sheetHub(),
            &PLMSheetHub::titleChanged,
            [ = ](int            projectId,
                  int            paperId,
                  const QString& newTitle) {
        QList<int>documentIds = this->getDocumentIdEverywhere(projectId, paperId);

        //        for(int docId : documentIds){
        //            plmdata->userHub()->set(projectId, m_tableName, docId,
        // "t_title", newTitle, true);
        //        }
    }
            );
}

QVariant PLMWriteDocumentListModel::getDocumentData(int                         projectId,
                                                    int                         paperId,
                                                    int                         subWindowId,
                                                    PLMDocumentListModel::Roles role)
const
{
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_paper_code =", paperId);
    where.insert("l_subWindow =",  subWindowId);

    QString roleString = this->translateRole(role);

    //    hash = plmdata->userHub()->getValueByIdsWhere(projectId,
    // m_tableName, roleString, where);

    if (hash.isEmpty()) { // create a new document
        return QVariant();
    }

    return hash.values().first();
}

int PLMWriteDocumentListModel::closeDocument(int projectId, int paperId, int subWindowId)
{
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_paper_code =", paperId);
    where.insert("l_subWindow =",  subWindowId);

    QString roleString = this->translateRole(PLMDocumentListModel::Roles::PropertyRole);

    //    hash = plmdata->userHub()->getValueByIdsWhere(projectId,
    // m_tableName, roleString, where);

    int documentId = hash.keys().first();


    if (hash.isEmpty()) {
        return -2;
    }

    return documentId;
}

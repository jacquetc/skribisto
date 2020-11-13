#include "plmpaperhub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"
#include "tasks/plmprojectmanager.h"
#include "plmdata.h"

#include <QDateTime>
#include <QDebug>

///
/// \brief PLMPaperHub::PLMPaperHub
/// \param parent
//
PLMPaperHub::PLMPaperHub(QObject *parent, const QString& tableName, const SKR::PaperType &paperType)
    : QObject(parent), m_tableName(tableName), m_paperType(paperType), m_last_added_id(-1), m_wordMeter(new SKRWordMeter(this))
{
    qRegisterMetaType<SKR::PaperType>("SKR::PaperType");
    //    qRegisterMetaType<Setting>(         "Setting");
    //    qRegisterMetaType<Stack>(           "Stack");
    //    qRegisterMetaType<OpenedDocSetting>("OpenedDocSetting");

    // connection for 'getxxx' functions to have a way to get errors.
    connect(this, &PLMPaperHub::errorSent, this, &PLMPaperHub::setError, Qt::DirectConnection);

    //connect(m_wordMeter, )


    connect(m_wordMeter, &SKRWordMeter::wordCountCalculated, [this](SKR::PaperType paperType, int projectId, int paperId, int wordCount){
        switch(paperType){
        case SKR::Sheet:
            plmdata->sheetPropertyHub()->setProperty(projectId, paperId, "word_count", QString::number(wordCount));
            emit wordCountChanged(paperType, projectId, paperId, wordCount);
            break;
        case SKR::Note:
            plmdata->notePropertyHub()->setProperty(projectId, paperId, "word_count", QString::number(wordCount));
            emit wordCountChanged(paperType, projectId, paperId, wordCount);
            break;
        }
    });

    connect(m_wordMeter, &SKRWordMeter::characterCountCalculated, [this](SKR::PaperType paperType, int projectId, int paperId, int characterCount){
        switch(paperType){
        case SKR::Sheet:
            plmdata->sheetPropertyHub()->setProperty(projectId, paperId, "char_count", QString::number(characterCount));
            emit characterCountChanged(paperType, projectId, paperId, characterCount);
            break;
        case SKR::Note:
            plmdata->notePropertyHub()->setProperty(projectId, paperId, "char_count", QString::number(characterCount));
            emit characterCountChanged(paperType, projectId, paperId, characterCount);
            break;
        }
    });

    connect(plmdata->projectHub(), &PLMProjectHub::projectLoaded, [this](int projectId){
        for(int paperId : this->getAllIds(projectId)){
            QString content = this->getContent(projectId, paperId);
            m_wordMeter->countText(m_paperType, projectId, paperId, content, false);
        }

    });




}

///
/// \brief PLMPaperHub::getAll
/// \param projectId
/// \return
/// unimplemented
QList<QHash<QString, QVariant> >PLMPaperHub::getAll(int projectId) const
{
    Q_UNUSED(projectId)

    QList<QHash<QString, QVariant> >hash;

    return hash;
}

///
/// \brief PLMPaperHub::getAllTitles
/// \param projectId
/// \return
///
QHash<int, QString>PLMPaperHub::getAllTitles(int projectId) const
{
    SKRResult result(this);

    QHash<int, QString>  hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("t_title", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntQString(out);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// ------------------------------------------------------------

///
/// \brief PLMPaperHub::getAllIndents
/// \param projectId
/// \return
///
// ------------------------------------------------------------

QHash<int, int>PLMPaperHub::getAllIndents(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_indent", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// ------------------------------------------------------------

/**
 * @brief PLMPaperHub::getAllIds
 * @param projectId
 * @return
 * Get sorted ids, trashed ids included
 */
QList<int>PLMPaperHub::getAllIds(int projectId) const
{
    SKRResult result(this);

    QList<int> list;
    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getSortedIds(out);
    IFOK(result) {
        list = out;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return list;
}

// ------------------------------------------------------------
///
/// \brief PLMPaperHub::getOverallSize
/// \return number of papers for all projects
///
int PLMPaperHub::getOverallSize()
{
    SKRResult result(this);

    QList<int> list;

    for (int projectId : plmProjectManager->projectIdList()) {
        PLMSqlQueries queries(projectId, m_tableName);

        result = queries.getIds(list);
        IFOK(result) {
            return list.size();
        }
        IFKO(result) {
            emit errorSent(result);

            return 0;
        }
    }
    return 0;
}

// ------------------------------------------------------------

QHash<int, int>PLMPaperHub::getAllSortOrders(int projectId) const
{
    SKRResult result(this);

    QHash<int, int> hash;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.getValueByIds("l_sort_order", out, "", QVariant(), true);
    IFOK(result) {
        hash = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return hash;
}

// ------------------------------------------------------------
///
/// \brief PLMPaperHub::setId
/// \param projectId
/// \param paperId
/// \param newId
/// \return
/// Change the id. No date updated. Be careful since ids must be unique (sql
/// constraint)
SKRResult PLMPaperHub::setId(int projectId, int paperId, int newId)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    result = queries.setId(paperId, newId);
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit paperIdChanged(projectId, paperId, newId);
        emit projectModified(projectId);
    }
    return result;
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setTitle(int projectId, int paperId, const QString& newTitle)
{
    SKRResult result = set(projectId, paperId, "t_title", newTitle);

    IFOK(result) {
        emit titleChanged(projectId, paperId, newTitle);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QString PLMPaperHub::getTitle(int projectId, int paperId) const
{
    return get(projectId, paperId, "t_title").toString();
}

SKRResult PLMPaperHub::setIndent(int projectId, int paperId, int newIndent)
{
    SKRResult result = set(projectId, paperId, "l_indent", newIndent);

    IFOK(result) {
        emit indentChanged(projectId, paperId, newIndent);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

int PLMPaperHub::getIndent(int projectId, int paperId) const
{
    int indent;

    if (paperId == -1) // is project item
        indent = -1;
    else {
        indent = get(projectId, paperId, "l_indent").toInt();
    }

    return indent;
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setSortOrder(int projectId, int paperId, int newSortOrder)
{
    SKRResult result = set(projectId, paperId, "l_sort_order", newSortOrder);

    IFOK(result) {
        emit sortOrderChanged(projectId, paperId, newSortOrder);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

int PLMPaperHub::getSortOrder(int projectId, int paperId) const
{
    return get(projectId, paperId, "l_sort_order").toInt();
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setContent(int projectId, int paperId, const QString& newContent)
{
    // count char and words, threading if needed

    if(plmdata->projectHub()->isProjectToBeClosed(projectId) ){
        m_wordMeter->countText(m_paperType, projectId, paperId, newContent, true);
    }
    else{
        m_wordMeter->countText(m_paperType, projectId, paperId, newContent, false);
    }


    SKRResult result = set(projectId, paperId, "m_content", newContent);

    IFOK(result) {
        emit contentChanged(projectId, paperId, newContent);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QString PLMPaperHub::getContent(int projectId, int paperId) const
{
    QString content = get(projectId, paperId, "m_content").toString();

    return content;
}

// ------------------------------------------------------------
SKRResult PLMPaperHub::untrashOnlyOnePaper(int projectId, int paperId) {
    SKRResult result = set(projectId, paperId, "b_trashed", false);

    IFOKDO(result, this->setTrashedDateToNull(projectId, paperId));

    IFOK(result) {
        emit trashedChanged(projectId, paperId, false);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------
SKRResult PLMPaperHub::setTrashedWithChildren(int  projectId,
                                              int  paperId,
                                              bool newTrashedState)
{
    SKRResult result(this);

    QList<int> childrenIdList = this->getAllChildren(projectId, paperId);


    // children deletion (or recovery)
    for (int& _id : childrenIdList) {
        IFOKDO(result, set(projectId, _id, "b_trashed", newTrashedState));

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(projectId, _id).isNull()) {
            result = this->setTrashedDateToNow(projectId, _id);
            emit trashedChanged(projectId, _id, newTrashedState);
        }

        // restore
        else if (!newTrashedState) {
            result = this->setTrashedDateToNull(projectId, _id);
            emit trashedChanged(projectId, _id, newTrashedState);
        }

        // else ignore those already trashed
    }


    // do parent :
    IFOK(result) {
        result = set(projectId, paperId, "b_trashed", newTrashedState);

        // set date but those already deleted
        if (newTrashedState && this->getTrashedDate(projectId, paperId).isNull()) {
            result = this->setTrashedDateToNow(projectId, paperId);
        }

        // restore
        else if (!newTrashedState) {
            result = this->setTrashedDateToNull(projectId, paperId);
        }
    }
    IFOK(result) {
        emit trashedChanged(projectId, paperId, newTrashedState);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

bool PLMPaperHub::getTrashed(int projectId, int paperId) const
{
    return get(projectId, paperId, "b_trashed").toBool();
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setTrashedDateToNow(int projectId, int paperId)
{
    return set(projectId, paperId, "dt_trashed", QDateTime::currentDateTime());
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setTrashedDateToNull(int projectId, int paperId)
{
    return set(projectId, paperId, "dt_trashed", "NULL");
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getTrashedDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_trashed").toDateTime();
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setCreationDate(int projectId, int paperId,
                                       const QDateTime& newDate)
{
    SKRResult result = set(projectId, paperId, "dt_created", newDate);

    IFOK(result) {
        emit creationDateChanged(projectId, paperId, newDate);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getCreationDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_created").toDateTime();
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setUpdateDate(int projectId, int paperId, const QDateTime& newDate)
{
    SKRResult result = set(projectId, paperId, "dt_updated", newDate);

    IFOK(result) {
        emit updateDateChanged(projectId, paperId, newDate);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getUpdateDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_updated").toDateTime();
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::setContentDate(int projectId, int paperId, const QDateTime& newDate)
{
    SKRResult result = set(projectId, paperId, "dt_content", newDate);

    IFOK(result) {
        emit contentDateChanged(projectId, paperId, newDate);
        emit projectModified(projectId);
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getContentDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_content").toDateTime();
}

// ------------------------------------------------------------

bool PLMPaperHub::hasChildren(int  projectId,
                              int  paperId,
                              bool trashedAreIncluded,
                              bool notTrashedAreIncluded) const
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    // if last of id list:
    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.isEmpty()) { // project item with nothing else
        return false;
    }

    if (paperId == idList.last()) {
        return false;
    }

    int indent = getIndent(projectId, paperId);

    int possibleFirstChildId;

    if (paperId == -1) {                     // project item
        possibleFirstChildId = idList.at(0); // first real paper in table
    }
    else {
        possibleFirstChildId = idList.at(idList.indexOf(paperId) + 1);
    }


    int possibleFirstChildIndent = getIndent(projectId, possibleFirstChildId);

    // verify indent of child
    if (indent == possibleFirstChildIndent - 1) {
        // verify if at least one child is not trashed
        bool haveOneNotTrashedChild = false;
        bool haveOneTrashedChild    = false;
        int  firstChildIndex        = idList.indexOf(possibleFirstChildId);

        for (int i = firstChildIndex; i < idList.count(); i++) {
            int childId = idList.at(i);
            int indent  = getIndent(projectId, childId);

            if (indent < possibleFirstChildIndent) {
                break;
            }

            if (indent == possibleFirstChildIndent) {
                if (getTrashed(projectId, childId) == false) {
                    haveOneNotTrashedChild = true;
                }
                else {
                    haveOneTrashedChild = true;
                }
            }
        }

        if (haveOneTrashedChild && trashedAreIncluded) {
            return true;
        }

        if (haveOneNotTrashedChild && notTrashedAreIncluded) {
            return true;
        }
        return false;
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return false;
}

// ------------------------------------------------------------


int PLMPaperHub::getTopPaperId(int projectId) const
{
    int value      = -2;
    QList<int> list = this->getAllIds(projectId);

    for (int id : list) {
        if (!this->getTrashed(projectId, id)) {
            value = id;
            break;
        }
    }


    return value;
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::getError()
{
    return m_error;
}

// ------------------------------------------------------------

void PLMPaperHub::setError(const SKRResult& result)
{
    m_error = result;
}

// ------------------------------------------------------------

SKRResult PLMPaperHub::set(int             projectId,
                           int             paperId,
                           const QString & fieldName,
                           const QVariant& value,
                           bool            setCurrentDateBool)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    result = queries.set(paperId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(result, queries.setCurrentDate(paperId, "dt_updated"));
    }

    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------

// SKRResult PLMPaperHub::setSetting(int             projectId,
//                                 const QString & fieldName,
//                                 const QVariant& value,
//                                 bool            setCurrentDateBool)
// {
//    SKRResult result(this);
//    PLMSqlQueries queries(projectId,
//                          "tbl_user_" + m_paperType + "_setting");

//    queries.beginTransaction();
//    result = queries.set(1, fieldName, value);

//    if (setCurrentDateBool) {
//        IFOKDO(result, queries.setCurrentDate(1, "dt_updated"));
//    }

//    IFKO(result) {
//        queries.rollback();
//    }
//    IFOK(result) {
//        queries.commit();
//    }
//    IFKO(result) {
//        emit errorSent(result);
//    }
//    return result;
// }

// ------------------------------------------------------------

// SKRResult PLMPaperHub::setDocSetting(int             projectId,
//                                    int             paperId,
//                                    const QString & fieldName,
//                                    const QVariant& value,
//                                    bool            setCurrentDateBool)
// {
//    SKRResult result(this);
//    PLMSqlQueries queries(projectId,
//                          "tbl_user_" + m_paperType + "_doc_list");

//    queries.beginTransaction();
//    result = queries.set(paperId, fieldName, value);

//    if (setCurrentDateBool) {
//        IFOKDO(result, queries.setCurrentDate(1, "dt_updated"));
//    }

//    IFKO(result) {
//        queries.rollback();
//    }
//    IFOK(result) {
//        queries.commit();
//    }
//    IFKO(result) {
//        emit errorSent(result);
//    }
//    return result;
// }

// ------------------------------------------------------------


QVariant PLMPaperHub::get(int projectId, int paperId, const QString& fieldName) const
{
    SKRResult result(this);
    QVariant var;
    QVariant value;
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.get(paperId, fieldName, var);
    IFOK(result) {
        value = var;
    }
    IFKO(result) {
        emit errorSent(result);
    }
    return value;
}

// ------------------------------------------------------------


// QVariant PLMPaperHub::getSetting(int projectId, const QString& fieldName)
// const
// {
//    SKRResult result(this);
//    QVariant var;
//    QVariant value;
//    PLMSqlQueries queries(projectId,
//                          "tbl_user_" + m_paperType + "_setting");

//    result = queries.get(1, fieldName, var);
//    IFOK(result) {
//        value = var;
//    }
//    IFKO(result) {
//        emit errorSent(result);
//    }
//    return value;
// }

//// ------------------------------------------------------------


// QVariant PLMPaperHub::getDocSetting(int projectId, int paperId,
//                                    const QString& fieldName) const
// {
//    SKRResult result(this);
//    QVariant var;
//    QVariant value;
//    PLMSqlQueries queries(projectId,
//                          "tbl_user_" + m_paperType + "_doc_list");

//    result = queries.get(paperId, fieldName, var);
//    IFOK(result) {
//        value = var;
//    }
//    IFKO(result) {
//        emit errorSent(result);
//    }
//    return value;
// }

// -----------------------------------------------------------------------------

int PLMPaperHub::getLastAddedId()
{
    return m_last_added_id;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::renumberSortOrders(int projectId)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);

    result = queries.renumberSortOrder();
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::addPaper(const QHash<QString, QVariant>& values, int projectId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    int newId      = -1;
    SKRResult result = queries.add(values, newId);

    IFOKDO(result, queries.renumberSortOrder());
    IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        m_last_added_id = newId;
        result.addData("paperId", newId);
        emit paperAdded(projectId, newId);
        emit projectModified(projectId);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::addPaperAbove(int projectId, int targetId)
{
    int target_indent = getIndent(projectId, targetId);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderBeforePaper(projectId, targetId);

    // finally add paper
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    IFOKDO(result, addPaper(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::addPaperBelow(int projectId, int targetId)
{
    int target_indent = getIndent(projectId, targetId);

    SKRResult result(this);
    int finalSortOrder = this->getValidSortOrderAfterPaper(projectId, targetId);

    // finally add paper
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    IFOKDO(result, addPaper(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// ------------------------------------------------------------------------------

int PLMPaperHub::getValidSortOrderBeforePaper(int projectId, int paperId) const
{
    int target_sort_order = getSortOrder(projectId, paperId);

    int finalSortOrder = target_sort_order - 1;

    return finalSortOrder;
}

// -----------------------------------------------------------------------------

int PLMPaperHub::getValidSortOrderAfterPaper(int projectId, int paperId) const
{
    int target_sort_order = getSortOrder(projectId, paperId);
    int target_indent     = getIndent(projectId, paperId);

    // find next node with the same indentation
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    PLMSqlQueries queries(projectId, m_tableName);
    SKRResult result     = queries.getValueByIdsWhere("l_sort_order", hash, where, true);
    int finalSortOrder = 0;

    // if node after
    if (!hash.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = hash.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != hash.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSort) {
                lowestSort = sort;
            }

            ++i;
        }

        finalSortOrder = lowestSort - 1;

        // if tree is empty
        if (finalSortOrder == -1) {
            finalSortOrder = 0;
        }
    }

    // if no node after (bottom of tree)
    else {
        QList<int> idList;
        IFOKDO(result, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            result = SKRResult(SKRResult::Critical, this, "idList_is_empty");
        }

        int lastId = idList.last();
        QHash<int, QVariant> hash2;
        IFOKDO(result,
               queries.getValueByIds("l_sort_order", hash2, "id", QVariant(lastId)));
        finalSortOrder = hash2.begin().value().toInt() + 1;
    }

    IFKO(result) {
        emit errorSent(result);
    }

    return finalSortOrder;
}

// -----------------------------------------------------------------------------

///
/// \brief PLMPaperHub::getAllChildren
/// \param projectId
/// \param paperId
/// \return
/// get all children, grand-children, ...  trashed or not
QList<int>PLMPaperHub::getAllChildren(int projectId, int paperId) {
    QList<int> childrenList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine children

    int  parentIndent = indentList.value(paperId);
    bool parentPassed = false;

    for (int id : sortedIdList) {
        if (id == paperId) {
            parentPassed = true;
            continue;
        }

        if (parentPassed) {
            int idIndent = indentList.value(id);

            if (idIndent > parentIndent) {
                childrenList.append(id);
            }

            if (idIndent <= parentIndent) {
                break;
            }
        }
    }

    return childrenList;
}

// -----------------------------------------------------------------------------

QList<int>PLMPaperHub::getAllAncestors(int projectId, int paperId) {
    QList<int> ancestorsList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine ancestors

    int indent = indentList.value(paperId);


    for (int i = sortedIdList.indexOf(paperId); i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == paperId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent - 1) {
            if (indent == -1) {
                break;
            }

            ancestorsList.append(id);

            indent = idIndent;
        }
    }

    return ancestorsList;
}

// -----------------------------------------------------------------------------

QList<int>PLMPaperHub::getAllSiblings(int projectId, int paperId) {
    QList<int> siblingsList;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);
    int paperSortedIdIndex     = sortedIdList.indexOf(paperId);


    // determine siblings

    int indent = indentList.value(paperId);

    // min sibling index
    int minSiblingIndex = paperSortedIdIndex;

    for (int i = paperSortedIdIndex; i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == paperId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if ((idIndent == indent - 1) || (indent == -1)) {
            break;
        }
        minSiblingIndex = i;
    }

    // min sibling index
    int maxSiblingIndex = paperSortedIdIndex;

    for (int i = paperSortedIdIndex; i < sortedIdList.count(); i++) {
        int id = sortedIdList.at(i);

        //        if (id == paperId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if ((idIndent == indent - 1) || (indent == -1)) {
            break;
        }
        maxSiblingIndex = i;
    }

    // alone, so no siblings
    if ((minSiblingIndex == paperSortedIdIndex) &&
            (maxSiblingIndex == paperSortedIdIndex)) {
        return siblingsList;
    }

    // same level

    for (int i = minSiblingIndex; i <= maxSiblingIndex; i++) {
        int id = sortedIdList.at(i);

        //        if (id == paperId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent) {
            siblingsList.append(id);
        }
    }

    siblingsList.removeAll(paperId);


    return siblingsList;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::addChildPaper(int projectId, int targetId)
{
    SKRResult result(this);
    PLMSqlQueries queries(projectId, m_tableName);


    int target_sort_order = getSortOrder(projectId, targetId);
    int target_indent     = getIndent(projectId, targetId);

    // for invalid parent ("root")
    if (targetId == -2) {
        result = SKRResult(SKRResult::Critical, this, "invalid_root_parent");
        return result;
    }

    // for project item as parent :
    if (targetId == -1) {
        target_indent = -1;

        // get the highest sort order
        QHash<int, QVariant> sortOrderResult;
        result = queries.getValueByIds("l_sort_order",
                                       sortOrderResult,
                                       QString(),
                                       QVariant(),
                                       true);

        target_sort_order = 0;

        for (const QVariant& sortOrder : sortOrderResult.values()) {
            target_sort_order = qMax(sortOrder.toInt(), target_sort_order);
        }
    }

    // find next node with the same indentation
    QHash<int, QVariant> hash;
    QHash<QString, QVariant> where;

    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    result = queries.getValueByIdsWhere("l_sort_order", hash, where, true);
    int finalSortOrder = 0;

    // if node after
    if (!hash.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = hash.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != hash.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSort) {
                lowestSort = sort;
            }

            ++i;
        }

        finalSortOrder = lowestSort - 1;

        // if tree is empty

        if (finalSortOrder == -1) {
            finalSortOrder = 0;
        }
    }

    // if no node after (bottom of tree)
    else {
        QList<int> idList;
        IFOKDO(result, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            finalSortOrder = 1000;
        } else {
            int lastId = idList.last();
            QHash<int, QVariant> hash2;
            IFOKDO(result,
                   queries.getValueByIds("l_sort_order", hash2, "id",
                                         QVariant(lastId)));
            finalSortOrder = hash2.begin().value().toInt() + 1;
        }
    }

    // finally add paper
    QHash<QString, QVariant> values;

    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent + 1);
    IFOKDO(result, addPaper(values, projectId));
    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::removePaper(int projectId, int targetId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    SKRResult result = queries.remove(targetId);

    IFOKDO(result, queries.renumberSortOrder())
            IFKO(result) {
        queries.rollback();
    }
    IFOK(result) {
        queries.commit();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit paperRemoved(projectId, targetId);
        emit projectModified(projectId);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::movePaper(int  sourceProjectId,
                                 int  sourcePaperId,
                                 int  targetPaperId,
                                 bool after)
{
    SKRResult result(this);
    int targetProjectId = sourceProjectId;
    PLMSqlQueries queries(sourceProjectId, m_tableName);


    // int sourceSortOrder = this->getSortOrder(sourceProjectId, sourcePaperId);
    int targetSortOrder = this->getSortOrder(sourceProjectId, targetPaperId);

    targetSortOrder = targetSortOrder + (after ? 1 : -1);
    result           = setSortOrder(sourceProjectId, sourcePaperId, targetSortOrder);
    IFOKDO(result, queries.renumberSortOrder())
            IFKO(result) {
        queries.rollback();
    }
    IFKO(result) {
        emit errorSent(result);
    }
    IFOK(result) {
        emit paperMoved(sourceProjectId, sourcePaperId, targetProjectId, targetPaperId);
        emit projectModified(sourceProjectId);
        emit projectModified(targetProjectId);
    }

    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::movePaperUp(int projectId, int paperId)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    // get paper before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.first() == paperId) {
        result = SKRResult(SKRResult::Critical, this, "first_in_idList_cant_move_up");
    }
    int targetPaperId = -2;

    IFOK(result) {
        // find paper before with same indent
        int possibleTargetPaperId = -2;

        for (int i = idList.indexOf(paperId) - 1; i >= 0; --i) {
            possibleTargetPaperId = idList.at(i);

            if (this->getIndent(projectId,
                                possibleTargetPaperId) ==
                    this->getIndent(projectId, paperId)) {
                targetPaperId = possibleTargetPaperId;
                break;
            }
        }

        if (possibleTargetPaperId == -2) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetPaperId_is_-2");
        }
        IFOK(result) {
            int targetIndent = this->getIndent(projectId, targetPaperId);
            int paperIndent  = this->getIndent(projectId, paperId);

            if (paperIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->movePaper(projectId, paperId, targetPaperId))


            IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::movePaperDown(int projectId, int paperId)
{
    SKRResult result(this);

    PLMSqlQueries queries(projectId, m_tableName);

    // get paper before this :

    QHash<int, QVariant> sortOrderResult;

    result = queries.getValueByIds("l_sort_order",
                                   sortOrderResult,
                                   QString(),
                                   QVariant(),
                                   true);


    QList<int> idList;

    IFOKDO(result, queries.getSortedIds(idList));

    if (idList.last() == paperId) {
        result = SKRResult(SKRResult::Critical, this, "last_in_idList_cant_move_down");
    }
    int targetPaperId = -2;

    IFOK(result) {
        // find paper after with same indent
        int possibleTargetPaperId = -2;

        for (int i = idList.indexOf(paperId) + 1; i < idList.count(); ++i) {
            possibleTargetPaperId = idList.at(i);

            if (this->getIndent(projectId,
                                possibleTargetPaperId) ==
                    this->getIndent(projectId, paperId)) {
                targetPaperId = possibleTargetPaperId;
                break;
            }
        }

        if (possibleTargetPaperId == -2) {
            result = SKRResult(SKRResult::Critical, this, "possibleTargetPaperId_is_-2");
        }
        IFOK(result) {
            int targetIndent = this->getIndent(projectId, targetPaperId);
            int paperIndent  = this->getIndent(projectId, paperId);

            if (paperIndent  != targetIndent) {
                result = SKRResult(SKRResult::Critical, this, "indents_dont_match");
            }
        }
    }
    IFOKDO(result, this->movePaper(projectId, paperId, targetPaperId, true))


            IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

SKRResult PLMPaperHub::movePaperAsChildOf(int projectId,
                                          int noteId,
                                          int targetParentId,
                                          int wantedSortOrder)
{
    SKRResult result(this);

    int validSortOrder = getValidSortOrderAfterPaper(projectId, targetParentId);

    if (wantedSortOrder == -1) {
        wantedSortOrder = validSortOrder;
    }

    if (wantedSortOrder > validSortOrder) {
        result = SKRResult(SKRResult::Critical, this, "wantedSortOrder_is_outside_scope_of_parent");
    }
    IFOK(result) {
        result = this->setSortOrder(projectId, noteId, wantedSortOrder);
    }
    IFOK(result) {
        int parentIndent = this->getIndent(projectId, targetParentId);

        result = this->setIndent(projectId, noteId, parentIndent + 1);
    }

    IFKO(result) {
        emit errorSent(result);
    }
    return result;
}

// -----------------------------------------------------------------------------

int PLMPaperHub::getParentId(int projectId, int paperId)
{
    int parentId = -2;

    // get indents
    QHash<int, int> indentList = getAllIndents(projectId);
    QList<int> sortedIdList    = getAllIds(projectId);


    // determine direct ancestor

    int indent = indentList.value(paperId);

    if (indent == 0) {
        return -1;
    }

    for (int i = sortedIdList.indexOf(paperId); i >= 0; i--) {
        int id = sortedIdList.at(i);

        //        if (id == paperId) {
        //            continue;
        //        }

        int idIndent = indentList.value(id);

        if (idIndent == indent - 1) {
            parentId = id;
            break;
        }
    }


    return parentId;
}

// -----------------------------------------------------------------------------

// SKRResult PLMPaperHub::settings_setStackSetting(PLMPaperHub::Stack   stack,
//                                               PLMPaperHub::Setting setting,
//                                               const QVariant     & value)
// {
//    SKRResult result(this);

//    foreach(int projectId, plmProjectManager->projectIdList()) {
//        if (setting == PLMPaperHub::SplitterState) {
//            result = setSetting(projectId, "m_splitter_state", value, true);
//        }

//        if (setting == PLMPaperHub::WindowState) {
//            result = setSetting(projectId, "m_window_state", value, true);
//        }

//        if (setting == PLMPaperHub::SettingDate) {
//            result = setSetting(projectId, "dt_updated", value, true);
//        }

//        if (setting == PLMPaperHub::Minimap) {
//            if (stack == PLMPaperHub::Zero) {
//                result = setSetting(projectId, "b_stack_0_map", value, true);
//            }

//            if (stack == PLMPaperHub::One) {
//                result = setSetting(projectId, "b_stack_1_map", value, true);
//            }
//        }

//        if (setting == PLMPaperHub::Fit) {
//            if (stack == PLMPaperHub::Zero) {
//                result = setSetting(projectId, "b_stack_0_fit", value, true);
//            }

//            if (stack == PLMPaperHub::One) {
//                result = setSetting(projectId, "b_stack_1_fit", value, true);
//            }
//        }

//        if (setting == PLMPaperHub::SpellCheck) {
//            if (stack == PLMPaperHub::Zero) {
//                result = setSetting(projectId, "b_stack_0_spellcheck", value,
// true);
//            }

//            if (stack == PLMPaperHub::One) {
//                result = setSetting(projectId, "b_stack_1_spellcheck", value,
// true);
//            }
//        }

//        if (setting == PLMPaperHub::StackState) {
//            if (stack == PLMPaperHub::Zero) {
//                result = setSetting(projectId, "b_stack_0_state", value, true);
//            }

//            if (stack == PLMPaperHub::One) {
//                result = setSetting(projectId, "b_stack_1_state", value, true);
//            }
//        }
//    }

//    IFOK(result) {
//        emit settings_settingChanged(stack, setting, value);
//    }
//    return result;
// }

//// -----------------------------------------------------------------------------

// QVariant PLMPaperHub::settings_getStackSetting(PLMPaperHub::Stack   stack,
//                                               PLMPaperHub::Setting setting)
// const
// {
//    int projectId = plmProjectManager->projectIdList().first();
//    QVariant value;

//    if (setting == PLMPaperHub::SplitterState) {
//        value = getSetting(projectId, "m_splitter_state");
//    }

//    if (setting == PLMPaperHub::WindowState) {
//        value = getSetting(projectId, "m_window_state");
//    }

//    if (setting == PLMPaperHub::SettingDate) {
//        value = getSetting(projectId, "dt_updated");
//    }

//    if (setting == PLMPaperHub::Minimap) {
//        if (stack == PLMPaperHub::Zero) {
//            value = getSetting(projectId, "b_stack_0_map");
//        }

//        if (stack == PLMPaperHub::One) {
//            value = getSetting(projectId, "b_stack_1_map");
//        }
//    }

//    if (setting == PLMPaperHub::Fit) {
//        if (stack == PLMPaperHub::Zero) {
//            value = getSetting(projectId, "b_stack_0_fit");
//        }

//        if (stack == PLMPaperHub::One) {
//            value = getSetting(projectId, "b_stack_1_fit");
//        }
//    }

//    if (setting == PLMPaperHub::SpellCheck) {
//        if (stack == PLMPaperHub::Zero) {
//            value = getSetting(projectId, "b_stack_0_spellcheck");
//        }

//        if (stack == PLMPaperHub::One) {
//            value = getSetting(projectId, "b_stack_1_spellcheck");
//        }
//    }

//    if (setting == PLMPaperHub::StackState) {
//        if (stack == PLMPaperHub::Zero) {
//            value = getSetting(projectId, "b_stack_0_state");
//        }

//        if (stack == PLMPaperHub::One) {
//            value = getSetting(projectId, "b_stack_1_state");
//        }
//    }

//    return value;
// }

//// -----------------------------------------------------------------------------

// SKRResult PLMPaperHub::settings_setDocSetting(int
//                           projectId,
//                                             int
//                           paperId,
//                                             PLMPaperHub::OpenedDocSetting
// setting,
//                                             const QVariant              &
// value)
// {
//    SKRResult result(this);

//    if (setting == PLMPaperHub::StackNumber) {
//        result = setDocSetting(projectId, paperId, "l_statck", value, true);
//    }

//    if (setting == PLMPaperHub::Hovering) {
//        result = setDocSetting(projectId, paperId, "b_hovering", value, true);
//    }

//    if (setting == PLMPaperHub::Visible) {
//        result = setDocSetting(projectId, paperId, "b_visible", value, true);
//    }

//    if (setting == PLMPaperHub::HasFocus) {
//        result = setDocSetting(projectId, paperId, "b_has_focus", value, true);
//    }

//    if (setting == PLMPaperHub::CursorPosition) {
//        result = setDocSetting(projectId, paperId, "l_cursor_pos", value,
// true);
//    }

//    if (setting == PLMPaperHub::HoveringGeometry) {
//        result = setDocSetting(projectId, paperId, "m_geometry", value, true);
//    }

//    if (setting == PLMPaperHub::Date) {
//        result = setDocSetting(projectId, paperId, "dt_updated", value, true);
//    }

//    IFOK(result) {
//        emit settings_docSettingChanged(projectId, paperId, setting, value);
//    }
//    return result;
// }

//// -----------------------------------------------------------------------------

// QVariant PLMPaperHub::settings_getDocSetting(int
//                           projectId,
//                                             int
//                           paperId,
//                                             PLMPaperHub::OpenedDocSetting
// setting) const
// {
//    QVariant value;

//    if (setting == PLMPaperHub::StackNumber) {
//        value = getDocSetting(projectId, paperId, "l_statck");
//    }

//    if (setting == PLMPaperHub::Hovering) {
//        value = getDocSetting(projectId, paperId, "b_hovering");
//    }

//    if (setting == PLMPaperHub::Visible) {
//        value = getDocSetting(projectId, paperId, "b_visible");
//    }

//    if (setting == PLMPaperHub::HasFocus) {
//        value = getDocSetting(projectId, paperId, "b_has_focus");
//    }

//    if (setting == PLMPaperHub::CursorPosition) {
//        value = getDocSetting(projectId, paperId, "l_cursor_pos");
//    }

//    if (setting == PLMPaperHub::HoveringGeometry) {
//        value = getDocSetting(projectId, paperId, "m_geometry");
//    }

//    if (setting == PLMPaperHub::Date) {
//        value = getDocSetting(projectId, paperId, "dt_updated");
//    }

//    return value;
// }

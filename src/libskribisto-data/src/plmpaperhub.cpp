#include "plmpaperhub.h"
#include "tasks/plmsqlqueries.h"
#include "tools.h"
#include "tasks/plmprojectmanager.h"

#include <QDateTime>
#include <QDebug>

///
/// \brief PLMPaperHub::PLMPaperHub
/// \param parent
//
PLMPaperHub::PLMPaperHub(QObject *parent, const QString& tableName)
    : QObject(parent), m_tableName(tableName), m_last_added_id(-1)
{
    qRegisterMetaType<Setting>(         "Setting");
    qRegisterMetaType<Stack>(           "Stack");
    qRegisterMetaType<OpenedDocSetting>("OpenedDocSetting");

    // connection for 'getxxx' functions to have a way to get errors.
    connect(this,
            &PLMPaperHub::errorSent,
            this,
            &PLMPaperHub::setError,
            Qt::DirectConnection);

    if (m_tableName.contains("note")) {
        m_paperType = "note";
    } else if (m_tableName.contains("sheet")) {
        m_paperType = "sheet";
    }
}

///
/// \brief PLMPaperHub::getAll
/// \param projectId
/// \return
/// unimplemented
QList<QHash<QString, QVariant> >PLMPaperHub::getAll(int projectId) const
{
    Q_UNUSED(projectId)

    QList<QHash<QString, QVariant> >result;
    return result;
}

///
/// \brief PLMPaperHub::getAllTitles
/// \param projectId
/// \return
///
QHash<int, QString>PLMPaperHub::getAllTitles(int projectId) const
{
    PLMError error;

    QHash<int, QString>  result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    error = queries.getValueByIds("t_title", out);
    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntQString(out);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
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
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    error = queries.getValueByIds("l_indent", out);
    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// ------------------------------------------------------------

/**
 * @brief PLMPaperHub::getAllIds
 * @param projectId
 * @return
 * Get sorted ids
 */
QList<int>PLMPaperHub::getAllIds(int projectId) const
{
    PLMError error;

    QList<int> result;
    QList<int> out;
    PLMSqlQueries queries(projectId, m_tableName);
    error = queries.getSortedIds(out);
    IFOK(error) {
        result = out;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// ------------------------------------------------------------
///
/// \brief PLMPaperHub::getOverallSize
/// \return number of papers for all projects
///
int PLMPaperHub::getOverallSize()
{
    PLMError error;

    QList<int> result;
    foreach(int projectId, plmProjectManager->projectIdList()) {
        PLMSqlQueries queries(projectId, m_tableName);

        error = queries.getIds(result);
        IFOK(error) {
            return result.size();
        }
        IFKO(error) {
            emit errorSent(error);

            return 0;
        }
    }
    return 0;
}

// ------------------------------------------------------------

QHash<int, int>PLMPaperHub::getAllSortOrders(int projectId) const
{
    PLMError error;

    QHash<int, int> result;
    QHash<int, QVariant> out;
    PLMSqlQueries queries(projectId, m_tableName);
    error = queries.getValueByIds("l_sort_order", out);
    IFOK(error) {
        result = HashIntQVariantConverter::convertToIntInt(out);
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// ------------------------------------------------------------
///
/// \brief PLMPaperHub::setId
/// \param projectId
/// \param paperId
/// \param newId
/// \return
/// Change de id. No date updated. Be careful since ids must be unique (sql
// constraint)
PLMError PLMPaperHub::setId(int projectId, int paperId, int newId)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    error = queries.setId(paperId, newId);
    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit paperIdChanged(projectId, paperId, newId);
    }
    return error;
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setTitle(int projectId, int paperId, const QString& newTitle)
{
    PLMError error = set(projectId, paperId, "t_title", newTitle);

    IFOK(error) {
        emit titleChanged(projectId, paperId, newTitle);
    }
    return error;
}

// ------------------------------------------------------------

QString PLMPaperHub::getTitle(int projectId, int paperId) const
{
    return get(projectId, paperId, "t_title").toString();
}

PLMError PLMPaperHub::setIndent(int projectId, int paperId, int newIndent)
{
    PLMError error = set(projectId, paperId, "l_indent", newIndent);

    IFOK(error) {
        emit indentChanged(projectId, paperId, newIndent);
    }
    return error;
}

// ------------------------------------------------------------

int PLMPaperHub::getIndent(int projectId, int paperId) const
{
    return get(projectId, paperId, "l_indent").toInt();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setSortOrder(int projectId, int paperId, int newSortOrder)
{
    PLMError error = set(projectId, paperId, "l_sort_order", newSortOrder);

    IFOK(error) {
        emit sortOrderChanged(projectId, paperId, newSortOrder);
    }
    return error;
}

// ------------------------------------------------------------

int PLMPaperHub::getSortOrder(int projectId, int paperId) const
{
    return get(projectId, paperId, "l_sort_order").toInt();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setContent(int projectId, int paperId, const QString& newContent)
{
    PLMError error = set(projectId, paperId, "m_content", newContent);

    IFOK(error) {
        emit contentChanged(projectId, paperId, newContent);
    }
    return error;
}

// ------------------------------------------------------------

QString PLMPaperHub::getContent(int projectId, int paperId) const
{
    return get(projectId, paperId, "m_content").toString();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setDeleted(int projectId, int paperId, bool newDeletedState)
{
    int target_sort_order = getSortOrder(projectId, paperId);
    int target_indent     = getIndent(projectId, paperId);

    QHash<int, QVariant> result;
    QHash<QString, QVariant> where;
    where.insert("l_indent >",     target_indent);
    where.insert("l_sort_order >", target_sort_order);

    if (newDeletedState) {
        where.insert("b_deleted =", 0);
    }

    PLMSqlQueries queries(projectId, m_tableName);
    PLMError error = queries.getValueByIdsWhere("l_sort_order", result, where, true);
    QHash<int, QVariant> result_same_indent;
    where.clear();
    where.insert("l_indent =",     target_indent);
    where.insert("l_sort_order >", target_sort_order);
    IFOKDO(error,
           queries.getValueByIdsWhere("l_sort_order", result_same_indent, where, true));
    int lowestSortSameLevel = 0;

    if (!result_same_indent.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = result_same_indent.constBegin();
        lowestSortSameLevel = i.value().toInt();

        while (i != result_same_indent.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSortSameLevel) {
                lowestSortSameLevel = sort;
            }

            ++i;
        }
    }

    // filter the children
    QList<int> chilrenIdList;

    if (!result.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = result.constBegin();

        // int lowestSort = i.value().toInt();

        while (i != result.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSortSameLevel) {
                chilrenIdList.append(i.key());
            }

            ++i;
        }

        // children deletion (or recovery)
        for (int& _id : chilrenIdList) {
            IFOKDO(error, set(projectId, _id, "b_deleted", newDeletedState));
            emit deletedChanged(projectId, _id, newDeletedState);
        }
    }

    // TODO:  deletion
    IFOKDO(error, set(projectId, paperId, "b_deleted", newDeletedState));
    IFOK(error) {
        emit deletedChanged(projectId, paperId, newDeletedState);
    }
    return error;
}

// ------------------------------------------------------------

bool PLMPaperHub::getDeleted(int projectId, int paperId) const
{
    // TODO: do recursive deletion
    return get(projectId, paperId, "b_deleted").toBool();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setCreationDate(int projectId, int paperId,
                                      const QDateTime& newDate)
{
    PLMError error = set(projectId, paperId, "dt_created", newDate);

    IFOK(error) {
        emit creationDateChanged(projectId, paperId, newDate);
    }
    return error;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getCreationDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_created").toDateTime();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setUpdateDate(int projectId, int paperId, const QDateTime& newDate)
{
    PLMError error = set(projectId, paperId, "dt_updated", newDate);

    IFOK(error) {
        emit updateDateChanged(projectId, paperId, newDate);
    }
    return error;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getUpdateDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_updated").toDateTime();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setContentDate(int projectId, int paperId, const QDateTime& newDate)
{
    PLMError error = set(projectId, paperId, "dt_content", newDate);

    IFOK(error) {
        emit contentDateChanged(projectId, paperId, newDate);
    }
    return error;
}

// ------------------------------------------------------------

QDateTime PLMPaperHub::getContentDate(int projectId, int paperId) const
{
    return get(projectId, paperId, "dt_content").toDateTime();
}

// ------------------------------------------------------------

PLMError PLMPaperHub::getError()
{
    return m_error;
}

// ------------------------------------------------------------

void PLMPaperHub::setError(const PLMError& error)
{
    m_error = error;
}

// ------------------------------------------------------------

PLMError PLMPaperHub::set(int             projectId,
                          int             paperId,
                          const QString & fieldName,
                          const QVariant& value,
                          bool            setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    error = queries.set(paperId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(paperId, "dt_updated"));
    }

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setSetting(int             projectId,
                                 const QString & fieldName,
                                 const QVariant& value,
                                 bool            setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId,
                          "tbl_user_" + m_paperType + "_setting");

    queries.beginTransaction();
    error = queries.set(1, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(1, "dt_updated"));
    }

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

// ------------------------------------------------------------

PLMError PLMPaperHub::setDocSetting(int             projectId,
                                    int             paperId,
                                    const QString & fieldName,
                                    const QVariant& value,
                                    bool            setCurrentDateBool)
{
    PLMError error;
    PLMSqlQueries queries(projectId,
                          "tbl_user_" + m_paperType + "_doc_list");

    queries.beginTransaction();
    error = queries.set(paperId, fieldName, value);

    if (setCurrentDateBool) {
        IFOKDO(error, queries.setCurrentDate(1, "dt_updated"));
    }

    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}

// ------------------------------------------------------------


QVariant PLMPaperHub::get(int projectId, int paperId, const QString& fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    PLMSqlQueries queries(projectId, m_tableName);

    error = queries.get(paperId, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// ------------------------------------------------------------


QVariant PLMPaperHub::getSetting(int projectId, const QString& fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    PLMSqlQueries queries(projectId,
                          "tbl_user_" + m_paperType + "_setting");

    error = queries.get(1, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// ------------------------------------------------------------


QVariant PLMPaperHub::getDocSetting(int projectId, int paperId,
                                    const QString& fieldName) const
{
    PLMError error;
    QVariant var;
    QVariant result;
    PLMSqlQueries queries(projectId,
                          "tbl_user_" + m_paperType + "_doc_list");

    error = queries.get(paperId, fieldName, var);
    IFOK(error) {
        result = var;
    }
    IFKO(error) {
        emit errorSent(error);
    }
    return result;
}

// -----------------------------------------------------------------------------

int PLMPaperHub::getLastAddedId()
{
    return m_last_added_id;
}


// -----------------------------------------------------------------------------

PLMError PLMPaperHub::renumberSortOrders(int projectId)
{
    PLMError error;
    PLMSqlQueries queries(projectId,
                          "tbl_user_" + m_paperType + "_doc_list");
    error = queries.renumberSortOrder();
    IFKO(error) {
        emit errorSent(error);
    }
    return error;
}
// -----------------------------------------------------------------------------

/////
///// \brief PLMPaperHub::getParentList
///// \param projectId
///// \param paperId
///// \return
///// get a list of the parents' ids
// QList<int>PLMPaperHub::getParentList(int projectId, int paperId) const
// {
//    Q_UNUSED(paperId)
//    PLMError error;
//    QList<int> var;
//    QList<int> result;
//    PLMSqlQueries queries(projectId, "tbl_" + m_paperType,
// PLMSqlQueries::ProjectDB);

//    // find paperId indent
//    // int paperIdIndent;
//    // error = queries.get(paperId, "l_indent", paperIdIndent);


//    // IFOKDO(error, queries.getValueByIdsWhere())


//    IFOK(error) {
//        result = var;
//    }
//    IFKO(error) {
//        emit errorSent(error);
//    }
//    return result;
// }

//// -----------------------------------------------------------------------------

// int PLMPaperHub::getRowAmongChildren(int projectId, int paperId) const
// {
//    int childIndent = getIndent(projectId, paperId);

//    // if child is top level
//    if (childIndent == 0)
//    {
//        return -1;
//    }


//    PLMError error;
//    PLMSqlQueries queries(projectId, m_tableName, PLMSqlQueries::ProjectDB);
//    QList<int>    sortedIds;
//    error = queries.getSortedIds(sortedIds);


//    IFKO(error) {
//        emit errorSent(error);

//        return -2;
//    }

//    // if no item
//    if (sortedIds.count() == 0)
//    {
//        return -1;
//    }

//    int childIndex = sortedIds.indexOf(paperId);

//    // find first child
//    int indent              = -2;
//    int possibleParentId    = -2;
//    int possibleParentIndex = childIndex;
//    int firstChildIndex     = childIndex;

//    while (indent != childIndent - 1) {
//        firstChildIndex      = possibleParentIndex;
//        possibleParentIndex -= 1;
//        possibleParentId     = sortedIds.at(possibleParentIndex);
//        indent               = getIndent(projectId, possibleParentId);
//    }

//    // find last child
//    indent = -2;
//    int possibleLastChildId = -2;
//    int lastChildIndex      = childIndex;

//    while (indent >= childIndent) {
//        // if last of list
//        if (sortedIds.count() - 1 == lastChildIndex) break;

//        // else
//        lastChildIndex     += 1;
//        possibleLastChildId = sortedIds.at(lastChildIndex);
//        indent              = getIndent(projectId, possibleLastChildId);
//    }

//    while (indent == childIndent) {
//        lastChildIndex     -= 1;
//        possibleLastChildId = sortedIds.at(lastChildIndex);
//        indent              = getIndent(projectId, possibleLastChildId);
//    }


//    // find number of children:
//    int childrenCount = lastChildIndex - firstChildIndex + 1;
//    Q_UNUSED(childrenCount)

//    int row = childIndex - firstChildIndex;

//    return row;
// }

//// -----------------------------------------------------------------------------

// int PLMPaperHub::getChildIdFromParentAndRow(int projectId, int parentId, int
// row) const
// {
//    int parentIndent     = getIndent(projectId, parentId);
//    int firstChildIndent = parentIndent + 1;


//    PLMError error;
//    PLMSqlQueries queries(projectId, m_tableName, PLMSqlQueries::ProjectDB);

//    QList<int> sortedIds;
//    error = queries.getSortedIds(sortedIds);


//    IFKO(error) {
//        emit errorSent(error);

//        return -2;
//    }

//    int parentIndex = sortedIds.indexOf(parentId);
//    QList<int> listOfDirectChildren;
//    int indent = firstChildIndent;
//    int index  = parentIndex;

//    while (indent >= firstChildIndent) {
//        index += 1;

//        int id = sortedIds.at(index);
//        indent = getIndent(projectId, id);

//        if (indent == firstChildIndent) {
//            listOfDirectChildren.append(id);
//        }
//    }


//    return listOfDirectChildren.at(row);
// }

//// -----------------------------------------------------------------------------

// int PLMPaperHub::getChildRowCount(int projectId, int parentId) const
// {
//    if (parentId == -1) {
//        PLMError error;
//        PLMSqlQueries queries(projectId, m_tableName,
// PLMSqlQueries::ProjectDB);

//        QList<int> sortedIds;
//        error = queries.getSortedIds(sortedIds);


//        IFKO(error) {
//            emit errorSent(error);

//            return -2;
//        }

//        QList<int> listOfDirectChildren;
//        int indent = 0;
//        int index  = 0;

//        while (indent >= 0) {
//            index += 1;

//            if (index >= sortedIds.count()) break;

//            int id = sortedIds.at(index);
//            indent = getIndent(projectId, id);

//            if (indent == 0) {
//                listOfDirectChildren.append(id);
//            }
//        }

//        return listOfDirectChildren.count();
//    }


//    int parentIndent     = getIndent(projectId, parentId);
//    int firstChildIndent = parentIndent + 1;


//    PLMError error;
//    PLMSqlQueries queries(projectId, m_tableName, PLMSqlQueries::ProjectDB);

//    QList<int> sortedIds;
//    error = queries.getSortedIds(sortedIds);


//    IFKO(error) {
//        emit errorSent(error);

//        return -2;
//    }

//    int parentIndex = sortedIds.indexOf(parentId);
//    QList<int> listOfDirectChildren;
//    int indent = firstChildIndent;
//    int index  = parentIndex;

//    while (indent >= firstChildIndent) {
//        index += 1;

//        int id = sortedIds.at(index);
//        indent = getIndent(projectId, id);

//        if (indent == firstChildIndent) {
//            listOfDirectChildren.append(id);
//        }
//    }


//    return listOfDirectChildren.count();
// }

//// -----------------------------------------------------------------------------

// int PLMPaperHub::getDirectParentId(int projectId, int paperId) const
// {
//    int childIndent = getIndent(projectId, paperId);

//    // if child is top level
//    if (childIndent == 0)
//    {
//        return -1;
//    }


//    PLMError error;
//    PLMSqlQueries queries(projectId, m_tableName, PLMSqlQueries::ProjectDB);
//    QList<int>    sortedIds;
//    error = queries.getSortedIds(sortedIds);


//    IFKO(error) {
//        emit errorSent(error);

//        return -2;
//    }

//    // if no item
//    if (sortedIds.count() == 0)
//    {
//        return -1;
//    }

//    int childIndex = sortedIds.indexOf(paperId);

//    int indent              = -1;
//    int possibleParentId    = -1;
//    int possibleParentIndex = childIndex;

//    while (indent != childIndent - 1) {
//        possibleParentId = sortedIds.at(possibleParentIndex - 1);
//        indent           = getIndent(projectId, possibleParentId);
//    }
//    return possibleParentId;
// }

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::addPaper(const QHash<QString, QVariant>& values, int projectId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    int newId      = -1;
    PLMError error = queries.add(values, newId);
    IFOKDO(error, queries.renumberSortOrder());
    IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    m_last_added_id = newId;
    IFOK(error) {
        emit paperAdded(projectId, newId);
    }
    return error;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::addPaperBelow(int projectId, int targetId)
{
    int target_indent     = getIndent(projectId, targetId);

    PLMError error;
    int finalSortOrder = this->getValidSortOrderAfterPaper(projectId, targetId);

    // finally add paper
    QHash<QString, QVariant> values;
    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent);
    IFOKDO(error, addPaper(values, projectId));
    return error;
}
//------------------------------------------------------------------------------

int PLMPaperHub::getValidSortOrderAfterPaper(int projectId, int paperId) const
{
    int target_sort_order = getSortOrder(projectId, paperId);
    int target_indent     = getIndent(projectId, paperId);

    // find next node with the same indentation
    QHash<int, QVariant> result;
    QHash<QString, QVariant> where;
    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    PLMSqlQueries queries(projectId, m_tableName);
    PLMError error     = queries.getValueByIdsWhere("l_sort_order", result, where, true);
    int finalSortOrder = 0;

    // if node after
    if (!result.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = result.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != result.constEnd()) {
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
        IFOKDO(error, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            error.setSuccess(false);
            return error;
        }

        int lastId = idList.last();
        QHash<int, QVariant> result2;
        IFOKDO(error,
               queries.getValueByIds("l_sort_order", result2, "id", QVariant(lastId)));
        finalSortOrder = result2.begin().value().toInt() + 1;
    }

    IFKO(error) {
        emit errorSent(error);
    }

    return finalSortOrder;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::addChildPaper(int projectId, int targetId)
{
    PLMError error;
    PLMSqlQueries queries(projectId, m_tableName);



    int target_sort_order = getSortOrder(projectId, targetId);
    int target_indent     = getIndent(projectId, targetId);

    //for invalid parent ("root")
    if(targetId == 0){
        target_indent = -1;

        //get the highest sort order
        QHash<int, QVariant> sortOrderResult;
        error     = queries.getValueByIds("l_sort_order", sortOrderResult);

        target_sort_order = 0;
        for(const QVariant &sortOrder : sortOrderResult.values()){
            target_sort_order = qMax(sortOrder.toInt(), target_sort_order);
        }
    }

    // find next node with the same indentation
    QHash<int, QVariant> result;
    QHash<QString, QVariant> where;
    where.insert("l_indent <=",    target_indent);
    where.insert("l_sort_order >", target_sort_order);
    error     = queries.getValueByIdsWhere("l_sort_order", result, where, true);
    int finalSortOrder = 0;

    // if node after
    if (!result.isEmpty()) {
        QHash<int, QVariant>::const_iterator i = result.constBegin();
        int lowestSort                         = 0;
        lowestSort = i.value().toInt();

        while (i != result.constEnd()) {
            int sort = i.value().toInt();

            if (sort < lowestSort) {
                lowestSort = sort;
            }

            ++i;
        }

        finalSortOrder = lowestSort - 1;

        // if tree is empty
        IFKO(error) {
            queries.rollback();
        }

        if (finalSortOrder == -1) {
            finalSortOrder = 0;
        }
    }

    // if no node after (bottom of tree)
    else {
        QList<int> idList;
        IFOKDO(error, queries.getSortedIds(idList));

        if (idList.isEmpty()) {
            error.setSuccess(false);
            return error;
        }

        int lastId = idList.last();
        QHash<int, QVariant> result2;
        IFOKDO(error,
               queries.getValueByIds("l_sort_order", result2, "id", QVariant(lastId)));
        finalSortOrder = result2.begin().value().toInt() + 1;
    }

    // finally add paper
    QHash<QString, QVariant> values;
    values.insert("l_sort_order", finalSortOrder);
    values.insert("l_indent",     target_indent + 1);
    IFOKDO(error, addPaper(values, projectId));
    return error;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::removePaper(int projectId, int targetId)
{
    PLMSqlQueries queries(projectId, m_tableName);

    queries.beginTransaction();
    PLMError error = queries.remove(targetId);
    IFOKDO(error, queries.renumberSortOrder())
            IFKO(error) {
        queries.rollback();
    }
    IFOK(error) {
        queries.commit();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit paperRemoved(projectId, targetId);
    }
    return error;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::movePaper(int sourceProjectId, int sourcePaperId, int targetPaperId)
{
    PLMError error;
    int targetProjectId = sourceProjectId;
    PLMSqlQueries queries(sourceProjectId, m_tableName);



    //int sourceSortOrder = this->getSortOrder(sourceProjectId, sourcePaperId);
    int targetSortOrder = this->getSortOrder(sourceProjectId, targetPaperId);

    error = setSortOrder(sourceProjectId, sourcePaperId, targetSortOrder - 1);
    IFOKDO(error, queries.renumberSortOrder())
            IFKO(error) {
        queries.rollback();
    }
    IFKO(error) {
        emit errorSent(error);
    }
    IFOK(error) {
        emit paperMoved(sourceProjectId, sourcePaperId, targetProjectId, targetPaperId);
    }

    return error;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::settings_setStackSetting(PLMPaperHub::Stack   stack,
                                               PLMPaperHub::Setting setting,
                                               const QVariant     & value)
{
    PLMError error;

    foreach(int projectId, plmProjectManager->projectIdList()) {
        if (setting == PLMPaperHub::SplitterState) {
            error = setSetting(projectId, "m_splitter_state", value, true);
        }

        if (setting == PLMPaperHub::WindowState) {
            error = setSetting(projectId, "m_window_state", value, true);
        }

        if (setting == PLMPaperHub::SettingDate) {
            error = setSetting(projectId, "dt_updated", value, true);
        }

        if (setting == PLMPaperHub::Minimap) {
            if (stack == PLMPaperHub::Zero) {
                error = setSetting(projectId, "b_stack_0_map", value, true);
            }

            if (stack == PLMPaperHub::One) {
                error = setSetting(projectId, "b_stack_1_map", value, true);
            }
        }

        if (setting == PLMPaperHub::Fit) {
            if (stack == PLMPaperHub::Zero) {
                error = setSetting(projectId, "b_stack_0_fit", value, true);
            }

            if (stack == PLMPaperHub::One) {
                error = setSetting(projectId, "b_stack_1_fit", value, true);
            }
        }

        if (setting == PLMPaperHub::SpellCheck) {
            if (stack == PLMPaperHub::Zero) {
                error = setSetting(projectId, "b_stack_0_spellcheck", value, true);
            }

            if (stack == PLMPaperHub::One) {
                error = setSetting(projectId, "b_stack_1_spellcheck", value, true);
            }
        }

        if (setting == PLMPaperHub::StackState) {
            if (stack == PLMPaperHub::Zero) {
                error = setSetting(projectId, "b_stack_0_state", value, true);
            }

            if (stack == PLMPaperHub::One) {
                error = setSetting(projectId, "b_stack_1_state", value, true);
            }
        }
    }

    IFOK(error) {
        emit settings_settingChanged(stack, setting, value);
    }
    return error;
}

// -----------------------------------------------------------------------------

QVariant PLMPaperHub::settings_getStackSetting(PLMPaperHub::Stack   stack,
                                               PLMPaperHub::Setting setting) const
{
    int projectId = plmProjectManager->projectIdList().first();
    QVariant value;

    if (setting == PLMPaperHub::SplitterState) {
        value = getSetting(projectId, "m_splitter_state");
    }

    if (setting == PLMPaperHub::WindowState) {
        value = getSetting(projectId, "m_window_state");
    }

    if (setting == PLMPaperHub::SettingDate) {
        value = getSetting(projectId, "dt_updated");
    }

    if (setting == PLMPaperHub::Minimap) {
        if (stack == PLMPaperHub::Zero) {
            value = getSetting(projectId, "b_stack_0_map");
        }

        if (stack == PLMPaperHub::One) {
            value = getSetting(projectId, "b_stack_1_map");
        }
    }

    if (setting == PLMPaperHub::Fit) {
        if (stack == PLMPaperHub::Zero) {
            value = getSetting(projectId, "b_stack_0_fit");
        }

        if (stack == PLMPaperHub::One) {
            value = getSetting(projectId, "b_stack_1_fit");
        }
    }

    if (setting == PLMPaperHub::SpellCheck) {
        if (stack == PLMPaperHub::Zero) {
            value = getSetting(projectId, "b_stack_0_spellcheck");
        }

        if (stack == PLMPaperHub::One) {
            value = getSetting(projectId, "b_stack_1_spellcheck");
        }
    }

    if (setting == PLMPaperHub::StackState) {
        if (stack == PLMPaperHub::Zero) {
            value = getSetting(projectId, "b_stack_0_state");
        }

        if (stack == PLMPaperHub::One) {
            value = getSetting(projectId, "b_stack_1_state");
        }
    }

    return value;
}

// -----------------------------------------------------------------------------

PLMError PLMPaperHub::settings_setDocSetting(int                           projectId,
                                             int                           paperId,
                                             PLMPaperHub::OpenedDocSetting setting,
                                             const QVariant              & value)
{
    PLMError error;

    if (setting == PLMPaperHub::StackNumber) {
        error = setDocSetting(projectId, paperId, "l_statck", value, true);
    }

    if (setting == PLMPaperHub::Hovering) {
        error = setDocSetting(projectId, paperId, "b_hovering", value, true);
    }

    if (setting == PLMPaperHub::Visible) {
        error = setDocSetting(projectId, paperId, "b_visible", value, true);
    }

    if (setting == PLMPaperHub::HasFocus) {
        error = setDocSetting(projectId, paperId, "b_has_focus", value, true);
    }

    if (setting == PLMPaperHub::CursorPosition) {
        error = setDocSetting(projectId, paperId, "l_cursor_pos", value, true);
    }

    if (setting == PLMPaperHub::HoveringGeometry) {
        error = setDocSetting(projectId, paperId, "m_geometry", value, true);
    }

    if (setting == PLMPaperHub::Date) {
        error = setDocSetting(projectId, paperId, "dt_updated", value, true);
    }

    IFOK(error) {
        emit settings_docSettingChanged(projectId, paperId, setting, value);
    }
    return error;
}

// -----------------------------------------------------------------------------

QVariant PLMPaperHub::settings_getDocSetting(int                           projectId,
                                             int                           paperId,
                                             PLMPaperHub::OpenedDocSetting setting) const
{
    QVariant value;

    if (setting == PLMPaperHub::StackNumber) {
        value = getDocSetting(projectId, paperId, "l_statck");
    }

    if (setting == PLMPaperHub::Hovering) {
        value = getDocSetting(projectId, paperId, "b_hovering");
    }

    if (setting == PLMPaperHub::Visible) {
        value = getDocSetting(projectId, paperId, "b_visible");
    }

    if (setting == PLMPaperHub::HasFocus) {
        value = getDocSetting(projectId, paperId, "b_has_focus");
    }

    if (setting == PLMPaperHub::CursorPosition) {
        value = getDocSetting(projectId, paperId, "l_cursor_pos");
    }

    if (setting == PLMPaperHub::HoveringGeometry) {
        value = getDocSetting(projectId, paperId, "m_geometry");
    }

    if (setting == PLMPaperHub::Date) {
        value = getDocSetting(projectId, paperId, "dt_updated");
    }

    return value;
}

#include "skrsearchnotelistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

SKRSearchNoteListProxyModel::SKRSearchNoteListProxyModel(QObject *parent) :
    QSortFilterProxyModel(parent),
    m_showTrashedFilter(true), m_showNotTrashedFilter(true), m_textFilter(""),
    m_projectIdFilter(-2), m_parentIdFilter(-2), m_showParentWhenParentIdFilter(false)

{
    this->setSourceModel(plmmodels->noteListModel());


    connect(
        plmdata->projectHub(),
        &PLMProjectHub::projectLoaded,
        this,
        &SKRSearchNoteListProxyModel::loadProjectSettings);
    connect(
        plmdata->projectHub(),
        &PLMProjectHub::projectToBeClosed,
        this,
        &SKRSearchNoteListProxyModel::saveProjectSettings,
        Qt::DirectConnection);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->clearFilters();
    });


}

Qt::ItemFlags SKRSearchNoteListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------


QVariant SKRSearchNoteListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();


    PLMNoteItem *item = static_cast<PLMNoteItem *>(sourceIndex.internalPointer());
    int projectId     = item->projectId();
    int paperId       = item->paperId();


    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMNoteItem::Roles::NameRole).toString();
    }

    if ((role == Qt::CheckStateRole) && (col == 0)) {
        return m_checkedIdsHash.value(paperId, Qt::Unchecked);
    }


    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool SKRSearchNoteListProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                          int role)
{
    QModelIndex sourceIndex = this->mapToSource(index);


    PLMNoteItem *item =
        static_cast<PLMNoteItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::NameRole);
        }
    }


    if ((role == Qt::CheckStateRole) && (sourceIndex.column() == 0)) {
        int paperId               = item->paperId();
        Qt::CheckState checkState = static_cast<Qt::CheckState>(value.toInt());
        m_checkedIdsHash.insert(paperId, checkState);

        if ((checkState == Qt::Checked) || (checkState == Qt::Unchecked)) {
            this->checkStateOfAllChildren(item->projectId(), item->paperId(), checkState);
        }
        this->determineCheckStateOfAllAncestors(item->projectId(),
                                                item->paperId(),
                                                checkState);

    }

    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::checkStateOfAllChildren(int            projectId,
                                                          int            paperId,
                                                          Qt::CheckState checkState) {
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    QList<int> childrenIdsList = this->getChildrenList(projectId,
                                                       paperId,
                                                       m_showTrashedFilter,
                                                       m_showNotTrashedFilter);

    for (int childId : childrenIdsList) {
        m_checkedIdsHash.insert(childId, checkState);
        QModelIndex modelIndex = model->getModelIndex(projectId, childId).first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::determineCheckStateOfAllAncestors(
    int            projectId,
    int            paperId,
    Qt::CheckState checkState)
{
    PLMNoteListModel *model    = static_cast<PLMNoteListModel *>(this->sourceModel());
    QList<int> ancestorsIdsList = this->getAncestorsList(projectId,
                                                         paperId,
                                                         m_showTrashedFilter,
                                                         m_showNotTrashedFilter);

    if(ancestorsIdsList.isEmpty()){
        return;
    }

    //default state :
    Qt::CheckState ancestorCheckState = Qt::Unchecked;

    if (checkState == Qt::Unchecked) {
        // see if the direct ancestor has all its children unchecked
        QList<int> siblingsIdsList = this->getSiblingsList(projectId,
                                                           paperId,
                                                           m_showTrashedFilter,
                                                           m_showNotTrashedFilter);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::PartiallyChecked;
        }
        else {
            bool areNoneOfTheSiblingsChecked         = false;
            bool areAtLeastOneSiblingChecked   = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if (state == Qt::Checked){
                    areAtLeastOneSiblingChecked = true;
                }
                if(state == Qt::PartiallyChecked){
                    areAtLeastOneSiblingPartiallyChecked = true;
                }
                if (state == Qt::Unchecked){
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areNoneOfTheSiblingsChecked = !areAtLeastOneSiblingChecked && !areAtLeastOneSiblingPartiallyChecked;

            if (areAtLeastOneSiblingChecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }
            if (areAtLeastOneSiblingPartiallyChecked) {
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if(areNoneOfTheSiblingsChecked){
                ancestorCheckState = Qt::PartiallyChecked;
            }
        }
    }
    else if (checkState == Qt::Checked) {
        // see if the direct ancestor has all its children checked


        QList<int> siblingsIdsList = this->getSiblingsList(projectId,
                                                           paperId,
                                                           m_showTrashedFilter,
                                                           m_showNotTrashedFilter);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::Checked;
        }
        else {
            bool areAllSiblingsChecked         = false;
            bool areAtLeastOneSiblingChecked   = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if (state == Qt::Checked){
                    areAtLeastOneSiblingChecked = true;
                }
                if(state == Qt::PartiallyChecked){
                    areAtLeastOneSiblingPartiallyChecked = true;
                }
                if(state == Qt::Unchecked){
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areAllSiblingsChecked = !areAtLeastOneSiblingUnchecked && !areAtLeastOneSiblingPartiallyChecked;

            if (areAllSiblingsChecked) {
                ancestorCheckState = Qt::Checked;
            }
            else if (areAtLeastOneSiblingUnchecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if (areAtLeastOneSiblingPartiallyChecked) {
                ancestorCheckState = Qt::PartiallyChecked;
            }

        }
    }
    else if (checkState == Qt::PartiallyChecked) {
        ancestorCheckState = Qt::PartiallyChecked;
    }

    if (!m_paperIdListFilter.isEmpty()) {
        QList<int> newList;

        for (int id : ancestorsIdsList) {
            if (m_paperIdListFilter.contains(id)) {
                newList.append(id);
            }
        }

        ancestorsIdsList = newList;
    }

    if(ancestorsIdsList.empty()){
        return;
    }

    //for (int ancestorId : ancestorsIdsList) {
        m_checkedIdsHash.insert(ancestorsIdsList.first(), ancestorCheckState);
        QModelIndex modelIndex = model->getModelIndex(projectId, ancestorsIdsList.first()).first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    //}
        determineCheckStateOfAllAncestors(m_projectIdFilter, ancestorsIdsList.first(), ancestorCheckState);

}

// --------------------------------------------------------------

QList<int>SKRSearchNoteListProxyModel::getCheckedIdsList() {
    QList<int> result;

    QHash<int, Qt::CheckState>::const_iterator i = m_checkedIdsHash.constBegin();

    while (i != m_checkedIdsHash.constEnd()) {
        if ((i.value() == Qt::Checked) || (i.value() == Qt::PartiallyChecked)) {
            result << i.key();
            ++i;
        }
    }

    return result;
}

// --------------------------------------------------------------


void SKRSearchNoteListProxyModel::setCheckedIdsList(const QList<int> checkedIdsList){
    PLMNoteListModel *model    = static_cast<PLMNoteListModel *>(this->sourceModel());


    this->clearCheckedList();


    if(checkedIdsList.isEmpty()){
        return;
    }

    for(int i = checkedIdsList.count() - 1 ; i >= 0 ;  i--){

        int paperId  = checkedIdsList[i];


        if(!this->hasChildren(m_projectIdFilter, paperId)){
            m_checkedIdsHash.insert(paperId, Qt::Checked);
        }
        else { // has children so verify if there is one checked or partially checked



            Qt::CheckState finalState = Qt::Unchecked;
            QList<int> childrenIdsList = this->getChildrenList(m_projectIdFilter,
                                                               paperId,
                                                               m_showTrashedFilter,
                                                               m_showNotTrashedFilter);


                bool areAllChildrenChecked         = false;
                bool areAtLeastOneChildChecked   = false;
                bool areAtLeastOneChildPartiallyChecked   = false;
                bool areAtLeastOneChildUnchecked = false;

                for (int childId : childrenIdsList) {
                    Qt::CheckState state = m_checkedIdsHash.value(childId, Qt::Unchecked);

                    if (state == Qt::Checked) {
                        areAtLeastOneChildChecked = true;
                    }
                    else if(state == Qt::PartiallyChecked) {
                            areAtLeastOneChildPartiallyChecked = true;
                        }
                    else {
                        areAtLeastOneChildUnchecked = true;
                    }
                }

                areAllChildrenChecked = !areAtLeastOneChildUnchecked && !areAtLeastOneChildPartiallyChecked;

                if (areAtLeastOneChildUnchecked) { // but this one
                    finalState = Qt::PartiallyChecked;
                }
                else if (areAllChildrenChecked) {
                    finalState = Qt::Checked;
                }
                else {
                    finalState = Qt::PartiallyChecked;
                }





            m_checkedIdsHash.insert(paperId, finalState);
        }

        //m_checkedIdsHash.insert(paperId, Qt::Checked);
        QModelIndexList modelIndexList = model->getModelIndex(m_projectIdFilter, paperId);
        if(modelIndexList.isEmpty()){
            continue;
        }
        QModelIndex modelIndex = modelIndexList.first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }


}
// --------------------------------------------------------------

///
/// \brief SKRSearchNoteListProxyModel::findIdsTrashedAtTheSameTimeThan
/// \param projectId
/// \param paperId
/// \return
/// more or less 1 second between items' trashed dates
QList<int> SKRSearchNoteListProxyModel::findIdsTrashedAtTheSameTimeThan(int projectId, int paperId){


    QList<int> result;

    QDateTime parentDate = plmdata->noteHub()->getTrashedDate(projectId, paperId);


    if(!m_paperIdListFilter.isEmpty()){

        for(int id : m_paperIdListFilter){

            QDateTime childDate = plmdata->noteHub()->getTrashedDate(projectId, id);

            if(qAbs(childDate.secsTo(parentDate)) < 1){
                result.append(id);
            }


        }


    }
    else {
        QList<int> childrenIds  = this->getChildrenList(projectId, paperId, m_showTrashedFilter, m_showNotTrashedFilter);
        for(int id : childrenIds){

            QDateTime childDate = plmdata->noteHub()->getTrashedDate(projectId, id);

            if(qAbs(childDate.secsTo(parentDate)) < 1){
                result.append(id);
            }


        }
    }



    return result;

}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::deleteDefinitively(int projectId, int paperId)
{
    plmdata->noteHub()->removePaper(projectId, paperId);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::setParentIdFilter(int projectIdfilter)
{

    m_parentIdFilter = projectIdfilter;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();

}

// --------------------------------------------------------------


void SKRSearchNoteListProxyModel::setShowParentWhenParentIdFilter(bool showParent)
{
    m_showParentWhenParentIdFilter = showParent;
    emit showParentWhenParentIdFilterChanged(showParent);

    if(m_parentIdFilter != -2){

        this->invalidateFilter();
    }

}

// --------------------------------------------------------------


bool SKRSearchNoteListProxyModel::filterAcceptsRow(int                sourceRow,
                                                   const QModelIndex& sourceParent) const
{
    bool result = true;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    PLMNoteItem *item       = static_cast<PLMNoteItem *>(index.internalPointer());
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    // avoid project item
    if (item->data(PLMNoteItem::Roles::PaperIdRole).toInt() == -1) {
        result = false;
    }

    // project filtering :
    if (result &&
        (item->data(PLMNoteItem::Roles::ProjectIdRole).toInt() == m_projectIdFilter)) {
        result = true;
    }
    else if (result) {
        result = false;
    }

    // trashed filtering :
    if (result && (item->data(PLMNoteItem::Roles::TrashedRole).toBool() == true)) {
        result = m_showTrashedFilter;
    }

    // 'not trashed' filtering :
    if (result && (item->data(PLMNoteItem::Roles::TrashedRole).toBool() == false)) {
        result = m_showNotTrashedFilter;
    }


    if (result &&
        item->data(PLMNoteItem::Roles::NameRole).toString().contains(m_textFilter,
                                                                     Qt::CaseInsensitive))
    {
        result = true;
    }
    else if (result) {
        result = false;
    }


    // paperIdListFiltering :
    if (!m_paperIdListFilter.isEmpty()) {
        if (result &&
            m_paperIdListFilter.contains(item->data(
                                             PLMNoteItem::Roles::PaperIdRole).toInt()))
        {
            result = true;
        }
        else if (result) {
            result = false;
        }
    }


    // parentId filtering :
    if (result && m_parentIdFilter != -2) {

        if(m_showParentWhenParentIdFilter && m_parentIdFilter == item->data(PLMNoteItem::Roles::PaperIdRole).toInt()){
            result = true;
        }
        else {

            PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
            PLMNoteItem *parentItem = model->getParentNoteItem(item);

            if (parentItem) {
                if (parentItem->paperId() == m_parentIdFilter) {
                    result = true;
                }
                else{
                    result = false;
                }
            }
        }
    }

    return result;
}

void SKRSearchNoteListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    m_parentIdFilter = -2;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::clearFilters()
{
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);

    m_showTrashedFilter = true;
    emit showTrashedFilterChanged(m_showTrashedFilter);

    m_showNotTrashedFilter = true;
    emit showNotTrashedFilterChanged(m_showNotTrashedFilter);

    m_textFilter = "";
    emit textFilterChanged(m_textFilter);

    m_parentIdFilter = -2;
    emit parentIdFilterChanged(m_parentIdFilter);

    m_showParentWhenParentIdFilter = false;
    emit showParentWhenParentIdFilterChanged(m_showParentWhenParentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::clearCheckedList()
{
    m_checkedIdsHash.clear();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::checkAll()
{
    PLMNoteListModel *model    = static_cast<PLMNoteListModel *>(this->sourceModel());

    if(m_paperIdListFilter.isEmpty()){

        QList<int> filteredIdsList;
        QList<int> allIdsList = plmdata->noteHub()->getAllIds(m_projectIdFilter);

        for (int id : allIdsList) {
            bool isTrashed = plmdata->noteHub()->getTrashed(m_projectIdFilter, id);

            if (m_showTrashedFilter && isTrashed) {
                filteredIdsList.append(id);
            }

            if (m_showNotTrashedFilter && !isTrashed) {
                filteredIdsList.append(id);
            }
        }

        for (int id : filteredIdsList) {
            m_checkedIdsHash.insert(id, Qt::Checked);

            QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, id).first();

            emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);
        }

    }
    else {
        for(int paperId : m_paperIdListFilter){
            m_checkedIdsHash.insert(paperId, Qt::Checked);

            QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, paperId).first();

            emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);

        }




    }

}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::checkNone()
{
    PLMNoteListModel *model    = static_cast<PLMNoteListModel *>(this->sourceModel());
    QHash<int, Qt::CheckState> checkedIdsHash(m_checkedIdsHash);

    this->clearCheckedList();

    for(int paperId : checkedIdsHash.keys()){
        QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, paperId).first();
        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }

}
// --------------------------------------------------------------


PLMNoteItem * SKRSearchNoteListProxyModel::getItem(int projectId, int paperId)
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    PLMNoteItem *item = model->getItem(projectId, paperId);

    return item;
}

// --------------------------------------------------------------


void SKRSearchNoteListProxyModel::loadProjectSettings(int projectId)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    int writeCurrentParent = settings.value("noteCurrentParent",
    // 0).toInt();
    //    this->setParentFilter(projectId, noteCurrentParent);
    settings.endGroup();
}

// --------------------------------------------------------------


void SKRSearchNoteListProxyModel::saveProjectSettings(int projectId)
{
    if (m_projectIdFilter != projectId) {
        return;
    }

    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    settings.setValue("noteCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::setPaperIdListFilter(
    const QList<int>& paperIdListFilter)
{
    m_paperIdListFilter = paperIdListFilter;

    emit paperIdListFilterChanged(paperIdListFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::setCurrentPaperId(int projectId, int paperId)
{
    if (projectId == -2) {
        return;
    }

    if ((paperId == -2) && (projectId != -2)) {
        return;
    }


    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    PLMNoteItem *item       = this->getItem(projectId, paperId);

    if (!item) {
        paperId = plmdata->noteHub()->getTopPaperId(projectId);
        item    = this->getItem(projectId, paperId);
    }

    this->setForcedCurrentIndex(projectId, paperId);
}

void SKRSearchNoteListProxyModel::setTextFilter(const QString& value)
{
    m_textFilter = value;
    emit textFilterChanged(value);

    this->invalidateFilter();
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchNoteListProxyModel::getChildrenList(int  projectId,
                                                       int  paperId,
                                                       bool getTrashed,
                                                       bool getNotTrashed)
{
    QList<int> resultChildrenIdList;

    QList<int> allChildrenIdList = plmdata->noteHub()->getAllChildren(projectId, paperId);

    for (int childId : allChildrenIdList) {
        bool isTrashed = plmdata->noteHub()->getTrashed(projectId, childId);

        if (getTrashed && isTrashed) {
            resultChildrenIdList.append(childId);
        }

        if (getNotTrashed && !isTrashed) {
            resultChildrenIdList.append(childId);
        }
    }

    return resultChildrenIdList;
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchNoteListProxyModel::getAncestorsList(int  projectId,
                                                        int  paperId,
                                                        bool getTrashed,
                                                        bool getNotTrashed) {
    QList<int> resultAllAncestorsIdList;

    QList<int> allAncestorsIdList =
        plmdata->noteHub()->getAllAncestors(projectId, paperId);

    for (int ancestorId : allAncestorsIdList) {
        bool isTrashed = plmdata->noteHub()->getTrashed(projectId, ancestorId);

        if (getTrashed && isTrashed) {
            resultAllAncestorsIdList.append(ancestorId);
        }

        if (getNotTrashed && !isTrashed) {
            resultAllAncestorsIdList.append(ancestorId);
        }
    }

    return resultAllAncestorsIdList;
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchNoteListProxyModel::getSiblingsList(int  projectId,
                                                       int  paperId,
                                                       bool getTrashed,
                                                       bool getNotTrashed) {
    QList<int> resultAllSiblingsIdList;

    QList<int> allSiblingsIdList =
        plmdata->noteHub()->getAllSiblings(projectId, paperId);

    for (int ancestorId : allSiblingsIdList) {
        bool isTrashed = plmdata->noteHub()->getTrashed(projectId, ancestorId);

        if (getTrashed && isTrashed) {
            resultAllSiblingsIdList.append(ancestorId);
        }

        if (getNotTrashed && !isTrashed) {
            resultAllSiblingsIdList.append(ancestorId);
        }
    }

    return resultAllSiblingsIdList;
}

// ----------------------------------------------------------------------------------


void SKRSearchNoteListProxyModel::setShowNotTrashedFilter(bool showNotTrashedFilter)
{
    m_showNotTrashedFilter = showNotTrashedFilter;

    emit showNotTrashedFilterChanged(showNotTrashedFilter);

    this->invalidateFilter();
}

void SKRSearchNoteListProxyModel::setShowTrashedFilter(bool showTrashedFilter)
{
    m_showTrashedFilter = showTrashedFilter;

    emit showTrashedFilterChanged(showTrashedFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

bool SKRSearchNoteListProxyModel::hasChildren(int projectId, int paperId)
{
    return plmdata->noteHub()->hasChildren(projectId,
                                           paperId,
                                           m_showTrashedFilter,
                                           m_showNotTrashedFilter);
}

// --------------------------------------------------------------

int SKRSearchNoteListProxyModel::findVisualIndex(int projectId, int paperId)
{
    int rowCount = this->rowCount(QModelIndex());

    int result = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        PLMNoteItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           PLMNoteItem::Roles::PaperIdRole).toInt() == paperId)) {
            result = i;
            break;
        }
    }
    return result;
}

// --------------------------------------------------------------
QString SKRSearchNoteListProxyModel::getItemName(int projectId, int paperId)
{
    // qDebug() << "getItemName" << projectId << paperId;
    if ((projectId == -2) || (paperId == -2)) {
        return "";
    }
    QString name = "";

    if (paperId == -1) {
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else {
        PLMNoteItem *item = this->getItem(projectId, paperId);

        if (item) {
            name = item->name();
        }
        else {
            qDebug() << this->metaObject()->className() << "no valid item found";
            name = "";
        }
    }

    return name;
}

// --------------------------------------------------------------

int SKRSearchNoteListProxyModel::getItemIndent(int projectId, int paperId)
{
    if ((projectId == -2) || (paperId == -2)) {
        return -2;
    }
    int indent = -2;

    if (paperId == -1) {
        indent = -1;
    }
    else {
        PLMNoteItem *item = this->getItem(projectId, paperId);

        if (item) {
            indent = item->indent();
        }
        else {
            qDebug() << this->metaObject()->className() << "no valid item found";
            indent = -2;
        }
    }

    return indent;
}

// -----------------------------------------------------------------

void SKRSearchNoteListProxyModel::addChildItem(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->noteHub()->addChildPaper(projectId, parentPaperId);
    this->setForcedCurrentIndex(visualIndex);
}

// -----------------------------------------------------------------

void SKRSearchNoteListProxyModel::addItemAbove(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->noteHub()->addPaperAbove(projectId, parentPaperId);
    this->setForcedCurrentIndex(visualIndex);
}


// -----------------------------------------------------------------

void SKRSearchNoteListProxyModel::addItemBelow(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->noteHub()->addPaperBelow(projectId, parentPaperId);
    this->setForcedCurrentIndex(visualIndex);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::moveUp(int projectId, int paperId, int visualIndex)
{
    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->noteHub()->movePaperUp(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex - 1);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::moveDown(int projectId, int paperId, int visualIndex)
{
    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->noteHub()->movePaperDown(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex + 1);
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::trashItemWithChildren(int projectId, int paperId)
{

    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }

    plmdata->noteHub()->setTrashedWithChildren(projectId, paperId, true);

}
// --------------------------------------------------------------

QHash<int, QByteArray>SKRSearchNoteListProxyModel::roleNames() const {
    QHash<int, QByteArray> roles = QSortFilterProxyModel::roleNames();

    roles[Qt::CheckStateRole] = "checkState";

    return roles;
}

#include "skrsearchnotelistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

SKRSearchNoteListProxyModel::SKRSearchNoteListProxyModel(QObject *parent) :
    QSortFilterProxyModel(parent),
    m_showTrashedFilter(true), m_showNotTrashedFilter(true), m_textFilter(""),
    m_projectIdFilter(-2)

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

        emit checkedIdsListChanged();
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
    PLMSheetListModel *model    = static_cast<PLMSheetListModel *>(this->sourceModel());
    QList<int> ancestorsIdsList = this->getAncestorsList(projectId,
                                                         paperId,
                                                         m_showTrashedFilter,
                                                         m_showNotTrashedFilter);
    Qt::CheckState ancestorCheckState;

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
            bool areAllSiblingsChecked         = false;
            bool areAtLeastOneSiblingChecked   = false;
            bool areAtLeastOneSiblingUnchecked = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if ((state == Qt::Checked) || (state == Qt::PartiallyChecked)) {
                    areAtLeastOneSiblingChecked = true;
                }
                else {
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areAllSiblingsChecked = !areAtLeastOneSiblingUnchecked;

            if (areAtLeastOneSiblingChecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if (!areAllSiblingsChecked) {
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
            bool areAtLeastOneSiblingUnchecked = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if ((state == Qt::Checked) || (state == Qt::PartiallyChecked)) {
                    areAtLeastOneSiblingChecked = true;
                }
                else {
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areAllSiblingsChecked = !areAtLeastOneSiblingUnchecked;

            if (areAtLeastOneSiblingUnchecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if (areAllSiblingsChecked) {
                ancestorCheckState = Qt::Checked;
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


    for (int ancestorId : ancestorsIdsList) {
        m_checkedIdsHash.insert(ancestorId, ancestorCheckState);
        QModelIndex modelIndex = model->getModelIndex(projectId, ancestorId).first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
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

    return result;
}

void SKRSearchNoteListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

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


    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchNoteListProxyModel::clearCheckedList()
{
    m_checkedIdsHash.clear();
    emit checkedIdsListChanged();
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

// --------------------------------------------------------------

QHash<int, QByteArray>SKRSearchNoteListProxyModel::roleNames() const {
    QHash<int, QByteArray> roles = QSortFilterProxyModel::roleNames();

    roles[Qt::CheckStateRole] = "checkState";

    return roles;
}

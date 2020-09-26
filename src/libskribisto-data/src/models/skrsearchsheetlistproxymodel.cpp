#include "skrsearchsheetlistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

SKRSearchSheetListProxyModel::SKRSearchSheetListProxyModel(QObject *parent) :
    QSortFilterProxyModel(parent),
    m_showTrashedFilter(true), m_showNotTrashedFilter(true), m_textFilter(""),
    m_projectIdFilter(-2)
{
    this->setSourceModel(plmmodels->sheetListModel());


    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectLoaded,
                                                                  this,
                                                                        &SKRSearchSheetListProxyModel::loadProjectSettings);
    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectToBeClosed,
                                                                  this,
                                                                        &SKRSearchSheetListProxyModel::saveProjectSettings,
        Qt::DirectConnection);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->clearFilters();
    });
}

Qt::ItemFlags SKRSearchSheetListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------


QVariant SKRSearchSheetListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMSheetItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool SKRSearchSheetListProxyModel::setData(const QModelIndex& index,
                                           const QVariant   & value,
                                           int                role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    PLMSheetItem *item =
        static_cast<PLMSheetItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::NameRole);
        }
    }

    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------


bool SKRSearchSheetListProxyModel::filterAcceptsRow(int                sourceRow,
                                                    const QModelIndex& sourceParent) const
{
    bool result = true;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    PLMSheetItem *item       = static_cast<PLMSheetItem *>(index.internalPointer());
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

    // avoid project item
    if (item->data(PLMSheetItem::Roles::PaperIdRole).toInt() == -1) {
        result = false;
    }

    // project filtering :
    if (result &&
        (item->data(PLMSheetItem::Roles::ProjectIdRole).toInt() == m_projectIdFilter)) {
        result = true;
    }
    else if (result) {
        result = false;
    }

    // trashed filtering :
    if (result && (item->data(PLMSheetItem::Roles::TrashedRole).toBool() == true)) {
        QString string = item->data(PLMSheetItem::Roles::NameRole).toString();
        result = m_showTrashedFilter;
    }

    // 'not trashed' filtering :
    if (result && (item->data(PLMSheetItem::Roles::TrashedRole).toBool() == false)) {
        QString string = item->data(PLMSheetItem::Roles::NameRole).toString();
        result = m_showNotTrashedFilter;
    }

    // text filtering :
    if (result &&
        item->data(PLMSheetItem::Roles::NameRole).toString().contains(m_textFilter,
                                                                      Qt::CaseInsensitive))
    {
        result = true;
    }
    else if (result) {
        result = false;
    }


    return result;
}

void SKRSearchSheetListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchSheetListProxyModel::clearFilters()
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

// --------------------------------------------------------------

PLMSheetItem * SKRSearchSheetListProxyModel::getItem(int projectId, int paperId)
{
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

    PLMSheetItem *item = model->getItem(projectId, paperId);

    return item;
}

// --------------------------------------------------------------


void SKRSearchSheetListProxyModel::loadProjectSettings(int projectId)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    int writeCurrentParent = settings.value("sheetCurrentParent",
    // 0).toInt();
    //    this->setParentFilter(projectId, sheetCurrentParent);
    settings.endGroup();
}

// --------------------------------------------------------------


void SKRSearchSheetListProxyModel::saveProjectSettings(int projectId)
{
    if (m_projectIdFilter != projectId) {
        return;
    }

    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    settings.setValue("sheetCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

void SKRSearchSheetListProxyModel::setTextFilter(const QString& value)
{
    m_textFilter = value;
    emit textFilterChanged(value);

    this->invalidateFilter();
}

void SKRSearchSheetListProxyModel::setShowNotTrashedFilter(bool showNotTrashedFilter)
{
    m_showNotTrashedFilter = showNotTrashedFilter;

    emit showNotTrashedFilterChanged(showNotTrashedFilter);

    this->invalidateFilter();
}

void SKRSearchSheetListProxyModel::setShowTrashedFilter(bool showTrashedFilter)
{
    m_showTrashedFilter = showTrashedFilter;

    emit showTrashedFilterChanged(showTrashedFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchSheetListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchSheetListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchSheetListProxyModel::setCurrentPaperId(int projectId, int paperId)
{
    if (projectId == -2) {
        return;
    }

    if ((paperId == -2) && (projectId != -2)) {
        return;
    }


    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());
    PLMSheetItem *item       = this->getItem(projectId, paperId);

    if (!item) {
        paperId = plmdata->sheetHub()->getTopPaperId(projectId);
    }

    this->setForcedCurrentIndex(projectId, paperId);
}

// --------------------------------------------------------------

bool SKRSearchSheetListProxyModel::hasChildren(int projectId, int paperId)
{
    return plmdata->sheetHub()->hasChildren(projectId, paperId);
}

// --------------------------------------------------------------

int SKRSearchSheetListProxyModel::findVisualIndex(int projectId, int paperId)
{
    int rowCount = this->rowCount(QModelIndex());

    int result = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        PLMSheetItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           PLMSheetItem::Roles::PaperIdRole).toInt() == paperId)) {
            result = i;
            break;
        }
    }
    return result;
}

// --------------------------------------------------------------
QString SKRSearchSheetListProxyModel::getItemName(int projectId, int paperId)
{
    // qDebug() << "getItemName" << projectId << paperId;
    if ((projectId == -2) || (paperId == -2)) {
        return "";
    }
    QString name = "";

    if ((paperId == 0) && (plmdata->projectHub()->getProjectIdList().count() <= 1)) {
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else {
        PLMSheetItem *item = this->getItem(projectId, paperId);

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

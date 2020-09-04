#include "skrsearchnotelistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

SKRSearchNoteListProxyModel::SKRSearchNoteListProxyModel(QObject *parent) : QSortFilterProxyModel(parent),
    m_showDeletedFilter(true), m_showNotDeletedFilter(true), m_textFilter(""), m_projectIdFilter(-2)

{
    this->setSourceModel(plmmodels->noteListModel());



    connect(plmdata->projectHub(), &PLMProjectHub::projectLoaded, this, &SKRSearchNoteListProxyModel::loadProjectSettings);
    connect(plmdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, &SKRSearchNoteListProxyModel::saveProjectSettings, Qt::DirectConnection);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this](){
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

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMNoteItem::Roles::NameRole).toString();
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

    return QSortFilterProxyModel::setData(index, value, role);
}

//--------------------------------------------------------------




bool SKRSearchNoteListProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
{
    bool result = true;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);
    if (!index.isValid()){
        return false;
    }
    PLMNoteItem *item = static_cast<PLMNoteItem *>(index.internalPointer());
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    // avoid project item
    if (item->data(PLMNoteItem::Roles::PaperIdRole).toInt() == -1){
        result = false;
    }

    // project filtering :
    if(result && item->data(PLMNoteItem::Roles::ProjectIdRole).toInt() == m_projectIdFilter){
        result = true;
    }
    else if(result){
        result = false;
    }

    // deleted filtering :
    if(result && item->data(PLMNoteItem::Roles::DeletedRole).toBool() == true){
        result = m_showDeletedFilter;
    }

    // 'not deleted' filtering :
    if(result && item->data(PLMNoteItem::Roles::DeletedRole).toBool() == false){
        result = m_showNotDeletedFilter;
    }


    if(result && item->data(PLMNoteItem::Roles::NameRole).toString().contains(m_textFilter, Qt::CaseInsensitive)){
        result = true;
    }
    else if(result){
        result = false;
    }



    return result;
}

void SKRSearchNoteListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);
    this->invalidate();
}

//--------------------------------------------------------------

void SKRSearchNoteListProxyModel::clearFilters()
{
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);
    m_showDeletedFilter = true;
    emit showDeletedFilterChanged(m_showDeletedFilter);
    m_showNotDeletedFilter = true;
    emit showNotDeletedFilterChanged(m_showNotDeletedFilter);
    m_textFilter = "";
    emit textFilterChanged(m_textFilter);


    this->invalidate();

}

//--------------------------------------------------------------

//--------------------------------------------------------------

PLMNoteItem *SKRSearchNoteListProxyModel::getItem(int projectId, int paperId)
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    PLMNoteItem *item = model->getItem(projectId, paperId);
    return item;
}



//--------------------------------------------------------------


void SKRSearchNoteListProxyModel::loadProjectSettings(int projectId)
{
    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);
    //    int writeCurrentParent = settings.value("noteCurrentParent", 0).toInt();
    //    this->setParentFilter(projectId, noteCurrentParent);
    settings.endGroup();

}
//--------------------------------------------------------------


void SKRSearchNoteListProxyModel::saveProjectSettings(int projectId)
{
    if(m_projectIdFilter != projectId){
        return;
    }

    QString unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;
    settings.beginGroup("project_" + unique_identifier);
    //    settings.setValue("noteCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

//--------------------------------------------------------------

void SKRSearchNoteListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

//--------------------------------------------------------------

void SKRSearchNoteListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);
    setForcedCurrentIndex(forcedCurrentIndex);
}

//--------------------------------------------------------------

void SKRSearchNoteListProxyModel::setCurrentPaperId(int projectId, int paperId)
{
    if(projectId == -2){
        return;
    }
    if (paperId == -2 && projectId != -2){
        return;
    }


    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    PLMNoteItem *item = this->getItem(projectId, paperId);
    if(!item){
        paperId = plmdata->noteHub()->getTopPaperId(projectId);
        item = this->getItem(projectId, paperId);
    }

    this->setForcedCurrentIndex(projectId, paperId);
}

void SKRSearchNoteListProxyModel::setTextFilter(const QString &value)
{
    m_textFilter = value;
    emit textFilterChanged(value);
    this->invalidate();
}

void SKRSearchNoteListProxyModel::setShowNotDeletedFilter(bool showNotDeletedFilter)
{
    m_showNotDeletedFilter = showNotDeletedFilter;

    emit showNotDeletedFilterChanged(showNotDeletedFilter);
    this->invalidate();
}

void SKRSearchNoteListProxyModel::setShowDeletedFilter(bool showDeletedFilter)
{
    m_showDeletedFilter = showDeletedFilter;

    emit showDeletedFilterChanged(showDeletedFilter);
    this->invalidate();
}


//--------------------------------------------------------------

bool SKRSearchNoteListProxyModel::hasChildren(int projectId, int paperId)
{
    return plmdata->noteHub()->hasChildren(projectId, paperId);
}

//--------------------------------------------------------------

int SKRSearchNoteListProxyModel::findVisualIndex(int projectId, int paperId)
{

    int rowCount = this->rowCount(QModelIndex());

    int result = -2;

    QModelIndex modelIndex;
    for(int i = 0; i < rowCount ; ++i){
        modelIndex = this->index(i, 0);
        if(this->data(modelIndex, PLMNoteItem::Roles::ProjectIdRole).toInt() == projectId
                && this->data(modelIndex, PLMNoteItem::Roles::PaperIdRole).toInt() == paperId){
            result = i;
            break;
        }

    }
    return result;
}

//--------------------------------------------------------------
QString SKRSearchNoteListProxyModel::getItemName(int projectId, int paperId)
{
    //qDebug() << "getItemName" << projectId << paperId;
    if(projectId == -2 || paperId == -2){
        return "";
    }
    QString name = "";
    if(paperId == 0 && plmdata->projectHub()->getProjectIdList().count() <= 1){
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else{
        PLMNoteItem *item = this->getItem(projectId, paperId);
        if(item){
            name = item->name();
        }
        else {
            qDebug() << this->metaObject()->className() << "no valid item found";
            name = "";
        }
    }

    return name;
}

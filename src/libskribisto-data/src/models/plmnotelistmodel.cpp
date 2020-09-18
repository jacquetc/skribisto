#include "plmnotelistmodel.h"

PLMNoteListModel::PLMNoteListModel(QObject *parent)
    : QAbstractListModel(parent), m_headerData(QVariant())
{

    m_rootItem = new PLMNoteItem();
    m_rootItem->setIsRootItem();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMNoteListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMNoteListModel::populate);


    connect(plmdata->noteHub(),
            &PLMNoteHub::paperAdded,
            this,
            &PLMNoteListModel::refreshAfterDataAddition);

    connect(plmdata->noteHub(),
            &PLMNoteHub::paperMoved,
            this,
            &PLMNoteListModel::refreshAfterDataMove);

    connect(plmdata->sheetHub(),
            &PLMNoteHub::deletedChanged, // careful, paper is deleted = true, not a true removal
            this,
            &PLMNoteListModel::refreshAfterDeletedStateChanged);

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectIsBackupChanged,
            this,
            &PLMNoteListModel::refreshAfterProjectIsBackupChanged);


    this->connectToPLMDataSignals();
}

QVariant PLMNoteListModel::headerData(int section, Qt::Orientation orientation,
                                       int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool PLMNoteListModel::setHeaderData(int             section,
                                      Qt::Orientation orientation,
                                      const QVariant& value,
                                      int             role)
{
    if (value != headerData(section, orientation, role)) {
        m_headerData = value;
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}


QModelIndex PLMNoteListModel::index(int row, int column, const QModelIndex &parent) const
{
    if (column != 0) return QModelIndex();
    PLMNoteItem *parentItem;
    if (!parent.isValid()) parentItem = m_rootItem;

    PLMNoteItem *childItem = m_allNoteItems.at(row);
    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    }
    return QModelIndex();


}

int PLMNoteListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allNoteItems.count();
}

QVariant PLMNoteListModel::data(const QModelIndex& index, int role) const
{
    //    qDebug() << "checkIndex :"
    //             << (checkIndex(index,
    //                   QAbstractItemModel::CheckIndexOption::IndexIsValid
    //                   |
    //  QAbstractItemModel::CheckIndexOption::DoNotUseParent));
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid
                        |  QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (!index.isValid()) return QVariant();

    PLMNoteItem *item = static_cast<PLMNoteItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(PLMNoteItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(PLMNoteItem::Roles::NameRole);
    }


    if (role == PLMNoteItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::PaperIdRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::ContentDateRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::HasChildrenRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::DeletedRole) {
        return item->data(role);
    }

    if (role == PLMNoteItem::Roles::ProjectIsBackupRole) {
        return item->data(role);
    }
    return QVariant();
}

bool PLMNoteListModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        PLMNoteItem *item = static_cast<PLMNoteItem *>(index.internalPointer());
        int projectId      = item->projectId();
        int paperId        = item->paperId();
        PLMError error;

        this->disconnectFromPLMDataSignals();

        switch (role) {
        case PLMNoteItem::Roles::ProjectNameRole:
            error = plmdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case PLMNoteItem::Roles::ProjectIdRole:

            // useless
            break;

        case PLMNoteItem::Roles::PaperIdRole:

            // useless
            break;

        case PLMNoteItem::Roles::NameRole:
                error = plmdata->sheetHub()->setTitle(projectId, paperId, value.toString());


        case PLMNoteItem::Roles::LabelRole:
            error = plmdata->notePropertyHub()->setProperty(projectId, paperId,
                                                             "label", value.toString());
            break;

        case PLMNoteItem::Roles::IndentRole:
            error = plmdata->noteHub()->setIndent(projectId, paperId, value.toInt());
            break;

        case PLMNoteItem::Roles::SortOrderRole:
            error = plmdata->noteHub()->setSortOrder(projectId, paperId, value.toInt());            
            IFOKDO(error, plmdata->noteHub()->renumberSortOrders(projectId));
            for(PLMNoteItem *item : m_allNoteItems){
                item->invalidateData(role);
            }
            this->populate();

            break;

        case PLMNoteItem::Roles::DeletedRole:
            error = plmdata->noteHub()->setDeleted(projectId, paperId, value.toBool());
            break;

        case PLMNoteItem::Roles::CreationDateRole:
            error = plmdata->noteHub()->setCreationDate(projectId,
                                                         paperId,
                                                         value.toDateTime());
            break;

        case PLMNoteItem::Roles::UpdateDateRole:
            error = plmdata->noteHub()->setUpdateDate(projectId,
                                                       paperId,
                                                       value.toDateTime());
            break;

        case PLMNoteItem::Roles::ContentDateRole:
            error = plmdata->noteHub()->setContentDate(projectId,
                                                        paperId,
                                                        value.toDateTime());
            break;

        case PLMNoteItem::Roles::HasChildrenRole:
            //useless
            break;

        case PLMNoteItem::Roles::CharCountRole:
            error = plmdata->notePropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "char_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;

        case PLMNoteItem::Roles::WordCountRole:

            error = plmdata->notePropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "word_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;
        }


        this->connectToPLMDataSignals();

        if (!error.isSuccess()) {
            return false;
        }
        item->invalidateData(role);

        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags PLMNoteListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

bool PLMNoteListModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool PLMNoteListModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

QHash<int, QByteArray>PLMNoteListModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[PLMNoteItem::Roles::ProjectNameRole]  = "projectName";
    roles[PLMNoteItem::Roles::PaperIdRole]      = "paperId";
    roles[PLMNoteItem::Roles::ProjectIdRole]    = "projectId";
    roles[PLMNoteItem::Roles::NameRole]         = "name";
    roles[PLMNoteItem::Roles::LabelRole]          = "label";
    roles[PLMNoteItem::Roles::IndentRole]       = "indent";
    roles[PLMNoteItem::Roles::SortOrderRole]    = "sortOrder";
    roles[PLMNoteItem::Roles::CreationDateRole] = "creationDate";
    roles[PLMNoteItem::Roles::UpdateDateRole]   = "updateDate";
    roles[PLMNoteItem::Roles::ContentDateRole]  = "contentDate";
    roles[PLMNoteItem::Roles::HasChildrenRole]  = "hasChildren";
    roles[PLMNoteItem::Roles::DeletedRole]  = "deleted";
    roles[PLMNoteItem::Roles::WordCountRole]  = "wordCount";
    roles[PLMNoteItem::Roles::CharCountRole]  = "charCount";
    roles[PLMNoteItem::Roles::ProjectIsBackupRole] = "projectIsBackup";
    return roles;
}

void PLMNoteListModel::populate()
{
    this->beginResetModel();

    m_allNoteItems.clear();

    for(int projectId : plmdata->projectHub()->getProjectIdList()) {
            PLMNoteItem *projectItem = new PLMNoteItem();
            projectItem->setIsProjectItem(projectId);
            m_allNoteItems.append(projectItem);


        auto idList         = plmdata->noteHub()->getAllIds(projectId);
        auto sortOrdersHash = plmdata->noteHub()->getAllSortOrders(projectId);
        auto indentsHash    = plmdata->noteHub()->getAllIndents(projectId);

        for(int sheetId : idList) {
            m_allNoteItems.append(new PLMNoteItem(projectId, sheetId,
                                                    indentsHash.value(sheetId),
                                                    sortOrdersHash.value(sheetId)));
        }
    }
    this->endResetModel();
}

void PLMNoteListModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allNoteItems);
    this->endResetModel();
}

void PLMNoteListModel::exploitSignalFromPLMData(int                 projectId,
                                             int                 paperId,
                                             PLMNoteItem::Roles role)
{
    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        PLMNoteItem::Roles::PaperIdRole,
                                        paperId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        PLMNoteItem *t = static_cast<PLMNoteItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

//--------------------------------------------------------------------

void PLMNoteListModel::refreshAfterDataAddition(int projectId, int paperId)
{
    //find parentIndex and row
    //    QModelIndex parentIndex;
    int row = 0;

    auto idList         = plmdata->noteHub()->getAllIds(projectId);
    auto sortOrdersHash = plmdata->noteHub()->getAllSortOrders(projectId);
    auto indentsHash    = plmdata->noteHub()->getAllIndents(projectId);


    int paperIndex = idList.indexOf(paperId);
    int paperIndent = indentsHash.value(paperId);
    int paperSortOrders = sortOrdersHash.value(paperId);

    bool parentFound = false;

    if(paperIndex == 0){ // meaning the parent have to be a project item
        this->populate();
        return;
    }

    if(!parentFound){
        for (int i = paperIndex - 1; i >= 0 ; --i ) {
            int possibleParentId = idList.at(i);
            int possibleParentIndent = indentsHash.value(possibleParentId);
            if(paperIndent == possibleParentIndent + 1){
                //                auto modelIndexList = this->getModelIndex(projectId, possibleParentId);
                //                if(modelIndexList.isEmpty()){
                //                    qWarning() << Q_FUNC_INFO << "if paperIndent == possibleParentIndent => modelIndexList.isEmpty()";
                //                    return;
                //                }
                //                parentIndex = modelIndexList.first();
                //int parentPaperId = parentIndex.data(PLMNoteItem::Roles::PaperIdRole).toInt();
                row = paperIndex - i;
                parentFound = true;
                break;
            }
        }
    }
    if(!parentFound){
        qWarning() << Q_FUNC_INFO << "parent not found, failsafe used";
        this->populate();
        return;
    }


    // find item just before in m_allNoteItems to determine item index to insert in:



    int idBefore = idList.at(paperIndex - 1);
    PLMNoteItem *itemBefore = this->getItem(projectId, idBefore);
    // needed because m_allNoteItems can have multiple projects :
    int indexBefore = m_allNoteItems.indexOf(itemBefore);

    int itemIndex = indexBefore + 1;

    //        if(itemIndex >= m_allNoteItems.count() && paperIndent == itemBefore->indent() + 1){
    //            qWarning() << Q_FUNC_INFO << "last in the m_allNoteItems list and child of previous item, so failsafe used";
    //            this->populate();
    //            return;
    //        }


    for(PLMNoteItem *item : m_allNoteItems){
        item->invalidateData(PLMNoteItem::Roles::SortOrderRole);
        item->invalidateData(PLMNoteItem::Roles::HasChildrenRole);
    }



    beginInsertRows(QModelIndex(), row, row);

    m_allNoteItems.insert(itemIndex, new PLMNoteItem(projectId, paperId,
                                                       indentsHash.value(paperId),
                                                       sortOrdersHash.value(paperId)));
    this->index(row, 0, QModelIndex());
    endInsertRows();
}

//--------------------------------------------------------------------

void PLMNoteListModel::refreshAfterDataMove(int sourceProjectId, int sourcePaperId, int targetProjectId, int targetPaperId)
{
    int sourceIndex = -2;
    int targetIndex = -2;
    int sourceRow = -2;
    int targetRow = -2;



    PLMNoteItem* sourceItem = this->getItem(sourceProjectId, sourcePaperId);

    if(!sourceItem){
        qWarning() << "refreshAfterDataMove no sourceItem";
        return;
    }
    PLMNoteItem* targetItem = this->getItem(targetProjectId, targetPaperId);
    if(!targetItem){
        qWarning() << "refreshAfterDataMove no targetItem";
        return;
    }

    int i = 0;
    for(PLMNoteItem *item : m_allNoteItems){
        if(item->paperId() == sourcePaperId){
            sourceIndex = i;
        }
        if(item->paperId() == targetPaperId){
            targetIndex = i;
        }
        i++;

    }



    sourceRow = sourceIndex;
    targetRow = targetIndex;
    if(sourceRow < targetRow){
        targetRow += 1;
    }

    for(PLMNoteItem *item : m_allNoteItems){
        item->invalidateData(PLMNoteItem::Roles::SortOrderRole);
    }

    beginMoveRows(QModelIndex(), sourceRow, sourceRow, QModelIndex(), targetRow);

    PLMNoteItem *tempItem = m_allNoteItems.takeAt(sourceIndex);

    m_allNoteItems.insert(targetIndex, tempItem);

    endMoveRows();
}

//--------------------------------------------------------------------
///
/// \brief PLMNoteListModel::refreshAfterDeletedStateChanged
/// \param projectId
/// \param paperId
/// \param newDeletedState
/// careful, paper is deleted = true, not a true removal
void PLMNoteListModel::refreshAfterDeletedStateChanged(int projectId, int paperId, bool newDeletedState)
{
    Q_UNUSED(projectId)
    Q_UNUSED(paperId)
    Q_UNUSED(newDeletedState)

    for(PLMNoteItem *item : m_allNoteItems){
        item->invalidateData(PLMNoteItem::Roles::SortOrderRole);
        item->invalidateData(PLMNoteItem::Roles::HasChildrenRole); // needed to refresh the parent item when no child anymore
    }

}

//--------------------------------------------------------------------

void PLMNoteListModel::refreshAfterProjectIsBackupChanged(int projectId, bool isProjectABackup)
{
    Q_UNUSED(projectId)
    Q_UNUSED(isProjectABackup)

    for(PLMNoteItem *item : m_allNoteItems){
        item->invalidateData(PLMNoteItem::Roles::ProjectIsBackupRole);
    }

}

//--------------------------------------------------------------------
void PLMNoteListModel::connectToPLMDataSignals()
{
    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::titleChanged, this,
                                           [this](int projectId, int paperId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId, PLMNoteItem::Roles::NameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->notePropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "label") this->exploitSignalFromPLMData(projectId, paperCode,
                                                          PLMNoteItem::Roles::LabelRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::paperIdChanged, this,
                                           [this](int projectId, int paperId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::PaperIdRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::indentChanged, this,
                                           [this](int projectId, int paperId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::IndentRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList.append(this->connect(plmdata->noteHub(),
                                               &PLMNoteHub::sortOrderChanged, this,
                                               [this](int projectId, int paperId,
                                               int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::SortOrderRole);
    }));
    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::contentDateChanged, this,
                                           [this](int projectId, int paperId,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::ContentDateRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::updateDateChanged, this,
                                           [this](int projectId, int paperId,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::deletedChanged, this,
                                           [this](int projectId, int paperId,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMNoteItem::Roles::DeletedRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->notePropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "char_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMNoteItem::Roles::
                                                                 CharCountRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->notePropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "word_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMNoteItem::Roles::
                                                                 WordCountRole);
    }, Qt::UniqueConnection);
}

void PLMNoteListModel::disconnectFromPLMDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}
//-----------------------------------------------------------------------------------


QModelIndexList PLMNoteListModel::getModelIndex(int projectId, int paperId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMNoteItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap | Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : modelList) {

        int indexPaperId = modelIndex.data(PLMNoteItem::Roles::PaperIdRole).toInt();
        if (indexPaperId == paperId) {
            list.append(modelIndex);
        }
    }

    return list;
}

//-----------------------------------------------------------------------------------

PLMNoteItem *PLMNoteListModel::getParentNoteItem(PLMNoteItem *chidItem)
{
    return chidItem->parent(m_allNoteItems);
}
//-----------------------------------------------------------------------------------
PLMNoteItem *PLMNoteListModel::getItem(int projectId, int paperId)
{
    PLMNoteItem *result_item = nullptr;

    for(PLMNoteItem *item : m_allNoteItems){
        if(item->projectId() == projectId && item->paperId() == paperId){
            result_item = item;
            break;
        }
    }

    if(!result_item){
        qDebug() << "result_item is null";
    }

    return result_item;
}

//-----------------------------------------------------------------------------------


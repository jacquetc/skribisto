#include "plmsheetlistmodel.h"

PLMSheetListModel::PLMSheetListModel(QObject *parent)
    : QAbstractListModel(parent), m_headerData(QVariant())
{
    m_rootItem = new PLMSheetItem();
    m_rootItem->setIsRootItem();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMSheetListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMSheetListModel::populate);


    connect(plmdata->sheetHub(),
            &PLMSheetHub::paperAdded,
            this,
            &PLMSheetListModel::refreshAfterDataAddition);


    connect(plmdata->sheetHub(),
            &PLMSheetHub::paperMoved,
            this,
            &PLMSheetListModel::refreshAfterDataMove);

    connect(plmdata->sheetHub(),
            &PLMSheetHub::trashedChanged, // careful, paper is trashed = true,
                                          // not a true removal
            this,
            &PLMSheetListModel::refreshAfterTrashedStateChanged);

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectIsBackupChanged,
            this,
            &PLMSheetListModel::refreshAfterProjectIsBackupChanged);

    connect(plmdata->projectHub(),
            &PLMProjectHub::activeProjectChanged,
            this,
            &PLMSheetListModel::refreshAfterProjectIsActiveChanged);

    this->connectToPLMDataSignals();
}

QVariant PLMSheetListModel::headerData(int section, Qt::Orientation orientation,
                                       int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool PLMSheetListModel::setHeaderData(int             section,
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

QModelIndex PLMSheetListModel::index(int row, int column, const QModelIndex& parent) const
{
    if (column != 0) return QModelIndex();

    PLMSheetItem *parentItem;

    if (!parent.isValid()) parentItem = m_rootItem;

    PLMSheetItem *childItem = m_allSheetItems.at(row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    }
    return QModelIndex();
}

int PLMSheetListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allSheetItems.count();
}

QVariant PLMSheetListModel::data(const QModelIndex& index, int role) const
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

    PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(PLMSheetItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(PLMSheetItem::Roles::NameRole);
    }


    if (role == PLMSheetItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::PaperIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ContentDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::HasChildrenRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::TrashedRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::CharCountRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::WordCountRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::SynopsisNoteIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ProjectIsBackupRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ProjectIsActiveRole) {
        return item->data(role);
    }

    return QVariant();
}

bool PLMSheetListModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());
        int projectId      = item->projectId();
        int paperId        = item->paperId();
        PLMError error;

        this->disconnectFromPLMDataSignals();

        switch (role) {
        case PLMSheetItem::Roles::ProjectNameRole:
            error = plmdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case PLMSheetItem::Roles::ProjectIdRole:

            // useless
            break;

        case PLMSheetItem::Roles::PaperIdRole:

            // useless
            break;

        case PLMSheetItem::Roles::NameRole:

            error = plmdata->sheetHub()->setTitle(projectId, paperId, value.toString());
            break;

        case PLMSheetItem::Roles::LabelRole:
            error = plmdata->sheetPropertyHub()->setProperty(projectId, paperId,
                                                             "label", value.toString());
            break;

        case PLMSheetItem::Roles::IndentRole:
            error = plmdata->sheetHub()->setIndent(projectId, paperId, value.toInt());
            break;

        case PLMSheetItem::Roles::SortOrderRole:
            error = plmdata->sheetHub()->setSortOrder(projectId, paperId, value.toInt());

            IFOKDO(error, plmdata->sheetHub()->renumberSortOrders(projectId));

            for (PLMSheetItem *item : m_allSheetItems) {
                item->invalidateData(role);
            }
            this->populate();

            break;

        case PLMSheetItem::Roles::TrashedRole:
            error = plmdata->sheetHub()->setTrashedWithChildren(projectId, paperId, value.toBool());
            break;

        case PLMSheetItem::Roles::CreationDateRole:
            error = plmdata->sheetHub()->setCreationDate(projectId,
                                                         paperId,
                                                         value.toDateTime());
            break;

        case PLMSheetItem::Roles::UpdateDateRole:
            error = plmdata->sheetHub()->setUpdateDate(projectId,
                                                       paperId,
                                                       value.toDateTime());
            break;

        case PLMSheetItem::Roles::ContentDateRole:
            error = plmdata->sheetHub()->setContentDate(projectId,
                                                        paperId,
                                                        value.toDateTime());
            break;

        case PLMSheetItem::Roles::HasChildrenRole:
            // useless
            break;

        case PLMSheetItem::Roles::CharCountRole:
            error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "char_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;

        case PLMSheetItem::Roles::WordCountRole:

            error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "word_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;

        case PLMSheetItem::Roles::ProjectIsActiveRole:

            plmdata->projectHub()->setActiveProject(projectId);
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

Qt::ItemFlags PLMSheetListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

bool PLMSheetListModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool PLMSheetListModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

QHash<int, QByteArray>PLMSheetListModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[PLMSheetItem::Roles::ProjectNameRole]     = "projectName";
    roles[PLMSheetItem::Roles::PaperIdRole]         = "paperId";
    roles[PLMSheetItem::Roles::ProjectIdRole]       = "projectId";
    roles[PLMSheetItem::Roles::NameRole]            = "name";
    roles[PLMSheetItem::Roles::LabelRole]           = "label";
    roles[PLMSheetItem::Roles::IndentRole]          = "indent";
    roles[PLMSheetItem::Roles::SortOrderRole]       = "sortOrder";
    roles[PLMSheetItem::Roles::CreationDateRole]    = "creationDate";
    roles[PLMSheetItem::Roles::UpdateDateRole]      = "updateDate";
    roles[PLMSheetItem::Roles::ContentDateRole]     = "contentDate";
    roles[PLMSheetItem::Roles::HasChildrenRole]     = "hasChildren";
    roles[PLMSheetItem::Roles::TrashedRole]         = "trashed";
    roles[PLMSheetItem::Roles::WordCountRole]       = "wordCount";
    roles[PLMSheetItem::Roles::CharCountRole]       = "charCount";
    roles[PLMSheetItem::Roles::SynopsisNoteIdRole]  = "synopsisNoteId";
    roles[PLMSheetItem::Roles::ProjectIsBackupRole] = "projectIsBackup";
    roles[PLMSheetItem::Roles::ProjectIsActiveRole] = "projectIsActive";
    return roles;
}

void PLMSheetListModel::populate()
{
    this->beginResetModel();

    m_allSheetItems.clear();

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        PLMSheetItem *projectItem = new PLMSheetItem();
        projectItem->setIsProjectItem(projectId);
        m_allSheetItems.append(projectItem);


        auto idList         = plmdata->sheetHub()->getAllIds(projectId);
        auto sortOrdersHash = plmdata->sheetHub()->getAllSortOrders(projectId);
        auto indentsHash    = plmdata->sheetHub()->getAllIndents(projectId);

        for (int sheetId : idList) {
            m_allSheetItems.append(new PLMSheetItem(projectId, sheetId,
                                                    indentsHash.value(sheetId),
                                                    sortOrdersHash.value(sheetId)));
        }
    }
    this->endResetModel();
}

void PLMSheetListModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allSheetItems);
    this->endResetModel();
}

void PLMSheetListModel::exploitSignalFromPLMData(int                 projectId,
                                                 int                 paperId,
                                                 PLMSheetItem::Roles role)
{
    PLMSheetItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        PLMSheetItem::Roles::PaperIdRole,
                                        paperId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        PLMSheetItem *t = static_cast<PLMSheetItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

// --------------------------------------------------------------------

void PLMSheetListModel::refreshAfterDataAddition(int projectId, int paperId)
{
    // find parentIndex and row
    //    QModelIndex parentIndex;
    int row = 0;

    auto idList         = plmdata->sheetHub()->getAllIds(projectId);
    auto sortOrdersHash = plmdata->sheetHub()->getAllSortOrders(projectId);
    auto indentsHash    = plmdata->sheetHub()->getAllIndents(projectId);


    int paperIndex      = idList.indexOf(paperId);
    int paperIndent     = indentsHash.value(paperId);
    int paperSortOrders = sortOrdersHash.value(paperId);

    bool parentFound = false;

    if (paperIndex == 0) { // meaning the parent have to be a project item
        this->populate();
        return;
    }

    if (!parentFound) {
        for (int i = paperIndex - 1; i >= 0; --i) {
            int possibleParentId     = idList.at(i);
            int possibleParentIndent = indentsHash.value(possibleParentId);

            if (paperIndent == possibleParentIndent + 1) {
                //                auto modelIndexList =
                // this->getModelIndex(projectId, possibleParentId);
                //                if(modelIndexList.isEmpty()){
                //                    qWarning() << Q_FUNC_INFO << "if
                // paperIndent == possibleParentIndent =>
                // modelIndexList.isEmpty()";
                //                    return;
                //                }
                //                parentIndex = modelIndexList.first();
                // int parentPaperId =
                // parentIndex.data(PLMSheetItem::Roles::PaperIdRole).toInt();
                row         = paperIndex - i;
                parentFound = true;
                break;
            }
        }
    }

    if (!parentFound) {
        qWarning() << Q_FUNC_INFO << "parent not found, failsafe used";
        this->populate();
        return;
    }


    // find item just before in m_allSheetItems to determine item index to
    // insert in:


    int idBefore             = idList.at(paperIndex - 1);
    PLMSheetItem *itemBefore = this->getItem(projectId, idBefore);

    // needed because m_allSheetItems can have multiple projects :
    int indexBefore = m_allSheetItems.indexOf(itemBefore);

    int itemIndex = indexBefore + 1;

    //        if(itemIndex >= m_allSheetItems.count() && paperIndent ==
    // itemBefore->indent() + 1){
    //            qWarning() << Q_FUNC_INFO << "last in the m_allSheetItems list
    // and child of previous item, so failsafe used";
    //            this->populate();
    //            return;
    //        }


    for (PLMSheetItem *item : m_allSheetItems) {
        item->invalidateData(PLMSheetItem::Roles::SortOrderRole);
        item->invalidateData(PLMSheetItem::Roles::HasChildrenRole);
    }


    beginInsertRows(QModelIndex(), row, row);

    m_allSheetItems.insert(itemIndex, new PLMSheetItem(projectId, paperId,
                                                       indentsHash.value(paperId),
                                                       sortOrdersHash.value(paperId)));
    this->index(row, 0, QModelIndex());
    endInsertRows();
}

// --------------------------------------------------------------------

void PLMSheetListModel::refreshAfterDataMove(int sourceProjectId,
                                             int sourcePaperId,
                                             int targetProjectId,
                                             int targetPaperId)
{
    int sourceIndex = -2;
    int targetIndex = -2;
    int sourceRow   = -2;
    int targetRow   = -2;


    PLMSheetItem *sourceItem = this->getItem(sourceProjectId, sourcePaperId);

    if (!sourceItem) {
        qWarning() << "refreshAfterDataMove no sourceItem";
        return;
    }
    PLMSheetItem *targetItem = this->getItem(targetProjectId, targetPaperId);

    if (!targetItem) {
        qWarning() << "refreshAfterDataMove no targetItem";
        return;
    }

    int i = 0;

    for (PLMSheetItem *item : m_allSheetItems) {
        if (item->paperId() == sourcePaperId) {
            sourceIndex = i;
        }

        if (item->paperId() == targetPaperId) {
            targetIndex = i;
        }
        i++;
    }


    sourceRow = sourceIndex;
    targetRow = targetIndex;

    if (sourceRow < targetRow) {
        targetRow += 1;
    }

    for (PLMSheetItem *item : m_allSheetItems) {
        item->invalidateData(PLMSheetItem::Roles::SortOrderRole);
    }

    beginMoveRows(QModelIndex(), sourceRow, sourceRow, QModelIndex(), targetRow);

    PLMSheetItem *tempItem = m_allSheetItems.takeAt(sourceIndex);

    m_allSheetItems.insert(targetIndex, tempItem);

    endMoveRows();
}

// --------------------------------------------------------------------
///
/// \brief PLMSheetListModel::refreshAfterTrashedStateChanged
/// \param projectId
/// \param paperId
/// \param newTrashedState
/// careful, paper is trashed = true, not a true removal
void PLMSheetListModel::refreshAfterTrashedStateChanged(int  projectId,
                                                        int  paperId,
                                                        bool newTrashedState)
{
    Q_UNUSED(projectId)
    Q_UNUSED(paperId)
    Q_UNUSED(newTrashedState)

    for (PLMSheetItem *item : m_allSheetItems) {
        item->invalidateData(PLMSheetItem::Roles::TrashedRole);
        item->invalidateData(PLMSheetItem::Roles::HasChildrenRole); // needed to
                                                                    // refresh
                                                                    // the
                                                                    // parent
                                                                    // item when
                                                                    // no child
                                                                    // anymore
    }
}

// --------------------------------------------------------------------

void PLMSheetListModel::refreshAfterProjectIsBackupChanged(int  projectId,
                                                           bool isProjectABackup)
{
    Q_UNUSED(projectId)
    Q_UNUSED(isProjectABackup)

    for (PLMSheetItem *item : m_allSheetItems) {
        item->invalidateData(PLMSheetItem::Roles::ProjectIsBackupRole);
    }
}

// --------------------------------------------------------------------

void PLMSheetListModel::refreshAfterProjectIsActiveChanged(int projectId)
{
    Q_UNUSED(projectId)

    for (PLMSheetItem *item : m_allSheetItems) {
        item->invalidateData(PLMSheetItem::Roles::ProjectIsActiveRole);
    }
}

// void PLMSheetListModel::movePaper(int sourceProjectId, int sourcePaperId, int
// targetProjectId, int targetPaperId)
// {


//    QModelIndex sourceIndex = this->getModelIndex(sourceProjectId,
// sourcePaperId).first();
//    QModelIndex targetIndex = this->getModelIndex(targetProjectId,
// targetPaperId).first();


////    if (from == to)
////        return;
////    int modelFrom = from;
////    int modelTo = to + (from < to ? 1 : 0);


////beginMoveRows(QModelIndex(), sourceIndex.row(), sourceIndex.row(),
// QModelIndex(), targetIndex.row());
// beginMoveRows(QModelIndex(), 0, 0, QModelIndex(), 12);


// endMoveRows();
// }

// --------------------------------------------------------------------


void PLMSheetListModel::connectToPLMDataSignals()
{
    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::titleChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId, PLMSheetItem::Roles::NameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->projectHub(),
                                           &PLMProjectHub::projectNameChanged, this,
                                           [this](int projectId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, -1,
                                       PLMSheetItem::Roles::ProjectNameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "label") this->exploitSignalFromPLMData(projectId, paperCode,
                                                            PLMSheetItem::Roles::LabelRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::paperIdChanged, this,
                                           [this](int projectId, int paperId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::PaperIdRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::indentChanged, this,
                                           [this](int projectId, int paperId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::IndentRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList.append(this->connect(plmdata->sheetHub(),
                                               &PLMSheetHub::sortOrderChanged, this,
                                               [this](int projectId, int paperId,
                                                      int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::SortOrderRole);
    }));
    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::contentDateChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::ContentDateRole);
    }, Qt::UniqueConnection);


    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::updateDateChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::trashedChanged, this,
                                           [this](int projectId, int paperId,
                                                  bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::TrashedRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "char_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMSheetItem::Roles::
                                                                 CharCountRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "word_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMSheetItem::Roles::
                                                                 WordCountRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::sheetNoteRelationshipChanged,
                                           this,
                                           [this](int projectId, int sheetId,
                                                  int noteId) {
        Q_UNUSED(noteId)
        this->exploitSignalFromPLMData(projectId, sheetId,
                                       PLMSheetItem::Roles::SynopsisNoteIdRole);
    },
                                           Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->noteHub(),
                                           &PLMNoteHub::sheetNoteRelationshipAdded,
                                           this,
                                           [this](int projectId, int sheetId,
                                                  int noteId) {
        Q_UNUSED(noteId)
        this->exploitSignalFromPLMData(projectId, sheetId,
                                       PLMSheetItem::Roles::SynopsisNoteIdRole);
    },
                                           Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->projectHub(),
                                           &PLMProjectHub::activeProjectChanged, this,
                                           [this](int projectId) {
        Q_UNUSED(projectId)

        for (int projectId : plmdata->projectHub()->getProjectIdList()) {
            this->exploitSignalFromPLMData(projectId, -1,
                                           PLMSheetItem::Roles::ProjectIsActiveRole);
        }
    }, Qt::UniqueConnection);
}

void PLMSheetListModel::disconnectFromPLMDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}

// -----------------------------------------------------------------------------------


QModelIndexList PLMSheetListModel::getModelIndex(int projectId, int paperId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMSheetItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap |
                                             Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : modelList) {
        int indexPaperId = modelIndex.data(PLMSheetItem::Roles::PaperIdRole).toInt();

        if (indexPaperId == paperId) {
            list.append(modelIndex);
        }
    }

    return list;
}

// -----------------------------------------------------------------------------------

PLMSheetItem * PLMSheetListModel::getParentSheetItem(PLMSheetItem *chidItem)
{
    return chidItem->parent(m_allSheetItems);
}

// -----------------------------------------------------------------------------------

PLMSheetItem * PLMSheetListModel::getItem(int projectId, int paperId)
{
    PLMSheetItem *result_item = nullptr;

    for (PLMSheetItem *item : m_allSheetItems) {
        if ((item->projectId() == projectId) && (item->paperId() == paperId)) {
            result_item = item;
            break;
        }
    }

    if (!result_item) {
        qDebug() << "result_item is null";
    }

    return result_item;
}

// -----------------------------------------------------------------------------------

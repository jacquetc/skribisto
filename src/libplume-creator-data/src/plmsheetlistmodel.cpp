#include "plmsheetlistmodel.h"

PLMSheetListModel::PLMSheetListModel(QObject *parent)
    : QAbstractListModel(parent), m_headerData(QVariant())
{
    qRegisterMetaType<QList<PLMSheetListItem> >("QList<PLMSheetListItem>");

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMSheetListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMSheetListModel::populate);
}

QVariant PLMSheetListModel::headerData(int section, Qt::Orientation orientation,
                                       int role) const
{
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

int PLMSheetListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allPapers.count();
}

QVariant PLMSheetListModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    int row    = index.row();
    int column = index.column();


    int projectId = m_allPapers[row].projectId;
    int paperId   = m_allPapers[row].paperId;

    if (role == Qt::DisplayRole) {
        return plmdata->sheetHub()->getTitle(projectId, paperId);
    }

    if (role == PLMSheetListModel::NameRole) {
        return plmdata->sheetHub()->getTitle(projectId, paperId);
    }

    if (role == PLMSheetListModel::PaperIdRole) {
        return paperId;
    }

    if (role == PLMSheetListModel::ProjectIdRole) {
        return projectId;
    }

    if (role == PLMSheetListModel::TagRole) {
        return plmdata->sheetPropertyHub()->getProperty(projectId, paperId, "tag");
    }

    if (role == PLMSheetListModel::IndentRole) {
        return plmdata->sheetHub()->getIndent(projectId, paperId);
    }

    if (role == PLMSheetListModel::SortOrderRole) {
        return plmdata->sheetHub()->getSortOrder(projectId, paperId);
    }

    if (role == PLMSheetListModel::CreationDateRole) {
        return plmdata->sheetHub()->getCreationDate(projectId, paperId);
    }

    if (role == PLMSheetListModel::UpdateDateRole) {
        return plmdata->sheetHub()->getUpdateDate(projectId, paperId);
    }

    if (role == PLMSheetListModel::ContentDateRole) {
        return plmdata->sheetHub()->getContentDate(projectId, paperId);
    }
    return QVariant();
}

bool PLMSheetListModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    if (data(index, role) != value) {
        // FIXME: Implement me!
        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags PLMSheetListModel::flags(const QModelIndex& index) const
{
    if (!index.isValid()) return Qt::NoItemFlags;

    return Qt::ItemIsEditable; // FIXME: Implement me!
}

bool PLMSheetListModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
}

bool PLMSheetListModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
}

QHash<int, QByteArray>PLMSheetListModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[PaperIdRole]      = "paperId";
    roles[ProjectIdRole]    = "projectId";
    roles[NameRole]         = "name";
    roles[TagRole]          = "tag";
    roles[IndentRole]       = "indent";
    roles[SortOrderRole]    = "sortOrder";
    roles[CreationDateRole] = "creationDate";
    roles[UpdateDateRole]   = "updateDate";
    roles[ContentDateRole]  = "contentDate";
    return roles;
}

void PLMSheetListModel::populate()
{
    this->beginResetModel();

    m_allPapers.clear();
    foreach(int projectId, plmProjectManager->projectIdList()) {
        foreach(int paperId, plmdata->sheetHub()->getAllIds(projectId)) {
            m_allPapers.append(PLMSheetListItem(projectId, paperId));
        }
    }
    this->endResetModel();
}

void PLMSheetListModel::clear()
{
    this->beginResetModel();
    m_allPapers.clear();
    this->endResetModel();
}

// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------

PLMSheetListItem::PLMSheetListItem()
{
    this->projectId = -1;
    this->paperId   = -1;
}

PLMSheetListItem::PLMSheetListItem(int projectId, int paperId)
{
    this->projectId = projectId;
    this->paperId   = paperId;
}

PLMSheetListItem::~PLMSheetListItem()
{}

PLMSheetListItem::PLMSheetListItem(const PLMSheetListItem& item)
{
    this->projectId = item.projectId;
    this->paperId   = item.paperId;
}

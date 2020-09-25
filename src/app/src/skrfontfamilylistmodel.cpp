#include "skrfontfamilylistmodel.h"
#include <QFont>

SKRFontFamilyListModel::SKRFontFamilyListModel(QObject *parent)
    : QAbstractListModel(parent)
{}

QVariant SKRFontFamilyListModel::headerData(int section, Qt::Orientation orientation,
                                            int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return QVariant();
}

bool SKRFontFamilyListModel::setHeaderData(int             section,
                                           Qt::Orientation orientation,
                                           const QVariant& value,
                                           int             role)
{
    return false;
}

QModelIndex SKRFontFamilyListModel::index(int row, int column,
                                          const QModelIndex& parent) const
{
    if (column != 0) return QModelIndex();

    QModelIndex index = createIndex(row, column, nullptr);

    return index;

    return QModelIndex();
}

int SKRFontFamilyListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allFontFamilies.count();
}

QVariant SKRFontFamilyListModel::data(const QModelIndex& index, int role) const
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid
                        |  QAbstractItemModel::CheckIndexOption::DoNotUseParent));

    if (!index.isValid()) return QVariant();


    if (role == Qt::DisplayRole) {
        return m_allFontFamilies.at(index.row());
    }

    return QVariant();
}

bool SKRFontFamilyListModel::setData(const QModelIndex& index,
                                     const QVariant   & value,
                                     int                role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    return false;
}

Qt::ItemFlags SKRFontFamilyListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

bool SKRFontFamilyListModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool SKRFontFamilyListModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

QHash<int, QByteArray>SKRFontFamilyListModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[Qt::DisplayRole] = "fontFamilyName";

    return roles;
}

void SKRFontFamilyListModel::populate()
{
    this->beginResetModel();

    m_allFontFamilies.clear();


    this->endResetModel();
}

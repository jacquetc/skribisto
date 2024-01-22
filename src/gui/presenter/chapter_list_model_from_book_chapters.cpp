#include "chapter_list_model_from_book_chapters.h"
#include "book/book_controller.h"
#include "chapter/chapter_controller.h"
#include "event_dispatcher.h"
#include <QCoroTask>

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

ChapterListModelFromBookChapters::ChapterListModelFromBookChapters(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->book(), &BookSignals::allRelationsInvalidated, this, [this](int bookId) {
        if (bookId == m_bookId)
        {
            return;
        }
        auto task = Book::BookController::instance()->getWithDetails(bookId);
        QCoro::connect(std::move(task), this, [this, bookId](auto &&bookDetails) {
            if (bookDetails.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid bookId";
                return;
            }
            QList<ChapterDTO> newChapterList = bookDetails.chapters();
            QList<int> newChapterIdList;
            for (const auto &chapter : newChapterList)
            {
                newChapterIdList.append(chapter.id());
            }

            // first, add the missing chapters
            for (const auto &chapter : newChapterList)
            {
                if (!m_chapterIdList.contains(chapter.id()))
                {
                    // add the chapter
                    int row = m_chapterList.size();
                    beginInsertRows(QModelIndex(), row, row);
                    m_chapterList.append(chapter);
                    m_chapterIdList.append(chapter.id());
                    endInsertRows();
                }
            }

            // then, remove the chapterList that are not in the new list

            for (int i = m_chapterList.size() - 1; i >= 0; --i)
            {
                if (!newChapterIdList.contains(m_chapterList[i].id()))
                {
                    // remove the chapter
                    beginRemoveRows(QModelIndex(), i, i);
                    m_chapterList.removeAt(i);
                    m_chapterIdList.removeAt(i);
                    endRemoveRows();
                }
            }
            // then, move the current ones so the list is in the same order as the new list

            for (int i = 0; i < m_chapterList.size(); ++i)
            {
                if (m_chapterIdList[i] != newChapterList[i].id())
                {
                    // move the chapter
                    int row = newChapterList.indexOf(m_chapterList[i]);
                    beginMoveRows(QModelIndex(), i, i, QModelIndex(), row);
                    m_chapterList.move(i, row);
                    m_chapterIdList.move(i, row);
                    endMoveRows();
                }
            }

            // finally, update those that are in both lists if the updateDateDate has changed

            for (int i = 0; i < m_chapterList.size(); ++i)
            {
                if (m_chapterList[i].updateDate() != newChapterList[i].updateDate())
                {
                    // update the chapter
                    m_chapterList[i] = newChapterList[i];
                    QModelIndex topLeft = index(i, 0);
                    QModelIndex bottomRight = index(i, 0);
                    emit dataChanged(topLeft, bottomRight);
                }
            }

            return;
        });
    });

    connect(EventDispatcher::instance()->book(), &BookSignals::relationRemoved, this, [this](BookRelationDTO dto) {
        if (dto.relationField() != BookRelationDTO::RelationField::Chapters)
        {
            return;
        }

        // remove the chapter list
        QList<int> relatedIds = dto.relatedIds();

        for (int id : relatedIds)
        {
            if (!m_chapterIdList.contains(id))
            {
                continue;
            }

            int index = m_chapterIdList.indexOf(id);
            if (index != -1)
            {
                beginRemoveRows(QModelIndex(), index, index);
                m_chapterList.removeAt(index);
                m_chapterIdList.removeAt(index);
                endRemoveRows();
            }
        }
    });

    connect(EventDispatcher::instance()->book(), &BookSignals::relationInserted, this, [this](BookRelationDTO dto) {
        if (dto.id() != m_bookId || dto.relationField() != BookRelationDTO::RelationField::Chapters)
        {
            return;
        }

        int position = dto.position();

        // reverse dto.relatedIds()
        QList<int> relatedIds = dto.relatedIds();
        std::reverse(relatedIds.begin(), relatedIds.end());

        // fetch chapter list from controller
        for (int chapterId : relatedIds)
        {
            Chapter::ChapterController::instance()->get(chapterId).then(
                [this, chapterId, position](ChapterDTO chapter) {
                    // add chapter to this model
                    if (!m_chapterIdList.contains(chapterId))
                    {
                        beginInsertRows(QModelIndex(), position, position);
                        m_chapterList.insert(position, chapter);
                        m_chapterIdList.insert(position, chapterId);
                        endInsertRows();
                    }
                });
        }
    });

    connect(EventDispatcher::instance()->chapter(), &ChapterSignals::updated, this, [this](ChapterDTO dto) {
        for (int i = 0; i < m_chapterList.size(); ++i)
        {
            if (m_chapterIdList.at(i) == dto.id())
            {
                m_chapterList[i] = dto;
                m_chapterIdList[i] = dto.id();
                emit dataChanged(index(i), index(i));
                break;
            }
        }
    });
}

QVariant ChapterListModelFromBookChapters::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int ChapterListModelFromBookChapters::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_chapterList.count();
}

QVariant ChapterListModelFromBookChapters::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_chapterList.size())
        return QVariant();

    const ChapterDTO &chapter = m_chapterList.at(index.row());

    if (role == Qt::DisplayRole)
    {
        return chapter.title();
    }
    if (role == Qt::EditRole)
    {
        return chapter.title();
    }

    else if (role == IdRole)
        return chapter.id();
    else if (role == UuidRole)
        return chapter.uuid();
    else if (role == CreationDateRole)
        return chapter.creationDate();
    else if (role == UpdateDateRole)
        return chapter.updateDate();
    else if (role == TitleRole)
        return chapter.title();
    else if (role == LabelRole)
        return chapter.label();

    return QVariant();
}

Qt::ItemFlags ChapterListModelFromBookChapters::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable | QAbstractItemModel::flags(index);
}

bool ChapterListModelFromBookChapters::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid())
        return false;

    int row = index.row();
    if (row >= m_chapterList.size())
        return false;

    else if (role == Qt::EditRole)
    {
        return this->setData(index, value, TitleRole);
    }

    else if (role == IdRole)
    {
        if (value.canConvert<int>() == false)
        {
            qCritical() << "Cannot convert value to int";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setId(value.value<int>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == UuidRole)
    {
        if (value.canConvert<QUuid>() == false)
        {
            qCritical() << "Cannot convert value to QUuid";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setUuid(value.value<QUuid>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == CreationDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setCreationDate(value.value<QDateTime>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == UpdateDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setUpdateDate(value.value<QDateTime>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == TitleRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setTitle(value.value<QString>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == LabelRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const ChapterDTO &chapter = m_chapterList[row];

        UpdateChapterDTO dto;
        dto.setId(chapter.id());
        dto.setLabel(value.value<QString>());

        Chapter::ChapterController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid book";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }

    return false;
}

void ChapterListModelFromBookChapters::populate()
{
    if (m_bookId == 0)
        return;
    beginResetModel();
    m_chapterList.clear();
    m_chapterIdList.clear();
    endResetModel();

    auto task = Book::BookController::instance()->getWithDetails(m_bookId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Skribisto::Contracts::DTO::Chapter::ChapterDTO> chapterList = result.chapters();
        for (const auto &chapter : chapterList)
        {
            if (chapter.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return;
            }
        }
        beginInsertRows(QModelIndex(), 0, chapterList.size() - 1);
        m_chapterList = chapterList;
        // fill m_chapterIdList
        for (const auto &chapter : chapterList)
        {
            m_chapterIdList.append(chapter.id());
        }

        endInsertRows();
    });
}

int ChapterListModelFromBookChapters::bookId() const
{
    return m_bookId;
}

void ChapterListModelFromBookChapters::setBookId(int newBookId)
{
    if (m_bookId == newBookId)
        return;
    m_bookId = newBookId;

    if (m_bookId == 0)
    {
        beginResetModel();
        m_chapterList.clear();
        m_chapterIdList.clear();
        endResetModel();
    }
    else
    {
        populate();
    }
    emit bookIdChanged();
}

void ChapterListModelFromBookChapters::resetBookId()
{
    setBookId(0);
}

QHash<int, QByteArray> ChapterListModelFromBookChapters::roleNames() const
{
    QHash<int, QByteArray> names;

    // renaming id to itemId to avoid conflict with QML's id
    names[IdRole] = "itemId";
    names[UuidRole] = "uuid";
    names[CreationDateRole] = "creationDate";
    names[UpdateDateRole] = "updateDate";
    names[TitleRole] = "title";
    names[LabelRole] = "label";
    return names;
}
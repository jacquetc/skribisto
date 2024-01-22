// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter/chapter_dto.h"
#include "presenter_export.h"
#include <QAbstractListModel>

using namespace Skribisto::Contracts::DTO::Chapter;

namespace Skribisto::Presenter
{
class SKRIBISTO_PRESENTER_EXPORT ChapterListModelFromBookChapters : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int bookId READ bookId WRITE setBookId RESET resetBookId NOTIFY bookIdChanged FINAL)

  public:
    enum Roles
    {

        IdRole = Qt::UserRole + 0,
        UuidRole = Qt::UserRole + 1,
        CreationDateRole = Qt::UserRole + 2,
        UpdateDateRole = Qt::UserRole + 3,
        TitleRole = Qt::UserRole + 4,
        LabelRole = Qt::UserRole + 5
    };
    Q_ENUM(Roles)

    explicit ChapterListModelFromBookChapters(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    int bookId() const;
    void setBookId(int newBookId);
    void resetBookId();

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    QHash<int, QByteArray> roleNames() const override;

  signals:
    void bookIdChanged();

  private:
    void populate();

    QList<ChapterDTO> m_chapterList;
    QList<int> m_chapterIdList;
    int m_bookId = 0;
};

} // namespace Skribisto::Presenter
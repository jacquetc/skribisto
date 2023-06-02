#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::Chapter
{

class ChapterDTOBase : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString title READ getTitle WRITE setTitle NOTIFY titleChanged)
    Q_PROPERTY(QDateTime creationDate READ getCreationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ getUpdateDate WRITE setUpdateDate)

  public:
    ChapterDTOBase(QObject *parent = nullptr) : QObject(parent)
    {
    }
    ChapterDTOBase(const QString &title) : QObject(), m_title(title)
    {
    }
    ChapterDTOBase(const ChapterDTOBase &other)
        : QObject(), m_title(other.m_title), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate)
    {
    }

    ChapterDTOBase &operator=(const ChapterDTOBase &other)
    {

        m_title = other.m_title;
        m_creationDate = other.m_creationDate;
        m_updateDate = other.m_updateDate;
        return *this;
    }
    bool operator==(const ChapterDTOBase &other) const
    {
        return m_title == other.m_title && m_creationDate == other.m_creationDate && m_updateDate == other.m_updateDate;
    }
    QDateTime getCreationDate() const;
    QDateTime creationDate() const;
    void setCreationDate(const QDateTime &newCreationDate);
    QDateTime getUpdateDate() const;
    QDateTime updateDate() const;
    void setUpdateDate(const QDateTime &newUpdateDate);
    QString getTitle() const;
    QString title() const;
    void setTitle(const QString &newTitle);

  signals:
    void titleChanged(const QString &newTitle);

  private:
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_title;
};

inline QDateTime ChapterDTOBase::getCreationDate() const
{
    return m_creationDate;
}

inline QDateTime ChapterDTOBase::creationDate() const
{
    return m_creationDate;
}

inline void ChapterDTOBase::setCreationDate(const QDateTime &newCreationDate)
{
    m_creationDate = newCreationDate;
}

inline QDateTime ChapterDTOBase::getUpdateDate() const
{
    return m_updateDate;
}

inline QDateTime ChapterDTOBase::updateDate() const
{
    return m_updateDate;
}

inline void ChapterDTOBase::setUpdateDate(const QDateTime &newUpdateDate)
{
    m_updateDate = newUpdateDate;
}

inline QString ChapterDTOBase::getTitle() const
{
    return m_title;
}

inline QString ChapterDTOBase::title() const
{
    return m_title;
}

inline void ChapterDTOBase::setTitle(const QString &newTitle)
{
    if (m_title != newTitle)
    {
        m_title = newTitle;
        emit titleChanged(newTitle);
    }
}

} // namespace Contracts::DTO::Chapter

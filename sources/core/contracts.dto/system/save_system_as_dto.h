#pragma once

#include <QObject>
#include <QUrl>

namespace Contracts::DTO::System
{
class SaveSystemAsDTO : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QUrl fileName MEMBER m_fileName)
  public:
    SaveSystemAsDTO(QObject *parent = nullptr)
    {
    }

    SaveSystemAsDTO(const SaveSystemAsDTO &other) : m_fileName(other.m_fileName)
    {
    }

    SaveSystemAsDTO &operator=(const SaveSystemAsDTO &other)
    {
        if (this != &other)
        {
            m_fileName = other.m_fileName;
        }
        return *this;
    }

    QUrl fileName() const
    {
        return m_fileName;
    }

    void setFileName(const QUrl &fileName)
    {
        m_fileName = fileName;
    }

  private:
    QUrl m_fileName;
};
} // namespace Contracts::DTO::System
